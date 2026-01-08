use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Using the same imports as the original file
use crate::modbus::modbus_transaction_types::*;
use crate::modbus::{ModbusAddressType, ModbusManager, ModbusState, ModbusValue, RegisterAddress};
use crate::error::{Error, Result};
use crate::{debug_targeted, error_targeted};

/// Defines a contiguous chunk of Modbus registers to be polled.
#[derive(Clone, Debug)]
pub enum ModbusChunk {
    Coils { address: u16, count: u16 },
    DiscreteInputs { address: u16, count: u16 },
    InputRegisters { address: u16, count: u16 },
    HoldingRegisters { address: u16, count: u16 },
}

/// The main handle for the application. 
/// It is cheap to clone (all fields are Arcs or internal handles) and thread-safe.
/// It DOES NOT contain the update logic or the chunk list.
#[derive(Clone)]
pub struct CachedModbus {
    pub manager: ModbusManager,

    // Thread-safe caches
    coils: Arc<RwLock<HashMap<u16, bool>>>,
    discrete_inputs: Arc<RwLock<HashMap<u16, bool>>>,
    holding_registers: Arc<RwLock<HashMap<u16, u16>>>,
    input_registers: Arc<RwLock<HashMap<u16, u16>>>,
}

/// The driver struct responsible for polling.
/// This is NOT thread-safe and is intended to live in a single background task.
pub struct ModbusUpdater {
    target: CachedModbus,
    chunks: &'static[ModbusChunk],
    cursor: usize,
}

impl CachedModbus {
    /// Create a new pair of (Cache Access, Cache Updater).
    /// The `CachedModbus` can be cloned and sent to UI/Logic threads.
    /// The `ModbusUpdater` should be kept in the polling loop.
    pub fn new_with_updater(manager: ModbusManager, chunks: &'static[ModbusChunk]) -> (Self, ModbusUpdater) {
        let cache = Self {
            manager,
            coils: Arc::new(RwLock::new(HashMap::new())),
            discrete_inputs: Arc::new(RwLock::new(HashMap::new())),
            holding_registers: Arc::new(RwLock::new(HashMap::new())),
            input_registers: Arc::new(RwLock::new(HashMap::new())),
        };

        let updater = ModbusUpdater {
            target: cache.clone(),
            chunks,
            cursor: 0,
        };

        (cache, updater)
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

    // --- Internal Update Helpers (Called by Updater) ---

    async fn update_coils(&self, start: u16, count: u16) -> Result<()> {
        let coils = self.manager.read_coils(ReadCoilsRequest { address: start, count }).await?.values;
        let mut cache = self.coils.write().await;
        for (i, v) in coils.iter().enumerate() {
            cache.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    async fn update_discs(&self, start: u16, count: u16) -> Result<()> {
        let discs = self.manager.read_discrete_inputs(ReadDiscreteInputsRequest { address: start, count }).await?.values;
        let mut cache = self.discrete_inputs.write().await;
        for (i, v) in discs.iter().enumerate() {
            cache.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    async fn update_ireg_chunk(&self, start: u16, count: u16) -> Result<()> {
        let iregs = self.manager.read_input_registers(ReadInputRegistersRequest { address: start, count }).await?.values;
        let mut cache = self.input_registers.write().await;
        for (i, v) in iregs.iter().enumerate() {
            cache.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    async fn update_hreg_chunk(&self, start: u16, count: u16) -> Result<()> {
        let hregs = self.manager.read_holding_registers(ReadHoldingRegistersRequest { address: start, count }).await?.values;
        let mut cache = self.holding_registers.write().await;
        for (i, v) in hregs.iter().enumerate() {
            cache.insert(i as u16 + start, *v);
        }
        Ok(())
    }

    // --- Public Read/Write API ---

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
                error_targeted!(MODBUS, "tried to read unsupported register type for u32: {:?}", address.register_type);
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

impl ModbusUpdater {
    /// Performs a single update cycle.
    /// Returns:
    /// - Ok(()) if successful or if no chunks exist.
    /// - Err(Error::NotConnected) if manager is disconnected (cache is cleared).
    /// - Err(...) if the specific Modbus request failed.
    pub async fn update(&mut self) -> Result<()> {
        if self.target.manager.get_connection_state().await? != ModbusState::Connected {
            self.target.clear_cache().await;
            return Err(Error::NotConnected);
        }

        if self.chunks.is_empty() {
            return Ok(());
        }

        // Round Robin: Select current chunk
        // We use simple usize because this struct is not shared across threads.
        let chunk = &self.chunks[self.cursor];

        // Execute the update on the target cache
        match chunk {
            ModbusChunk::Coils { address, count } => {
                self.target.update_coils(*address, *count).await?;
            }
            ModbusChunk::DiscreteInputs { address, count } => {
                self.target.update_discs(*address, *count).await?;
            }
            ModbusChunk::InputRegisters { address, count } => {
                self.target.update_ireg_chunk(*address, *count).await?;
            }
            ModbusChunk::HoldingRegisters { address, count } => {
                self.target.update_hreg_chunk(*address, *count).await?;
            }
        }

        // Advance cursor, wrapping around safely
        self.cursor = (self.cursor + 1) % self.chunks.len();

        Ok(())
    }
}