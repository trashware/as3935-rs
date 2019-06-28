use crate::device::registers::{Mode, Register};

pub(crate) struct CalibrateOscillators;
impl Register for CalibrateOscillators {
    fn name(&self) -> &'static str {
        &"CALIB_RCO"
    }

    fn description(&self) -> &'static str {
        &"Calibrates automatically the internal RC Oscillators"
    }

    fn address(&self) -> u8 {
        0x3D
    }

    fn mode(&self) -> Mode {
        Mode::Write
    }

    fn mask(&self) -> u8 {
        0b_1111_1111
    }

    fn default_value(&self) -> u8 {
        0b_0000_0000
    }
}
