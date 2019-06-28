use crate::device::registers::{Mode, Register};

pub(crate) struct InternalTuningCapacitors;
impl Register for InternalTuningCapacitors {
    fn name(&self) -> &'static str {
        &"TUN_CAP"
    }

    fn description(&self) -> &'static str {
        &"Internal Tuning Capacitors (from 0 to 120pF in steps of 8pf)"
    }

    fn address(&self) -> u8 {
        0x08
    }

    fn mode(&self) -> Mode {
        Mode::ReadWrite
    }

    fn mask(&self) -> u8 {
        0b_0000_1111
    }

    fn default_value(&self) -> u8 {
        0b_0000
    }
}
