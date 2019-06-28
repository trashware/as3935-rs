use crate::device::registers::{Mode, Register};

pub(crate) struct PresetDefault;
impl Register for PresetDefault {
    fn name(&self) -> &'static str {
        &"PRESET_DEFAULT"
    }

    fn description(&self) -> &'static str {
        &"Sets all registers in default mode"
    }

    fn address(&self) -> u8 {
        0x3C
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
