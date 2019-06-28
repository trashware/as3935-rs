use crate::device::registers::{Mode, Register};

pub(crate) struct Interrupt;
impl Register for Interrupt {
    fn name(&self) -> &'static str {
        &"INT"
    }

    fn description(&self) -> &'static str {
        &"Interrupt"
    }

    fn address(&self) -> u8 {
        0x03
    }

    fn mode(&self) -> Mode {
        Mode::Read
    }

    fn mask(&self) -> u8 {
        0b_0000_1111
    }

    fn default_value(&self) -> u8 {
        0b_0000
    }
}
