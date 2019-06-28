use crate::device::registers::{Mode, Register};

pub(crate) struct FrequencyDivisionRationForAntennaTuning;
impl Register for FrequencyDivisionRationForAntennaTuning {
    fn name(&self) -> &'static str {
        &"LCO_FDIV"
    }

    fn description(&self) -> &'static str {
        &"Frequency division ration for antenna tuning"
    }

    fn address(&self) -> u8 {
        0x03
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_1100_0000
    }

    fn default_value(&self) -> u8 {
        0b_00
    }
}
