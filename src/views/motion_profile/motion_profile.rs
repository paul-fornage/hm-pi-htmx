use super::raw_motion_profile::RawMotionProfile;
use serde::{Deserialize, Serialize};

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
            raw_profile: RawMotionProfile::default(),
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
