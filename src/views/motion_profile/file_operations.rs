use std::path::{Path, PathBuf};
use log::{info, warn};
use crate::file_io::{deserialize_json, serialize_json, FileIoError, NamedDiskFile};
use crate::paths::subdirs::Subdir;
use super::motion_profile::{MotionProfile, ProfileListEntry};

pub fn profile_path() -> PathBuf {
    Subdir::MotionProfiles.full_local_path()
}
const PROFILE_EXTENSION: &str = "json";

fn name_to_path(name: &str) -> PathBuf {
    profile_path().join(format!("{}.{}", name, PROFILE_EXTENSION))
}

impl NamedDiskFile for MotionProfile {
    const SUBDIR: Subdir = Subdir::MotionProfiles;
    const EXT: &'static str = PROFILE_EXTENSION;

    fn serialize_value(value: &Self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(value, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        deserialize_json(contents, path)
    }
}

pub async fn save_profile(profile: &MotionProfile) -> Result<(), FileIoError> {
    MotionProfile::save(&profile.name, profile).await?;
    info!("Saved profile '{}' to {:?}", profile.name, name_to_path(&profile.name));
    Ok(())
}

pub async fn load_profile(name: &str) -> Result<MotionProfile, FileIoError> {
    let profile = MotionProfile::load(name).await?;

    if profile.name != name {
        warn!(
            "Profile file name mismatch: expected '{}', got '{}' in file",
            name, profile.name
        );
    }

    info!("Loaded profile '{}' from {:?}", name, name_to_path(name));
    Ok(profile)
}

pub async fn delete_profile(name: &str) -> Result<(), FileIoError> {
    MotionProfile::delete(name).await?;
    info!("Deleted profile '{}' from {:?}", name, name_to_path(name));
    Ok(())
}

pub async fn list_profiles() -> Result<Vec<ProfileListEntry>, FileIoError> {
    let names = MotionProfile::list().await?;
    let mut profiles = Vec::new();

    for name in names {
        match MotionProfile::load(&name).await {
            Ok(profile) => profiles.push(ProfileListEntry::from(&profile)),
            Err(e) => {
                warn!("Failed to load profile '{}': {}", name, e);
                continue;
            }
        }
    }

    profiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(profiles)
}
