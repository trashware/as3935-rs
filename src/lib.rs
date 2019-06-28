#[macro_use]
extern crate log;

use crate::device::registers::{
    AfeGainBoost, CalibrateOscillators, DisplayTrcoOnIrqPin, DistanceEstimation, Interrupt,
    MaskDisturber, MinimumNumberOfLightning, NoiseFloorLevel, PowerDown, PresetDefault,
    WatchdogThreshold,
};
use crate::interface::i2c::{I2cAddress, I2cInterface};
use crate::interface::{
    Interface, Irq, CLOCK_GENERATION_DELAY, IRQ_TRIGGER_TO_READY_DELAY, LIGHTNING_CALCULATION_DELAY,
};
use rppal::gpio::{InputPin, Level, Trigger};
use rppal::i2c::I2c;
use rppal::spi::Spi;
use std::error;
use std::fmt;
use std::result::Result::Err;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

pub(crate) mod device;
pub mod interface;

pub type IrqPin = InputPin;

#[derive(Debug)]
pub enum Error {
    Deadlock,
    InterfaceError(interface::Error),
    InvalidState,
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        unimplemented!()
    }
}

impl From<crate::interface::Error> for Error {
    fn from(error: crate::interface::Error) -> Self {
        Error::InterfaceError(error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SensorPlacing {
    Indoor,
    Outdoor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MinimumLightningThreshold {
    One,
    Five,
    Nine,
    Sixteen,
}

/// Larger values correspond to more robust disturber rejection, with a decrease of the detection efficiency,
/// Refer to Figure 20 in the datasheet for the relationship between this threshold and its impact.
/// Defaults to 2.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignalVerificationThreshold(pub(crate) u8);

impl SignalVerificationThreshold {
    pub fn new(value: u8) -> ::std::result::Result<Self, &'static str> {
        if value > 10 {
            return Err("Signal verification threshold must be in range 0-10");
        }

        Ok(Self(value))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NoiseFloorThreshold(pub(crate) u8);

impl NoiseFloorThreshold {
    pub fn new(value: u8) -> ::std::result::Result<Self, &'static str> {
        if value > 11 {
            return Err("Noise level threshold must be in range 0-11");
        }

        Ok(Self(value))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IgnoreDisturbances {
    Yes,
    No,
}

/// Estimated distance to the head of storm, in kilometers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeadOfStormDistance {
    /// the storm is within 5-40 km range
    Kilometers(u8),
    /// the storm is out of range (>40 km)
    OutOfRange,
    /// the storm is overhead (<5 km)
    Overhead,
}

pub enum InterfaceSelection {
    I2c(I2c, I2cAddress),
    Spi(Spi, u8),
}

pub enum Event {
    Disturbance,
    Lightning(HeadOfStormDistance),
    Noise,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Listening,
    PoweredDown,
    StandingBy,
}

#[derive(Default)]
pub struct ListeningParameters {
    pub(crate) sensor_placing: Option<SensorPlacing>,
    pub(crate) minimum_lightning_threshold: Option<MinimumLightningThreshold>,
    pub(crate) noise_floor_threshold: Option<NoiseFloorThreshold>,
    pub(crate) signal_verification_threshold: Option<SignalVerificationThreshold>,
    pub(crate) ignore_disturbances: Option<IgnoreDisturbances>,
}

impl ListeningParameters {
    pub fn with_sensor_placing(mut self, sensor_placing: SensorPlacing) -> Self {
        self.sensor_placing = Some(sensor_placing);
        self
    }

    pub fn with_minimum_lightning_threshold(
        mut self,
        minimum_lightning_threshold: MinimumLightningThreshold,
    ) -> Self {
        self.minimum_lightning_threshold = Some(minimum_lightning_threshold);
        self
    }

    pub fn with_noise_floor_threshold(
        mut self,
        noise_floor_threshold: NoiseFloorThreshold,
    ) -> Self {
        self.noise_floor_threshold = Some(noise_floor_threshold);
        self
    }

    pub fn with_signal_verification_threshold(
        mut self,
        signal_verification_threshold: SignalVerificationThreshold,
    ) -> Self {
        self.signal_verification_threshold = Some(signal_verification_threshold);
        self
    }

    pub fn with_ignore_disturbances(mut self, ignore_disturbances: IgnoreDisturbances) -> Self {
        self.ignore_disturbances = Some(ignore_disturbances);
        self
    }
}

pub struct AS3935 {
    interface: Arc<Mutex<Box<dyn Interface>>>,
    irq_pin: IrqPin,
    state: State,
}

impl AS3935 {
    pub fn new(interface_selection: InterfaceSelection, irq_pin: IrqPin) -> Result<Self> {
        Ok(match interface_selection {
            InterfaceSelection::I2c(i2c, i2c_address) => Self {
                interface: Arc::new(Mutex::new(Box::new(I2cInterface::new(i2c, i2c_address)?))),
                irq_pin,
                state: State::StandingBy,
            },
            InterfaceSelection::Spi(_, _) => unimplemented!(),
        })
    }

    pub fn listen(&mut self, parameters: ListeningParameters) -> Result<Receiver<Event>> {
        self.assert_state(&self.state, &[State::StandingBy, State::PoweredDown])?;

        info!("starting listen sequence");

        debug!("powering up");
        self.power_up()?;

        debug!("calibrating clock");
        self.calibrate_clock()?;

        debug!("resetting to defaults");
        self.configure_defaults()?;

        debug!("configuring listen parameters");
        self.configure_listen_parameters(parameters)?;

        let (sender, receiver) = channel::<Event>();
        self.setup_irq(sender)?;

        self.state = State::Listening;

        Ok(receiver)
    }

    pub fn terminate(&mut self) -> Result<()> {
        self.assert_state(&self.state, &[State::Listening])?;

        self.irq_pin.clear_async_interrupt().unwrap();
        self.power_down()?;

        self.state = State::PoweredDown;

        Ok(())
    }

    pub fn is_listening(&self) -> bool {
        self.state == State::Listening
    }

    fn power_up(&mut self) -> Result<()> {
        self.assert_state(&self.state, &[State::StandingBy, State::PoweredDown])?;

        self.interface
            .lock()
            .unwrap()
            .write(Box::new(PowerDown), 0b_0)?;
        sleep(Duration::from_millis(2));

        Ok(())
    }

    fn power_down(&mut self) -> Result<()> {
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(PowerDown), 0b_1)?;

        Ok(())
    }

    fn calibrate_clock(&mut self) -> Result<()> {
        self.assert_state(&self.state, &[State::StandingBy, State::PoweredDown])?;

        debug!("sending CALIB_RCO direct command");
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(CalibrateOscillators), 0x96)?;
        sleep(Duration::from_millis(2));

        debug!("setting DISP_TRCO=1");
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(DisplayTrcoOnIrqPin), 0b_1)?;

        sleep(CLOCK_GENERATION_DELAY);

        debug!("setting DISP_TRCO=0");
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(DisplayTrcoOnIrqPin), 0)?;
        sleep(Duration::from_millis(2));

        Ok(())
    }

    fn configure_defaults(&mut self) -> Result<()> {
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(PresetDefault), 0x96)?;

        Ok(())
    }

