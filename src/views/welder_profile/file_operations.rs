use std::path::{Path, PathBuf};
use tokio::fs;
use log::{error, info, warn};
use super::weld_profile::{WeldProfile, ProfileListEntry};

const PROFILES_DIR: &str = "./weld_profiles";
const PROFILE_EXTENSION: &str = "json";

/// Ensures the profiles directory exists, creating it if necessary.
async fn ensure_profiles_dir() -> Result<(), String> {
    match fs::create_dir_all(PROFILES_DIR).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to create profiles directory '{}': {}", PROFILES_DIR, e);
            Err(format!("Failed to create profiles directory: {}", e))
        }
    }
}

/// Converts a profile name to a filesystem path.
fn name_to_path(name: &str) -> PathBuf {
    Path::new(PROFILES_DIR).join(format!("{}.{}", name, PROFILE_EXTENSION))
}

/// Validates that a filename is safe for filesystem use.
///
/// This is the last line of defense. A malicious or malformed filename could:
/// - Traverse directories (../, /etc/passwd)
/// - Overwrite system files
/// - Cause filesystem corruption
/// - Break on different operating systems
///
/// Requirements:
/// - Must not be empty
/// - Must contain only alphanumeric, underscore, hyphen, and space
/// - Must not start or end with whitespace
/// - Must not be "." or ".."
/// - Must not contain path separators (/, \)
/// - Must not exceed reasonable length (255 bytes is filesystem limit)
fn validate_filename(name: &str) -> Result<(), String> {
    // Check empty
    if name.is_empty() {
        return Err("Profile name cannot be empty".to_string());
    }

    // Check length (reserve space for extension)
    if name.len() > 240 {
        return Err("Profile name too long (maximum 240 characters)".to_string());
    }

    // Check for leading/trailing whitespace
    if name != name.trim() {
        return Err("Profile name cannot start or end with whitespace".to_string());
    }

    // Check for reserved names
    if name == "." || name == ".." {
        return Err("Invalid profile name".to_string());
    }

    // Check for path separators and other dangerous characters
    for c in name.chars() {
        match c {
            '/' | '\\' => {
                return Err("Profile name cannot contain path separators".to_string());
            }
            '\0' => {
                return Err("Profile name cannot contain null bytes".to_string());
            }
            '<' | '>' | ':' | '"' | '|' | '?' | '*' => {
                return Err(format!("Profile name cannot contain '{}'", c));
            }
            c if c.is_control() => {
                return Err("Profile name cannot contain control characters".to_string());
            }
            _ => {} // Character is acceptable
        }
    }

    Ok(())
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
pub async fn save_profile(profile: &WeldProfile) -> Result<(), String> {
    validate_filename(&profile.name)?;
    ensure_profiles_dir().await?;

    let path = name_to_path(&profile.name);

    let json = serde_json::to_string_pretty(profile).map_err(|e| {
        error!("Failed to serialize profile '{}': {}", profile.name, e);
        format!("Failed to serialize profile: {}", e)
    })?;

    fs::write(&path, json).await.map_err(|e| {
        error!("Failed to write profile '{}' to {:?}: {}", profile.name, path, e);
        format!("Failed to write profile to disk: {}", e)
    })?;

    info!("Saved profile '{}' to {:?}", profile.name, path);
    Ok(())
}

/// Loads a welding profile from disk.
///
/// This function reads critical machine parameters. A corrupted or malicious
/// profile file could cause the machine to operate with dangerous parameters.
/// All data is validated during deserialization by serde.
pub async fn load_profile(name: &str) -> Result<WeldProfile, String> {
    validate_filename(name)?;

    let path = name_to_path(name);

    let json = fs::read_to_string(&path).await.map_err(|e| {
        error!("Failed to read profile '{}' from {:?}: {}", name, path, e);
        format!("Failed to read profile from disk: {}", e)
    })?;

    let profile = serde_json::from_str::<WeldProfile>(&json).map_err(|e| {
        error!("Failed to parse profile '{}': {}", name, e);
        format!("Failed to parse profile: {}", e)
    })?;

    // Verify that the loaded profile's name matches what we expected
    if profile.name != name {
        warn!(
            "Profile file name mismatch: expected '{}', got '{}' in file",
            name, profile.name
        );
    }

    info!("Loaded profile '{}' from {:?}", name, path);
    Ok(profile)
}

/// Deletes a welding profile from disk.
///
/// This operation is irreversible. Once deleted, the profile cannot be recovered.
pub async fn delete_profile(name: &str) -> Result<(), String> {
    validate_filename(name)?;

    let path = name_to_path(name);

    // Verify file exists before attempting deletion
    if !path.exists() {
        return Err(format!("Profile '{}' does not exist", name));
    }

    fs::remove_file(&path).await.map_err(|e| {
        error!("Failed to delete profile '{}' from {:?}: {}", name, path, e);
        format!("Failed to delete profile from disk: {}", e)
    })?;

    info!("Deleted profile '{}' from {:?}", name, path);
    Ok(())
}

/// Lists all available welding profiles, returning lightweight metadata.
///
/// This function reads all profile files in the profiles directory and extracts
/// just the name and description for display purposes. Files that cannot be
/// parsed are logged and skipped rather than causing the entire operation to fail.
pub async fn list_profiles() -> Result<Vec<ProfileListEntry>, String> {
    // If directory doesn't exist yet, return empty list
    if !Path::new(PROFILES_DIR).exists() {
        return Ok(Vec::new());
    }

    let mut dir_entries = fs::read_dir(PROFILES_DIR).await.map_err(|e| {
        error!("Failed to read profiles directory '{}': {}", PROFILES_DIR, e);
        format!("Failed to read profiles directory: {}", e)
    })?;

    let mut profiles = Vec::new();

    while let Some(entry) = dir_entries.next_entry().await.transpose() {
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

        // Only process regular files, not directories or symlinks
        match entry.file_type().await {
            Ok(ft) if ft.is_file() => {},
            Ok(_) => continue, // Skip non-files
            Err(e) => {
                warn!("Failed to get file type for {:?}: {}", path, e);
                continue;
            }
        }

        // Try to load the profile metadata
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
    profiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(profiles)
}

/// Checks if a profile with the given name exists on disk.
///
/// This is used to prevent accidental overwrites and to provide user feedback.
pub async fn profile_exists(name: &str) -> bool {
    match validate_filename(name) {
        Ok(_) => {},
        Err(_) => return false,
    }

    let path = name_to_path(name);
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_validation() {
        // Valid names
        assert!(validate_filename("my_profile").is_ok());
        assert!(validate_filename("Profile-123").is_ok());
        assert!(validate_filename("Test Profile").is_ok());
        assert!(validate_filename("profile_2024_01_07").is_ok());

        // Invalid: empty
        assert!(validate_filename("").is_err());

        // Invalid: path traversal
        assert!(validate_filename("/etc/passwd").is_err());
        assert!(validate_filename("../secret").is_err());
        assert!(validate_filename("foo/bar").is_err());
        assert!(validate_filename("foo\\bar").is_err());

        // Invalid: reserved names
        assert!(validate_filename(".").is_err());
        assert!(validate_filename("..").is_err());

        // Invalid: whitespace issues
        assert!(validate_filename(" profile").is_err());
        assert!(validate_filename("profile ").is_err());
        assert!(validate_filename("  ").is_err());

        // Invalid: dangerous characters
        assert!(validate_filename("profile\0name").is_err());
        assert!(validate_filename("profile<name").is_err());
        assert!(validate_filename("profile>name").is_err());
        assert!(validate_filename("profile:name").is_err());
        assert!(validate_filename("profile\"name").is_err());
        assert!(validate_filename("profile|name").is_err());
        assert!(validate_filename("profile?name").is_err());
        assert!(validate_filename("profile*name").is_err());

        // Invalid: too long
        let long_name = "a".repeat(250);
        assert!(validate_filename(&long_name).is_err());
    }
}
