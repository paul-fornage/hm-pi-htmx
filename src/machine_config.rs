use crate::miller::miller_register_types::WelderModel;
use crate::error::{Result, HmPiError};

use std::path::Path;
use std::fs;

pub const MACHINE_CONFIG_PATH: &str = "machine_config.json";

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MachineConfig {
    pub welder_model: WelderModel,
    pub udp_logging_port: u16,
    #[serde(default)]
    pub ps_ui_disable: bool,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            welder_model: WelderModel::Maxstar210,
            udp_logging_port: 42069,
            ps_ui_disable: false,
        }
    }
}

impl MachineConfig {
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: MachineConfig = serde_json::from_str(&contents)
            .map_err(|e| {
                // Check if it's a missing field error, which indicates version mismatch
                if e.classify() == serde_json::error::Category::Data {
                    HmPiError::ConfigVersionMismatch
                } else {
                    HmPiError::JsonError(e)
                }
            })?;
        Ok(config)
    }
}
