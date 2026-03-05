use std::path::Path;
use log::info;
use crate::file_io::{FileIoError, FixedDiskFile, serialize_json};
use crate::paths::subdirs::Subdir;
use super::config_data::ClearcoreConfig;

impl FixedDiskFile for ClearcoreConfig {
    const SUBDIR: Subdir = Subdir::Config;
    const FILE_NAME: &'static str = "clearcore_static_config.json";

    fn serialize_value(&self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(self, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        ClearcoreConfig::deserialize(contents).map_err(|e| FileIoError::Validation {
            message: format!("Clearcore config parse error at {}: {}", path.display(), e),
        })
    }
}

impl ClearcoreConfig {
    /// Saves the clearcore configuration to disk.
    ///
    /// This function writes critical machine parameters. The configuration is
    /// saved to a single file that gets overwritten on each save.
    pub async fn save_to_file(&self) -> Result<(), FileIoError> {
        self.save().await?;
        let path = Subdir::Config.full_local_path().join(<Self as FixedDiskFile>::FILE_NAME);
        info!("Saved clearcore config to {}", path.display());
        Ok(())
    }

    /// Loads the clearcore configuration from disk.
    ///
    /// Returns a FileIoError::NotFound if the file doesn't exist (first-time setup).
    pub async fn load_config() -> Result<Self, FileIoError> {
        let config = Self::load().await?;
        let path = Subdir::Config.full_local_path().join(<Self as FixedDiskFile>::FILE_NAME);
        info!("Loaded clearcore config from {}", path.display());
        Ok(config)
    }
}
