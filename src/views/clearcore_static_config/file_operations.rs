use tokio::fs;
use log::{error, info};
use crate::error::HmPiError;
use crate::error_targeted;
use super::config_data::ClearcoreConfig;

impl ClearcoreConfig{
    const CONFIG_PATH: &str = "./clearcore_static_config.json";

    /// Saves the clearcore configuration to disk.
    ///
    /// This function writes critical machine parameters. The configuration is
    /// saved to a single file that gets overwritten on each save.
    pub async fn save_to_file(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            error_targeted!(FS, "Failed to serialize clearcore config: {}", e);
            format!("Failed to serialize config: {}", e)
        })?;

        fs::write(Self::CONFIG_PATH, json).await.map_err(|e| {
            error!("Failed to write clearcore config to {}: {}", Self::CONFIG_PATH, e);
            format!("Failed to write config to disk: {}", e)
        })?;

        info!("Saved clearcore config to {}", Self::CONFIG_PATH);
        Ok(())
    }

    /// Loads the clearcore configuration from disk.
    ///
    /// Returns None if the file doesn't exist (first-time setup).
    pub async fn load_config() -> Result<Option<Self>, HmPiError> {
        if !std::path::Path::new(Self::CONFIG_PATH).exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(Self::CONFIG_PATH).await.map_err(|e| {
            error_targeted!(FS, "Failed to read clearcore config from {}: {}", Self::CONFIG_PATH, e);
            HmPiError::FailToReadFile(Self::CONFIG_PATH.into())
        })?;

        let config = ClearcoreConfig::deserialize(&json)?;

        info!("Loaded clearcore config from {}", Self::CONFIG_PATH);
        Ok(Some(config))
    }
}

