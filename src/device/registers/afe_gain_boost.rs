use crate::device::registers::{Mode, Register};

pub struct AfeGainBoost;
impl Register for AfeGainBoost {
    fn name(&self) -> &'static str {
        &"AFE_GB"
    }

    fn description(&self) -> &'static str {
        &"AFE Gain Boost"
    }

    fn address(&self) -> u8 {
        0x00
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_0011_1110
    }

    fn default_value(&self) -> u8 {
        0b_1_0010
    }
}
