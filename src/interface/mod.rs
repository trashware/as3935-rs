use std::fmt::{Display, Formatter};
use std::time::Duration;

pub(crate) mod conversion;
pub mod i2c;

pub(crate) const CLOCK_GENERATION_DELAY: Duration = Duration::from_millis(2);
pub(crate) const IRQ_TRIGGER_TO_READY_DELAY: Duration = Duration::from_millis(2);
pub(crate) const LIGHTNING_CALCULATION_DELAY: Duration = Duration::from_millis(2);
pub const DISTURBER_DEACTIVATION_PERIOD: Duration = Duration::from_millis(1500);
pub const APPROXIMATE_MINIMUM_LIGHTNING_INTERVAL: Duration = Duration::from_secs(1);

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Spi(::rppal::spi::Error),
    I2c(::rppal::i2c::Error),
}

impl ::std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            Error::Spi(e) => e.fmt(f),
            Error::I2c(e) => e.fmt(f),
        }
    }
}

impl From<::rppal::i2c::Error> for Error {
    fn from(error: ::rppal::i2c::Error) -> Self {
        Error::I2c(error)
    }
}

impl From<::rppal::spi::Error> for Error {
    fn from(error: ::rppal::spi::Error) -> Self {
        Error::Spi(error)
    }
}

pub(crate) enum Irq {
    DistanceEstimationChanged,
    /// INT_NH
    NoiseLevelTooHigh,
    /// INT_D
    DisturberDetected,
    /// INT_L
    Lightning,
}

pub(crate) trait Interface: Send {
    fn read(&mut self, register: Box<dyn crate::device::registers::Register>) -> Result<u8>;
    fn write(
        &mut self,
        register: Box<dyn crate::device::registers::Register>,
        payload: u8,
    ) -> Result<()>;
}

pub(crate) fn calculate_bitshift(mask: u8) -> u8 {
    for i in 0..7 {
        if (mask & (1 << i)) == 1 {
            return i;
        }
    }

    0
}
