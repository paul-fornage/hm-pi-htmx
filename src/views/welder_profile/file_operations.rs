use std::path::{Path, PathBuf};
use log::{info, warn};
use crate::file_io::{deserialize_json, serialize_json, FileIoError, NamedDiskFile, validate_filename};
use crate::paths::subdirs::Subdir;
use super::weld_profile::{WeldProfile, ProfileListEntry};

const PROFILE_EXTENSION: &str = "json";
const PROFILES_DIR: Subdir = Subdir::WeldProfiles;

fn profiles_path() -> PathBuf {
    PROFILES_DIR.full_local_path()
}

/// Converts a profile name to a filesystem path.
fn name_to_path(name: &str) -> PathBuf {
    profiles_path().join(format!("{}.{}", name, PROFILE_EXTENSION))
}

impl NamedDiskFile for WeldProfile {
    const SUBDIR: Subdir = Subdir::WeldProfiles;
    const EXT: &'static str = PROFILE_EXTENSION;

    fn serialize_value(value: &Self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(value, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        deserialize_json(contents, path)
    }
}

/// Saves a welding profile to disk.
///
/// This function performs critical file I/O operations. Failures could result in:
/// - Lost welding parameters
/// - Inconsistent machine state
/// - Corrupted profile data
///
/// The function validates the filename, ensures directory existence, serializes
/// the profile, and writes atomically to prevent partial writes.
pub async fn save_profile(profile: &WeldProfile) -> Result<(), FileIoError> {
    WeldProfile::save(&profile.name, profile).await?;
    info!("Saved profile '{}' to {:?}", profile.name, name_to_path(&profile.name));
    Ok(())
}

/// Loads a welding profile from disk.
///
/// This function reads critical machine parameters. A corrupted or malicious
/// profile file could cause the machine to operate with dangerous parameters.
/// All data is validated during deserialization by serde.
pub async fn load_profile(name: &str) -> Result<WeldProfile, FileIoError> {
    let profile = WeldProfile::load(name).await?;

    // Verify that the loaded profile's name matches what we expected
    if profile.name != name {
        warn!(
            "Profile file name mismatch: expected '{}', got '{}' in file",
            name, profile.name
        );
    }

    info!("Loaded profile '{}' from {:?}", name, name_to_path(name));
    Ok(profile)
}

/// Deletes a welding profile from disk.
///
/// This operation is irreversible. Once deleted, the profile cannot be recovered.
pub async fn delete_profile(name: &str) -> Result<(), FileIoError> {
    WeldProfile::delete(name).await?;
    info!("Deleted profile '{}' from {:?}", name, name_to_path(name));
    Ok(())
}

/// Lists all available welding profiles, returning lightweight metadata.
///
/// This function reads all profile files in the profiles directory and extracts
/// just the name and description for display purposes. Files that cannot be
/// parsed are logged and skipped rather than causing the entire operation to fail.
pub async fn list_profiles() -> Result<Vec<ProfileListEntry>, FileIoError> {
    let names = WeldProfile::list().await?;
    let mut profiles = Vec::new();

    for name in names {
        match WeldProfile::load(&name).await {
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

/// Checks if a profile with the given name exists on disk.
///
/// This is used to prevent accidental overwrites and to provide user feedback.
pub async fn profile_exists(name: &str) -> bool {
    if validate_filename(name).is_err() {
        return false;
    }

    match tokio::fs::try_exists(name_to_path(name)).await {
        Ok(exists) => exists,
        Err(e) => {
            warn!("Failed to check profile existence for '{}': {}", name, e);
            false
        }
    }
}
