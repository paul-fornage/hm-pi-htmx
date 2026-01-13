use std::collections::HashMap;
use crate::error_targeted;
use crate::views::clearcore_static_config::{CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS, CLEARCORE_STATIC_CONFIG_COILS, CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS};
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;


#[derive(Debug, thiserror::Error)]
pub enum CcConfigParseError {
    #[error("Failed to deserialize JSON: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Missing expected key: {0}")]
    MissingField(String),
    #[error("Expected boolean value for key {0}, found {1:?}")]
    InvalidCoilValue(String, serde_json::Value),
    #[error("Expected integer 0..<65536 for key {0}, found {1:?}")]
    InvalidHregValue(String, serde_json::Value),
    #[error("Expected map for key {0}, found {1:?}")]
    InvalidMapValue(String, serde_json::Value),
    #[error("Unexpected key in clearcore config: {0}")]
    UnexpectedKey(String),
    #[error("Wrong number of keys in map {0}. expected {1}, found {2}")]
    WrongNumberOfKeys(String, usize, usize),
}

fn get_map(json: &serde_json::Value, key: String) -> Result<serde_json::Map<String, serde_json::Value>, CcConfigParseError> {
    match json.get(&key){
        Some(v) => {
            match v.as_object() {
                Some(m) => Ok(m.clone()),
                None => {
                    error_targeted!(FS, "Failed to parse clearcore config: {key} object is not a map");
                    Err(CcConfigParseError::InvalidMapValue(key, v.clone()))
                }
            }
        }
        None => {
            error_targeted!(FS, "Failed to parse clearcore config: missing {key} object");
            Err(CcConfigParseError::MissingField(key))
        },
    }
}

impl ClearcoreConfig {
    pub fn deserialize(json_str: &str) -> Result<Self, CcConfigParseError> {
        let root: serde_json::Value = serde_json::from_str(json_str)?;

        // 1. Process Coils
        let coils_map = get_map(&root, Self::COILS_KEY.to_string())?;


        // strict check: ensure no extra or missing keys exist in the JSON
        if coils_map.len() != CLEARCORE_STATIC_CONFIG_COILS.len() {
            return Err(CcConfigParseError::WrongNumberOfKeys(
                Self::COILS_KEY.to_string(), CLEARCORE_STATIC_CONFIG_COILS.len(), coils_map.len()
            ));
        }

        let mut coils = HashMap::with_capacity(coils_map.len());
        for info in CLEARCORE_STATIC_CONFIG_COILS.iter() {
            let key = info.meta.name; // This gives us the &'static str we need
            let value_ref = coils_map.get(key).ok_or_else(|| {
                error_targeted!(FS, "Failed to parse clearcore config: missing coil {}", key);
                CcConfigParseError::MissingField(key.to_string())
            })?;

            let bool_value = value_ref.as_bool().ok_or_else(|| {
                error_targeted!(FS, "Failed to parse clearcore config: invalid coil value {}", key);
                CcConfigParseError::InvalidCoilValue(key.to_string(), value_ref.clone())
            })?;

            coils.insert(key, bool_value);
        }

        // 2. Process Analog Registers
        let regs_map = get_map(&root, Self::ANALOG_REGISTERS_KEY.to_string())?;

        if regs_map.len() != CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.len() {
            return Err(CcConfigParseError::WrongNumberOfKeys(
                Self::ANALOG_REGISTERS_KEY.to_string(),
                CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.len(),
                regs_map.len()
            ));
        }

        let mut analog_registers = HashMap::with_capacity(regs_map.len());
        for info in CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter() {
            let key = info.meta.name; // This gives us the &'static str we need
            let value_ref = regs_map.get(key).ok_or_else(|| {
                error_targeted!(FS, "Failed to parse clearcore config: missing hreg {}", key);
                CcConfigParseError::MissingField(key.to_string())
            })?;

            let reg_value = match value_ref.as_u64() {
                Some(val) if val <= u16::MAX as u64 => val as u16,
                Some(too_high) => {
                    error_targeted!(FS, "Failed to parse clearcore config: \
                        invalid hreg value {key} (too large: {too_high})");
                    return Err(CcConfigParseError::InvalidHregValue(key.to_string(), value_ref.clone()));
                },
                None => {
                    error_targeted!(FS, "Failed to parse clearcore config: invalid hreg value {key}");
                    return Err(CcConfigParseError::InvalidHregValue(key.to_string(), value_ref.clone()));
                }
            };
            
            analog_registers.insert(key, reg_value);
        }
        let dword_regs_map = get_map(&root, Self::ANALOG_DWORD_REGISTERS_KEY.to_string())?;

        if dword_regs_map.len() != CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.len() {
            return Err(CcConfigParseError::WrongNumberOfKeys(
                Self::ANALOG_DWORD_REGISTERS_KEY.to_string(),
                CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.len(),
                dword_regs_map.len()
            ));
        }

        let mut analog_dword_registers = HashMap::with_capacity(dword_regs_map.len());
        for info in CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.iter() {
            let key = info.get_meta().name;
            let value_ref = dword_regs_map.get(key).ok_or_else(|| {
                error_targeted!(FS, "Failed to parse clearcore config: missing dwrod hreg {}", key);
                CcConfigParseError::MissingField(key.to_string())
            })?;

            let dword_reg_value = match value_ref.as_u64() {
                Some(val) if val <= u32::MAX as u64 => val as u32,
                Some(too_high) => {
                    error_targeted!(FS, "Failed to parse clearcore config: \
                        invalid dword hreg value {key} (too large: {too_high})");
                    return Err(CcConfigParseError::InvalidHregValue(key.to_string(), value_ref.clone()));
                },
                None => {
                    error_targeted!(FS, "Failed to parse clearcore config: invalid dword hreg value {key}");
                    return Err(CcConfigParseError::InvalidHregValue(key.to_string(), value_ref.clone()));
                }
            };

            analog_dword_registers.insert(key, dword_reg_value);
        }

        Ok(Self {
            coils,
            analog_registers,
            analog_dword_registers,
        })
    }
}