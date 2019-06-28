#![allow(unused)]

mod afe_gain_boost;
mod calibrate_oscillators;
mod clear_statistics;
mod display_lco_on_irq_pin;
mod display_srco_on_irq_pin;
mod display_trco_on_irq_pin;
mod distance_estimation;
mod frequency_division_ration_for_antenna_tuning;
mod internal_tuning_capacitors;
mod interrupt;
mod mask_disturber;
mod minimum_number_of_lightning;
mod noise_floor_level;
mod power_down;
mod preset_default;
mod spike_rejection;
mod watchdog_threshold;

pub(crate) use afe_gain_boost::AfeGainBoost;
pub(crate) use calibrate_oscillators::CalibrateOscillators;
pub(crate) use clear_statistics::ClearStatistics;
pub(crate) use display_lco_on_irq_pin::DisplayLcoOnIrqPin;
pub(crate) use display_srco_on_irq_pin::DisplaySrcoOnIrqPin;
pub(crate) use display_trco_on_irq_pin::DisplayTrcoOnIrqPin;
pub(crate) use distance_estimation::DistanceEstimation;
pub(crate) use frequency_division_ration_for_antenna_tuning::FrequencyDivisionRationForAntennaTuning;
pub(crate) use internal_tuning_capacitors::InternalTuningCapacitors;
pub(crate) use interrupt::Interrupt;
pub(crate) use mask_disturber::MaskDisturber;
pub(crate) use minimum_number_of_lightning::MinimumNumberOfLightning;
pub(crate) use noise_floor_level::NoiseFloorLevel;
pub(crate) use power_down::PowerDown;
pub(crate) use preset_default::PresetDefault;
pub(crate) use spike_rejection::SpikeRejection;
pub(crate) use watchdog_threshold::WatchdogThreshold;

pub(crate) trait Register {
    /// Register's name as defined in the Detailed Register Map (Table 9).
    fn name(&self) -> &'static str;

    /// Register's description as defined in the Detailed Register Map (Table 9).
    fn description(&self) -> &'static str;

    /// Register's address as defined in the Detailed Register Map (Table 9).
    fn address(&self) -> u8;

    /// Register's supported access type as defined in the Detailed Register Map (Table 9).
    fn mode(&self) -> Mode;

    /// Register's bitmask as defined in the Detailed Register Map (Table 9).
    fn mask(&self) -> u8;

    /// Register's default value as defined in the Detailed Register Map (Table 9).
    fn default_value(&self) -> u8;
}

/// Supported access types.
#[allow(unused)]
pub(crate) enum Mode {
    Read,
    Write,
    ReadWrite,
}
