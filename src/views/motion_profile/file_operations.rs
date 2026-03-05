use std::path::{Path, PathBuf};
use tokio::fs;
use log::{error, info, warn};
use crate::paths::full_path_for_subdir;
use crate::paths::subdirs::MOTION_PROFILES;
use super::motion_profile::{MotionProfile, ProfileListEntry};

pub fn profile_path() -> PathBuf {
    full_path_for_subdir(MOTION_PROFILES)
}
const PROFILE_EXTENSION: &str = "json";

async fn ensure_profiles_dir() -> Result<(), String> {
    match fs::create_dir_all(profile_path()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to create profiles directory '{}': {}", profile_path().display(), e);
            Err(format!("Failed to create profiles directory: {}", e))
        }
    }
}

fn name_to_path(name: &str) -> PathBuf {
    profile_path().join(format!("{}.{}", name, PROFILE_EXTENSION))
}

fn validate_filename(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Profile name cannot be empty".to_string());
    }

    if name.len() > 240 {
        return Err("Profile name too long (maximum 240 characters)".to_string());
    }

    if name != name.trim() {
        return Err("Profile name cannot start or end with whitespace".to_string());
    }

    if name == "." || name == ".." {
        return Err("Invalid profile name".to_string());
    }

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
            _ => {}
        }
    }

    Ok(())
}

pub async fn save_profile(profile: &MotionProfile) -> Result<(), String> {
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

pub async fn load_profile(name: &str) -> Result<MotionProfile, String> {
    validate_filename(name)?;

    let path = name_to_path(name);

    let json = fs::read_to_string(&path).await.map_err(|e| {
        error!("Failed to read profile '{}' from {:?}: {}", name, path, e);
        format!("Failed to read profile from disk: {}", e)
    })?;

    let profile = serde_json::from_str::<MotionProfile>(&json).map_err(|e| {
        error!("Failed to parse profile '{}': {}", name, e);
        format!("Failed to parse profile: {}", e)
    })?;

    if profile.name != name {
        warn!(
            "Profile file name mismatch: expected '{}', got '{}' in file",
            name, profile.name
        );
    }

    info!("Loaded profile '{}' from {:?}", name, path);
    Ok(profile)
}

pub async fn delete_profile(name: &str) -> Result<(), String> {
    validate_filename(name)?;

    let path = name_to_path(name);

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

pub async fn list_profiles() -> Result<Vec<ProfileListEntry>, String> {
    let profile_path = profile_path();
    if !profile_path.exists() {
        return Ok(Vec::new());
    }

    let mut dir_entries = fs::read_dir(profile_path.clone()).await.map_err(|e| {
        error!("Failed to read profiles directory '{}': {}", profile_path.display(), e);
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

        if path.extension().and_then(|s| s.to_str()) != Some(PROFILE_EXTENSION) {
            continue;
        }

        if !path.is_file() {
            continue;
        }

        let json = match fs::read_to_string(&path).await {
            Ok(j) => j,
            Err(e) => {
                warn!("Failed to read profile file {:?}: {}", path, e);
                continue;
            }
        };

        let profile = match serde_json::from_str::<MotionProfile>(&json) {
            Ok(p) => p,
            Err(e) => {
                warn!("Failed to parse profile file {:?}: {}", path, e);
                continue;
            }
        };

        profiles.push(ProfileListEntry::from(&profile));
    }

    profiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(profiles)
}
