use crate::interface::i2c::I2cAddress;
use crate::interface::Irq;
use crate::{
    HeadOfStormDistance, IgnoreDisturbances, MinimumLightningThreshold, NoiseFloorThreshold,
    SensorPlacing, SignalVerificationThreshold,
};

impl From<u8> for Irq {
    fn from(irq: u8) -> Self {
        match irq {
            0b_0000 => Irq::DistanceEstimationChanged,
            0b_0001 => Irq::NoiseLevelTooHigh,
            0b_0100 => Irq::DisturberDetected,
            0b_1000 => Irq::Lightning,
            _ => panic!(),
        }
    }
}

impl Into<u8> for MinimumLightningThreshold {
    fn into(self) -> u8 {
        match self {
            MinimumLightningThreshold::One => 0b_00_u8,
            MinimumLightningThreshold::Five => 0b_01_u8,
            MinimumLightningThreshold::Nine => 0b_10_u8,
            MinimumLightningThreshold::Sixteen => 0b_11_u8,
        }
    }
}

impl From<u8> for HeadOfStormDistance {
    fn from(raw_distance: u8) -> Self {
        match raw_distance {
            0b_11_1111 => HeadOfStormDistance::OutOfRange,
            0b_10_1000 => HeadOfStormDistance::Kilometers(40),
            0b_10_0101 => HeadOfStormDistance::Kilometers(37),
            0b_10_0010 => HeadOfStormDistance::Kilometers(34),
            0b_01_1111 => HeadOfStormDistance::Kilometers(31),
            0b_01_1011 => HeadOfStormDistance::Kilometers(27),
            0b_01_1000 => HeadOfStormDistance::Kilometers(24),
            0b_01_0100 => HeadOfStormDistance::Kilometers(20),
            0b_01_0001 => HeadOfStormDistance::Kilometers(17),
            0b_00_1110 => HeadOfStormDistance::Kilometers(14),
            0b_00_1100 => HeadOfStormDistance::Kilometers(12),
            0b_00_1010 => HeadOfStormDistance::Kilometers(10),
            0b_00_1000 => HeadOfStormDistance::Kilometers(8),
            0b_00_0110 => HeadOfStormDistance::Kilometers(6),
            0b_00_0101 => HeadOfStormDistance::Kilometers(5),
            0b_00_0001 => HeadOfStormDistance::Overhead,
            _ => panic!(),
        }
    }
}

impl Into<u8> for SensorPlacing {
    fn into(self) -> u8 {
        match self {
            SensorPlacing::Indoor => 0b_1_0010_u8,
            SensorPlacing::Outdoor => 0b_0_1110_u8,
        }
    }
}

impl Into<u8> for SignalVerificationThreshold {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<u8> for NoiseFloorThreshold {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<u8> for IgnoreDisturbances {
    fn into(self) -> u8 {
        match self {
            IgnoreDisturbances::Yes => 0b_1,
            IgnoreDisturbances::No => 0b_0,
        }
    }
}

impl Into<u16> for I2cAddress {
    fn into(self) -> u16 {
        return self.0 as u16;
    }
}
