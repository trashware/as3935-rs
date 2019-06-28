use crate::device::registers::{Mode, Register};

pub(crate) struct DisplayLcoOnIrqPin;
impl Register for DisplayLcoOnIrqPin {
    fn name(&self) -> &'static str {
        &"DISP_LCO"
    }

    fn description(&self) -> &'static str {
        &"Display LCO on IRQ pin"
    }

    fn address(&self) -> u8 {
        0x08
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_1000_0000
    }

    fn default_value(&self) -> u8 {
        0b_0
    }
}
