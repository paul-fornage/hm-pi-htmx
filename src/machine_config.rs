use std::path::Path;
use crate::file_io::{FileIoError, FixedDiskFile, serialize_json};
use crate::miller::miller_register_types::WelderModel;
use crate::paths::subdirs::Subdir;

pub const MACHINE_CONFIG_PATH: &str = "machine_config.json";
const CONFIG_VERSION_MISMATCH_MSG: &str = "Configuration file version mismatch. Expected fields may not match. \
        This may indicate the file was created with a different version of the software.";

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

impl FixedDiskFile for MachineConfig {
    const SUBDIR: Subdir = Subdir::Config;
    const FILE_NAME: &'static str = MACHINE_CONFIG_PATH;

    fn serialize_value(&self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(self, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        match serde_json::from_str::<MachineConfig>(contents) {
            Ok(config) => Ok(config),
            Err(e) => {
                if e.classify() == serde_json::error::Category::Data {
                    Err(FileIoError::Validation {
                        message: CONFIG_VERSION_MISMATCH_MSG.to_string(),
                    })
                } else {
                    Err(FileIoError::Serde {
                        path: path.to_path_buf(),
                        source: e,
                    })
                }
            }
        }
    }
}
