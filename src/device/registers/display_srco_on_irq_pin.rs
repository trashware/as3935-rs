use crate::device::registers::{Mode, Register};

pub(crate) struct DisplaySrcoOnIrqPin;
impl Register for DisplaySrcoOnIrqPin {
    fn name(&self) -> &'static str {
        &"DISP_SRCO"
    }

    fn description(&self) -> &'static str {
        &"Display SRCO on IRQ pin"
    }

    fn address(&self) -> u8 {
        0x08
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_0100_0000
    }

    fn default_value(&self) -> u8 {
        0b_0
    }
}
