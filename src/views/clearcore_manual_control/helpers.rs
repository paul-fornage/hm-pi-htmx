use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterAddress};

pub(super) async fn read_bool(
    registers: &CachedModbus,
    address: &RegisterAddress,
) -> Result<bool, String> {
    match registers.read(address).await {
        Some(ModbusValue::Bool(value)) => Ok(value),
        Some(ModbusValue::U16(_)) => Err(format!("Register at {address} is not a boolean")),
        None => Err(format!("Register at {address} could not be read")),
    }
}

pub(super) async fn read_u16(
    registers: &CachedModbus,
    address: &RegisterAddress,
) -> Result<u16, String> {
    match registers.read(address).await {
        Some(ModbusValue::U16(value)) => Ok(value),
        Some(ModbusValue::Bool(_)) => Err(format!("Register at {address} is not a u16")),
        None => Err(format!("Register at {address} could not be read")),
    }
}
