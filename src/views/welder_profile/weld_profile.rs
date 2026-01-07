use serde::{Deserialize, Serialize};
use super::raw_weld_profile::RawWeldProfile;

/// Complete welding profile including metadata and all register values.
/// This is the top-level structure that gets serialized to/from disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeldProfile {
    /// Profile name (required - used as filename)
    pub name: String,

    /// Profile description (empty string if none)
    pub description: String,

    /// All register values
    pub raw_profile: RawWeldProfile,
}

impl WeldProfile {
    /// Creates a new profile with the given name, description, and raw register values.
    pub fn new(name: String, description: String, raw_profile: RawWeldProfile) -> Self {
        Self {
            name,
            description,
            raw_profile,
        }
    }

    /// Creates a profile name and description, with empty register values.
    /// Useful for testing or creating template profiles.
    pub fn new_empty(name: String, description: String) -> Self {
        Self {
            name,
            description,
            raw_profile: RawWeldProfile {
                use_dc_output: false,
                use_ep_polarity: false,
                boost_en: false,
                droop_en: false,
                use_low_ocv: false,
                pulser_en: false,
                use_low_ac_commutation_amp: false,
                ac_independant_en: false,
                tungsten_preset: 0,
                arc_start_polarity_phase: 0,
                ac_en_wave_shape: 0,
                ac_ep_wave_shape: 0,
                preset_min_amperage: 0,
                arc_start_amperage: 0,
                arc_start_time: 0,
                arc_start_slope_time: 0,
                arc_start_ac_time: 0,
                hot_start_time: 0,
                ac_en_amperage: 0,
                ac_ep_amperage: 0,
                ac_balance: 0,
                ac_frequency: 0,
                weld_amperage: 0,
                pulser_pps: 0,
                pulser_peak_time: 0,
                preflow_time: 0,
                initial_amperage: 0,
                initial_time: 0,
                initial_slope_time: 0,
                main_time: 0,
                final_slope_time: 0,
                final_amperage: 0,
                final_time: 0,
                hot_wire_voltage: 0,
                postflow_time: 0,
            },
        }
    }
}

/// Lightweight metadata for listing profiles without loading full register data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileListEntry {
    pub name: String,
    pub description: String,
}

impl From<&WeldProfile> for ProfileListEntry {
    fn from(profile: &WeldProfile) -> Self {
        Self {
            name: profile.name.clone(),
            description: profile.description.clone(),
        }
    }
}
