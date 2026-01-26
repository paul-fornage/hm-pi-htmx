use serde::{Deserialize, Serialize};
use super::raw_motion_profile::RawMotionProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionProfile {
    pub name: String,
    pub description: String,
    pub raw_profile: RawMotionProfile,
}

impl MotionProfile {
    pub fn new(name: String, description: String, raw_profile: RawMotionProfile) -> Self {
        Self {
            name,
            description,
            raw_profile,
        }
    }

    pub fn new_empty(name: String, description: String) -> Self {
        Self {
            name,
            description,
            raw_profile: RawMotionProfile {
                weld_enable: false,
                uses_y_axis: false,
                uses_z_axis: false,
                uses_w_axis: false,
                cycle_start_pos: 0,
                cycle_end_pos: 0,
                cycle_park_pos: 0,
                cycle_weld_speed: 0,
                cycle_reposition_speed: 0,
                cycle_wire_feed_speed: 0,
                axis_z_homing_offset: 0,
                axis_z_homing_speed: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileListEntry {
    pub name: String,
    pub description: String,
}

impl From<&MotionProfile> for ProfileListEntry {
    fn from(profile: &MotionProfile) -> Self {
        Self {
            name: profile.name.clone(),
            description: profile.description.clone(),
        }
    }
}
