use tokio::fs;
use log::{error, info};
use super::config_data::ClearcoreConfig;

const CONFIG_PATH: &str = "./clearcore_static_config.json";

/// Saves the clearcore configuration to disk.
/// 
/// This function writes critical machine parameters. The configuration is
/// saved to a single file that gets overwritten on each save.
pub async fn save_config(config: &ClearcoreConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config).map_err(|e| {
        error!("Failed to serialize clearcore config: {}", e);
        format!("Failed to serialize config: {}", e)
    })?;

    fs::write(CONFIG_PATH, json).await.map_err(|e| {
        error!("Failed to write clearcore config to {}: {}", CONFIG_PATH, e);
        format!("Failed to write config to disk: {}", e)
    })?;

    info!("Saved clearcore config to {}", CONFIG_PATH);
    Ok(())
}

/// Loads the clearcore configuration from disk.
/// 
/// Returns None if the file doesn't exist (first-time setup).
pub async fn load_config() -> Result<Option<ClearcoreConfig>, String> {
    if !std::path::Path::new(CONFIG_PATH).exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(CONFIG_PATH).await.map_err(|e| {
        error!("Failed to read clearcore config from {}: {}", CONFIG_PATH, e);
        format!("Failed to read config from disk: {}", e)
    })?;

    let config = serde_json::from_str::<ClearcoreConfig>(&json).map_err(|e| {
        error!("Failed to parse clearcore config: {}", e);
        format!("Failed to parse config: {}", e)
    })?;

    info!("Loaded clearcore config from {}", CONFIG_PATH);
    Ok(Some(config))
}
