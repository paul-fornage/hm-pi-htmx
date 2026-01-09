use serde::{Deserialize, Serialize};
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::ModbusValue;
use super::{CLEARCORE_STATIC_CONFIG_COILS, CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearcoreConfig {
    pub coils: Vec<(String, Option<bool>)>,
    pub analog_registers: Vec<(String, Option<u16>)>,
}

impl ClearcoreConfig {
    pub async fn from_modbus(cached_modbus: &CachedModbus) -> Self {
        let mut coils = Vec::new();
        for info in CLEARCORE_STATIC_CONFIG_COILS.iter() {
            let value = match cached_modbus.read(&info.meta.address).await {
                Some(ModbusValue::Bool(val)) => Some(val),
                _ => None,
            };
            coils.push((info.meta.name.to_string(), value));
        }

        let mut analog_registers = Vec::new();
        for info in CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter() {
            let value = match cached_modbus.read(&info.meta.address).await {
                Some(ModbusValue::U16(val)) => Some(val),
                _ => None,
            };
            analog_registers.push((info.meta.name.to_string(), value));
        }

        Self {
            coils,
            analog_registers,
        }
    }
}
