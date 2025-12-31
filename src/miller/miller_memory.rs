use std::cmp::PartialEq;
use std::collections::{HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::modbus::modbus_transaction_types::*;
// Adjust import path for ModbusManager as needed
use crate::modbus::{ModbusAddressType, ModbusManager, ModbusState, ModbusValue, RegisterAddress, RegisterMetadata};
use crate::error::{Error, Result};
use crate::{debug_targeted, error_targeted};
use crate::miller::miller_register_definitions::{ERROR_REG_1, ERROR_REG_2};

#[derive(Clone)]
pub struct MillerMemory {
    pub manager: ModbusManager,

    // Thread-safe caches
    coils: Arc<RwLock<HashMap<u16, bool>>>,
    discrete_inputs: Arc<RwLock<HashMap<u16, bool>>>,
    holding_registers: Arc<RwLock<HashMap<u16, u16>>>,
    input_registers: Arc<RwLock<HashMap<u16, u16>>>,
}


impl MillerMemory {
    /// Create a new cache manager.
    /// This will sort and batch the provided registers into chunks of 100.
    pub fn new(manager: ModbusManager, registers: &[RegisterMetadata]) -> Self {
        let mut coil_addrs = Vec::new();
        let mut disc_addrs = Vec::new();
        let mut hold_addrs = Vec::new();
        let mut input_addrs = Vec::new();

        for reg in registers {
            match reg.address.register_type {
                ModbusAddressType::Coil => coil_addrs.push(reg.address.address),
                ModbusAddressType::DiscreteInput => disc_addrs.push(reg.address.address),
                ModbusAddressType::HoldingRegister => hold_addrs.push(reg.address.address),
                ModbusAddressType::InputRegister => input_addrs.push(reg.address.address),
            }
        }

        Self {
            manager,
            coils: Arc::new(RwLock::new(HashMap::new())),
            discrete_inputs: Arc::new(RwLock::new(HashMap::new())),
            holding_registers: Arc::new(RwLock::new(HashMap::new())),
            input_registers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn clear_cache(&self) {
        self.coils.write().await.clear();
        self.discrete_inputs.write().await.clear();
        self.holding_registers.write().await.clear();
        self.input_registers.write().await.clear();
    }

    pub async fn get_connection_state(&self) -> Result<ModbusState> {
        self.manager.get_connection_state().await
    }

    pub async fn update(&self) -> Result<()> {
        // TODO round robin chunk this up
        if self.manager.get_connection_state().await? != ModbusState::Connected {
            self.clear_cache().await;
            return Err(Error::NotConnected)
        } else {
            self.update_coils(0, 21).await?;
            self.update_discs(2000, 19).await?;
            self.update_ireg_chunk(4016, 22).await?;
            self.update_ireg_chunk(4099, 5).await?;
            self.update_ireg_chunk(4200, 7).await?;
            self.update_ireg_chunk(4300, 8).await?;
            self.update_ireg_chunk(4400, 9).await?;
            self.update_hreg_chunk(6000, 4).await?;
            self.update_hreg_chunk(6100, 4).await?;
            self.update_hreg_chunk(6200, 18).await?;
            self.update_hreg_chunk(6300, 19).await?;
        }
        Ok(())
    }

    pub async fn update_coils(&self, start: u16, count: u16) -> Result<()> {
        let coils = self.manager.read_coils(ReadCoilsRequest { address: start, count }).await?.values;
        for (i, v) in coils.iter().enumerate() {
            self.coils.write().await.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    pub async fn update_discs(&self, start: u16, count: u16) -> Result<()> {
        let discs = self.manager.read_discrete_inputs(ReadDiscreteInputsRequest { address: start, count }).await?.values;
        for (i, v) in discs.iter().enumerate() {
            self.discrete_inputs.write().await.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    pub async fn update_ireg_chunk(&self, start: u16, count: u16) -> Result<()> {
        let iregs = self.manager.read_input_registers(ReadInputRegistersRequest { address: start, count }).await?.values;
        for (i, v) in iregs.iter().enumerate() {
            self.input_registers.write().await.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    pub async fn update_hreg_chunk(&self, start: u16, count: u16) -> Result<()> {
        let hregs = self.manager.read_holding_registers(ReadHoldingRegistersRequest { address: start, count }).await?.values;
        for (i, v) in hregs.iter().enumerate() {
            self.holding_registers.write().await.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    /// Reads a value directly from the local cache. Returns None if value hasn't been cached yet.
    pub async fn read(&self, address: &RegisterAddress) -> Option<ModbusValue> {
        let addr = address.address;
        match address.register_type {
            ModbusAddressType::Coil => {
                self.coils.read().await.get(&addr).map(|&v| ModbusValue::Bool(v))
            }
            ModbusAddressType::DiscreteInput => {
                self.discrete_inputs.read().await.get(&addr).map(|&v| ModbusValue::Bool(v))
            }
            ModbusAddressType::HoldingRegister => {
                self.holding_registers.read().await.get(&addr).map(|&v| ModbusValue::U16(v))
            }
            ModbusAddressType::InputRegister => {
                self.input_registers.read().await.get(&addr).map(|&v| ModbusValue::U16(v))
            }
        }
    }

    pub async fn read_u32(&self, address: &RegisterAddress) -> Option<u32> {
        let addr = address.address;
        let guard = match address.register_type {
            ModbusAddressType::HoldingRegister => {
                self.holding_registers.read().await

            }
            ModbusAddressType::InputRegister => {
                self.input_registers.read().await
            }
            _ => {
                error_targeted!(MODBUS, "tried to read unsupported register type: {:?}", address.register_type);
                return None;
            }
        };
        let lower = guard.get(&addr);
        let upper = guard.get(&(addr + 1));
        match (lower, upper) {
            (Some(&lower), Some(&upper)) => {
                let number: u32 = ((upper as u32) << 16) | lower as u32;
                Some(number)
            },
            _ => None,
        }
    }

    pub async fn read_coil(&self, address: u16) -> Option<bool> { self.coils.read().await.get(&address).cloned() }
    pub async fn read_disc(&self, address: u16) -> Option<bool> { self.discrete_inputs.read().await.get(&address).cloned() }
    pub async fn read_ireg(&self, address: u16) -> Option<u16> { self.input_registers.read().await.get(&address).cloned() }
    pub async fn read_hreg(&self, address: u16) -> Option<u16> { self.holding_registers.read().await.get(&address).cloned() }

    pub async fn write_coil(&self, address: u16, value: bool) -> Result<()> {
        self.manager.write_single_coil(WriteSingleCoilRequest { address, value }).await
    }
    pub async fn write_hreg(&self, address: u16, value: u16) -> Result<()> {
        self.manager.write_single_register(WriteSingleRegisterRequest { address, value }).await
    }

    /// Writes a value to the device.
    /// DOES NOT update the cache speculatively.
    pub async fn write(&self, address: &RegisterAddress, value: ModbusValue) -> Result<()> {
        let addr = address.address;
        match (address.register_type.clone(), value) {
            (ModbusAddressType::Coil, ModbusValue::Bool(v)) => {
                self.manager.write_single_coil(WriteSingleCoilRequest { address: addr, value: v }).await
            }
            (ModbusAddressType::HoldingRegister, ModbusValue::U16(v)) => {
                self.manager.write_single_register(WriteSingleRegisterRequest { address: addr, value: v }).await
            }
            // Error on type mismatch or read-only types
            (ModbusAddressType::DiscreteInput, value) | (ModbusAddressType::InputRegister, value) => {
                Err(Error::LocalRegisterTriedWriteReadOnly(value, address.clone()))
            }
            (_, value) => {
                // Mismatch between metadata type and provided value type
                Err(Error::LocalRegisterTypeMismatch(value, address.clone()))
            }
        }
    }
}