    fn configure_listen_parameters(&mut self, parameters: ListeningParameters) -> Result<()> {
        if let Some(sensor_placing) = parameters.sensor_placing {
            debug!("configuring sensor placing");
            self.configure_sensor_placing(&sensor_placing)?;
        }

        if let Some(minimum_lightning_threshold) = &parameters.minimum_lightning_threshold {
            debug!("configuring minimum lightning threshold");
            self.configure_minimum_lightning_threshold(&minimum_lightning_threshold)?;
        }

        if let Some(noise_floor_threshold) = &parameters.noise_floor_threshold {
            debug!("configuring noise floor threshold");
            self.configure_noise_floor_threshold(&noise_floor_threshold)?;
        }

        if let Some(signal_verification_threshold) = &parameters.signal_verification_threshold {
            debug!("configuring signal verification threshold");
            self.configure_signal_verification_threshold(&signal_verification_threshold)?;
        }

        if let Some(ignore_disturbances) = &parameters.ignore_disturbances {
            debug!("configuring ignoring of disturbances");
            self.configure_ignore_disturbances(&ignore_disturbances)?;
        }

        Ok(())
    }

    fn configure_sensor_placing(&mut self, placing: &SensorPlacing) -> Result<()> {
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(AfeGainBoost), (*placing).into())?;

        Ok(())
    }

    fn configure_minimum_lightning_threshold(
        &mut self,
        minimum_lightning_threshold: &MinimumLightningThreshold,
    ) -> Result<()> {
        self.interface.lock().unwrap().write(
            Box::new(MinimumNumberOfLightning),
            (*minimum_lightning_threshold).into(),
        )?;

        Ok(())
    }

    fn configure_noise_floor_threshold(
        &mut self,
        noise_floor_threshold: &NoiseFloorThreshold,
    ) -> Result<()> {
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(NoiseFloorLevel), (*noise_floor_threshold).into())?;

        Ok(())
    }

    fn configure_signal_verification_threshold(
        &mut self,
        signal_verification_threshold: &SignalVerificationThreshold,
    ) -> Result<()> {
        self.interface.lock().unwrap().write(
            Box::new(WatchdogThreshold),
            (*signal_verification_threshold).into(),
        )?;

        Ok(())
    }

    fn configure_ignore_disturbances(
        &mut self,
        ignore_disturbances: &IgnoreDisturbances,
    ) -> Result<()> {
        self.interface
            .lock()
            .unwrap()
            .write(Box::new(MaskDisturber), (*ignore_disturbances).into())?;

        Ok(())
    }

    fn setup_irq(&mut self, sender: Sender<Event>) -> Result<()> {
        let interface_mutex = self.interface.clone();

        self.irq_pin
            .set_async_interrupt(Trigger::RisingEdge, move |_level: Level| {
                sleep(IRQ_TRIGGER_TO_READY_DELAY);

                let mut interface = interface_mutex.lock().unwrap();

                let irq = Irq::from(interface.read(Box::new(Interrupt)).unwrap());

                let event = match irq {
                    Irq::DistanceEstimationChanged => return,
                    Irq::DisturberDetected => Event::Disturbance,
                    Irq::Lightning => {
                        sleep(LIGHTNING_CALCULATION_DELAY);
                        Event::Lightning(HeadOfStormDistance::from(
                            interface.read(Box::new(DistanceEstimation)).unwrap(),
                        ))
                    }
                    Irq::NoiseLevelTooHigh => Event::Noise,
                };

                sender.send(event).unwrap();
            })
            .unwrap();

        Ok(())
    }

    fn assert_state(&self, state: &State, valid_states: &[State]) -> Result<()> {
        if !valid_states.contains(state) {
            return Err(Error::InvalidState);
        }

        Ok(())
    }
}
