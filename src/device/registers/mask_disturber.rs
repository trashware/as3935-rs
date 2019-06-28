use crate::device::registers::{Mode, Register};

pub(crate) struct MaskDisturber;
impl Register for MaskDisturber {
    fn name(&self) -> &'static str {
        &"MASK_DIST"
    }

    fn description(&self) -> &'static str {
        &"Mask Disturber"
    }

    fn address(&self) -> u8 {
        0x03
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_0010_0000
    }

    fn default_value(&self) -> u8 {
        0b_0
    }
}
