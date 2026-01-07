use std::path::{Path, PathBuf};
use tokio::fs;
use log::{error, info, warn};
use super::weld_profile::{WeldProfile, ProfileListEntry};

const PROFILES_DIR: &str = "./weld_profiles";
const PROFILE_EXTENSION: &str = "json";

/// Ensures the profiles directory exists, creating it if necessary.
async fn ensure_profiles_dir() -> Result<(), String> {
    if let Err(e) = fs::create_dir_all(PROFILES_DIR).await {
        error!("Failed to create profiles directory: {}", e);
        return Err(format!("Failed to create profiles directory: {}", e));
    }
    Ok(())
}

/// Converts a profile name to a filesystem path.
fn name_to_path(name: &str) -> PathBuf {
    Path::new(PROFILES_DIR).join(format!("{}.{}", name, PROFILE_EXTENSION))
}

/// Validates that a filename is safe (basic check - frontend should do main sanitization).
fn is_valid_filename(name: &str) -> bool {
    !name.is_empty()
        && !name.contains('/')
        && !name.contains('\\')
        && name != "."
        && name != ".."
}

/// Saves a welding profile to disk.
/// Returns an error if the file operation fails.
pub async fn save_profile(profile: &WeldProfile) -> Result<(), String> {
    if !is_valid_filename(&profile.name) {
        error!("Invalid profile name: {}", profile.name);
        return Err("Invalid profile name".to_string());
    }

    ensure_profiles_dir().await?;

    let path = name_to_path(&profile.name);

    let json = match serde_json::to_string_pretty(profile) {
        Ok(j) => j,
        Err(e) => {
            error!("Failed to serialize profile {}: {}", profile.name, e);
            return Err(format!("Failed to serialize profile: {}", e));
        }
    };

    match fs::write(&path, json).await {
        Ok(_) => {
            info!("Saved profile {} to {:?}", profile.name, path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to write profile {} to disk: {}", profile.name, e);
            Err(format!("Failed to write profile to disk: {}", e))
        }
    }
}

/// Loads a welding profile from disk.
/// Returns an error if the file doesn't exist or cannot be read/parsed.
pub async fn load_profile(name: &str) -> Result<WeldProfile, String> {
    if !is_valid_filename(name) {
        error!("Invalid profile name: {}", name);
        return Err("Invalid profile name".to_string());
    }

    let path = name_to_path(name);

    let json = match fs::read_to_string(&path).await {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read profile {} from disk: {}", name, e);
            return Err(format!("Failed to read profile from disk: {}", e));
        }
    };

    match serde_json::from_str::<WeldProfile>(&json) {
        Ok(profile) => {
            info!("Loaded profile {} from {:?}", name, path);
            Ok(profile)
        }
        Err(e) => {
            error!("Failed to parse profile {} from JSON: {}", name, e);
            Err(format!("Failed to parse profile: {}", e))
        }
    }
}

/// Deletes a welding profile from disk.
/// Returns an error if the file doesn't exist or cannot be deleted.
pub async fn delete_profile(name: &str) -> Result<(), String> {
    if !is_valid_filename(name) {
        error!("Invalid profile name: {}", name);
        return Err("Invalid profile name".to_string());
    }

    let path = name_to_path(name);

    match fs::remove_file(&path).await {
        Ok(_) => {
            info!("Deleted profile {} from {:?}", name, path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to delete profile {} from disk: {}", name, e);
            Err(format!("Failed to delete profile from disk: {}", e))
        }
    }
}

/// Lists all available welding profiles, returning lightweight metadata.
/// Returns an empty list if the profiles directory doesn't exist or is empty.
pub async fn list_profiles() -> Result<Vec<ProfileListEntry>, String> {
    // If directory doesn't exist yet, return empty list
    if !Path::new(PROFILES_DIR).exists() {
        return Ok(Vec::new());
    }

    let mut entries = match fs::read_dir(PROFILES_DIR).await {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read profiles directory: {}", e);
            return Err(format!("Failed to read profiles directory: {}", e));
        }
    };

    let mut profiles = Vec::new();

    while let Some(entry) = entries.next_entry().await.transpose() {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("Failed to read directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        // Only process files with the correct extension
        if path.extension().and_then(|s| s.to_str()) != Some(PROFILE_EXTENSION) {
            continue;
        }

        // Try to load just the metadata (name and description) from the file
        let json = match fs::read_to_string(&path).await {
            Ok(content) => content,
            Err(e) => {
                warn!("Failed to read profile file {:?}: {}", path, e);
                continue;
            }
        };

        match serde_json::from_str::<WeldProfile>(&json) {
            Ok(profile) => {
                profiles.push(ProfileListEntry::from(&profile));
            }
            Err(e) => {
                warn!("Failed to parse profile file {:?}: {}", path, e);
                continue;
            }
        }
    }

    // Sort alphabetically by name for consistent ordering
    profiles.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(profiles)
}

/// Checks if a profile with the given name exists on disk.
pub async fn profile_exists(name: &str) -> bool {
    if !is_valid_filename(name) {
        return false;
    }

    let path = name_to_path(name);
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_validation() {
        assert!(is_valid_filename("my_profile"));
        assert!(is_valid_filename("Profile-123"));
        assert!(!is_valid_filename(""));
        assert!(!is_valid_filename("/etc/passwd"));
        assert!(!is_valid_filename("../secret"));
        assert!(!is_valid_filename("."));
        assert!(!is_valid_filename(".."));
    }
}
