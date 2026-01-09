use crate::modbus::{ModbusValue, RegisterAddress};
use crate::modbus::cached_modbus::CachedModbus;

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

pub async fn mb_read_bool_helper(cache: &CachedModbus, address: &RegisterAddress) -> Option<bool> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, cache.read(address)).await {
        Ok(Some(ModbusValue::Bool(val))) => Some(val),
        _ => None,
    }
}

pub async fn mb_read_word_helper(cache: &CachedModbus, address: &RegisterAddress) -> Option<u16> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, cache.read(address)).await {
        Ok(Some(ModbusValue::U16(val))) => Some(val),
        _ => None,
    }
}
