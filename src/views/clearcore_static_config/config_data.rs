use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::HmPiError;
use crate::{error_targeted, warn_targeted};
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterMetadata};
use crate::plc::plc_register_definitions::get_clearcore_register_metadata;
use super::{CLEARCORE_STATIC_CONFIG_COILS, CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS, CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS};

#[derive(Debug, Clone, Serialize)]
pub struct ClearcoreConfig {
    pub coils: HashMap<&'static str, bool>,
    pub analog_registers: HashMap<&'static str, u16>,
    pub analog_dword_registers: HashMap<&'static str, u32>,
}

impl ClearcoreConfig {
    pub const COILS_KEY: &'static str = "coils";
    pub const ANALOG_REGISTERS_KEY: &'static str = "analog_registers";
    pub const ANALOG_DWORD_REGISTERS_KEY: &'static str = "analog_dword_registers";

    pub async fn from_modbus(cached_modbus: &CachedModbus) -> Result<Self, HmPiError> {
        let cap = CLEARCORE_STATIC_CONFIG_COILS.len();
        let mut coils = HashMap::<&'static str, bool>::with_capacity(cap);
        for info in CLEARCORE_STATIC_CONFIG_COILS.iter() {
            match cached_modbus.read(&info.meta.address).await {
                Some(ModbusValue::Bool(val)) => {
                    coils.insert(info.meta.name, val);
                },
                _ => {
                    error_targeted!(MODBUS, "Failed to read coil {} from modbus for static config", info.meta.name);
                    return Err(HmPiError::MissingExpectedRegister(info.meta.address.clone(), info.meta.name.to_string()));
                },
            };
        }

        let cap = CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.len();
        let mut analog_registers = HashMap::<&'static str, u16>::with_capacity(cap);
        for info in CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter() {
            match cached_modbus.read(&info.meta.address).await {
                Some(ModbusValue::U16(val)) => {
                    analog_registers.insert(info.meta.name, val);
                },
                _ => {
                    error_targeted!(MODBUS, "Failed to read hreg {} from modbus for static config", info.meta.name);
                    return Err(HmPiError::MissingExpectedRegister(info.meta.address.clone(), info.meta.name.to_string()));
                },
            };
        }

        let cap = CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.len();
        let mut analog_dword_registers = HashMap::<&'static str, u32>::with_capacity(cap);
        for info in CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.iter() {
            match cached_modbus.read(&info.get_.address).await {
                Some(ModbusValue::U16(val)) => {
                    analog_registers.insert(info.meta.name, val);
                },
                _ => {
                    error_targeted!(MODBUS, "Failed to read hreg {} from modbus for static config", info.meta.name);
                    return Err(HmPiError::MissingExpectedRegister(info.meta.address.clone(), info.meta.name.to_string()));
                },
            };
        }


        Ok(Self {
            coils,
            analog_registers,
        })
    }

    /// Get all registers that differ between self and the one in mb cache.
    pub async fn modbus_diff(&self, cached_modbus: &CachedModbus) -> Result<Vec<&'static RegisterMetadata>, HmPiError> {
        let mut diff = Vec::new();
        for (name, value) in self.coils.iter() {
            let metadata = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?;

            match cached_modbus.read_coil(metadata.address.address).await{
                Some(val) => {
                    if val != *value {
                        diff.push(metadata)
                    }
                },
                None => {
                    warn_targeted!(MODBUS, "Failed to read coil register: {}", metadata.name);
                    diff.push(metadata)
                }
            }
        }
        for (name, value) in self.analog_registers.iter() {
            let metadata = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?;
            match cached_modbus.read_hreg(metadata.address.address).await{
                Some(val) => {
                    if val != *value {
                        diff.push(metadata)
                    }
                },
                None => {
                    warn_targeted!(MODBUS, "Failed to read holding register: {}", metadata.name);
                    diff.push(metadata)
                }
            }
        }
        Ok(diff)
    }

    pub async fn apply_to_modbus(&self, cached_modbus: &CachedModbus) -> Result<(), HmPiError> {
        for (&name, &value) in self.coils.iter() {
            let address = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?.address.address;
            cached_modbus.diff_write_coil(address, value).await?;
        }
        for (&name, &value) in self.analog_registers.iter() {
            let address = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?.address.address;
            cached_modbus.diff_write_hreg(address, value).await?;
        }
        Ok(())
    }

    pub async fn write_to_modbus(&self, cached_modbus: &CachedModbus) -> Result<(), HmPiError> {
        for (&name, &value) in self.coils.iter() {
            let address = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?.address.address;
            cached_modbus.write_coil(address, value).await?;
        }
        for (&name, &value) in self.analog_registers.iter() {
            let address = get_clearcore_register_metadata(name).ok_or(
                HmPiError::CcConfigBadRegisterKey(name.to_string()))?.address.address;
            cached_modbus.write_hreg(address, value).await?;
        }
        Ok(())
    }
}
