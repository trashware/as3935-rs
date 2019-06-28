use crate::device::registers::{Mode, Register};

pub(crate) struct DistanceEstimation;
impl Register for DistanceEstimation {
    fn name(&self) -> &'static str {
        &"DISTANCE"
    }

    fn description(&self) -> &'static str {
        &"Distance estimation"
    }

    fn address(&self) -> u8 {
        0x07
    }

    fn mode(&self) -> Mode {
        Mode::Read
    }

    fn mask(&self) -> u8 {
        0b_0011_1111
    }

    fn default_value(&self) -> u8 {
        0b_00_0000
    }
}
