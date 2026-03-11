mod modbus;

use crate::error::HmPiError;
use crate::modbus::cached_modbus::CachedModbus;
use crate::warn_targeted;
pub use modbus::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

pub mod modbus_transaction_types;
pub mod cached_modbus;

#[derive(Debug, Clone, PartialEq)]
pub enum ModbusAddressType{
    Coil,
    DiscreteInput,
    HoldingRegister,
    InputRegister,
}
impl Into<u8> for ModbusAddressType{
    fn into(self) -> u8{
        match self{
            ModbusAddressType::Coil => 0x01,
            ModbusAddressType::DiscreteInput => 0x02,
            ModbusAddressType::HoldingRegister => 0x03,
            ModbusAddressType::InputRegister => 0x04,
        }
    }
}
impl Display for ModbusAddressType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ModbusAddressType::Coil => write!(f, "Coil"),
            ModbusAddressType::DiscreteInput => write!(f, "Discrete Input"),
            ModbusAddressType::HoldingRegister => write!(f, "Holding Register"),
            ModbusAddressType::InputRegister => write!(f, "Input Register"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModbusValue {
    Bool(bool),
    U16(u16),
}
impl Display for ModbusValue{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ModbusValue::Bool(b) => write!(f, "{}", b),
            ModbusValue::U16(u) => write!(f, "{}", u),
        }
    }
}

impl ModbusValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ModbusValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            ModbusValue::U16(u) => Some(*u),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterAddress{
    pub register_type: ModbusAddressType,
    pub address: u16,
}
impl Hash for RegisterAddress{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl RegisterAddress{
    pub const fn new(register_type: ModbusAddressType, address: u16) -> Self{
        Self{register_type, address}
    }
}

impl Display for RegisterAddress{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:#04x}", self.register_type, self.address)
    }
}

pub struct RegisterMetadata{
    pub address: RegisterAddress,
    pub name: &'static str,
    pub description: &'static str,
}

impl RegisterMetadata{
    pub const fn new(address: RegisterAddress, name: &'static str, description: &'static str) -> Self{
        Self{address, name, description}
    }
    pub const fn new_raw(
        register_type: ModbusAddressType, address: u16,
        name: &'static str, description: &'static str) -> Self
    {
        Self{ address: RegisterAddress::new(register_type, address), name, description}
    }
}

impl Debug for RegisterMetadata{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.address, self.name)
    }
}


pub struct MbDiffStub{
    local_value: ModbusValue,
    value_in_mb: Option<ModbusValue>,
    register: &'static RegisterMetadata,
}

impl MbDiffStub{

    pub async fn check_bool(regs: &CachedModbus, register: &'static RegisterMetadata, local_value: bool) -> Option<Self>{
        match regs.read(&register.address).await{
            Some(ModbusValue::Bool(mb_val)) => if mb_val == local_value {
                None
            } else {
                Some(Self{local_value: ModbusValue::Bool(local_value), value_in_mb: Some(ModbusValue::Bool(mb_val)), register})
            },
            Some(ModbusValue::U16(other_val)) => {
                warn_targeted!(MODBUS, "Register {} is a U16, has value {} in modbus memory, but {} in local cache", register.name, other_val, local_value);
                None
            }
            None => None,
        }
    }

    pub async fn check_word(regs: &CachedModbus, register: &'static RegisterMetadata, local_value: u16) -> Option<Self>{
        match regs.read(&register.address).await{
            Some(ModbusValue::U16(mb_val)) => if mb_val == local_value {
                None
            } else {
                Some(Self{local_value: ModbusValue::U16(local_value), value_in_mb: Some(ModbusValue::U16(mb_val)), register})
            },
            Some(ModbusValue::Bool(other_val)) => {
                warn_targeted!(MODBUS, "Register {} is a bool, has value {} in modbus memory, but {} in local cache", register.name, other_val, local_value);
                None
            }
            None => None,
        }
    }

    pub async fn apply(&self, miller_regs: &CachedModbus) -> Result<(), HmPiError>{
        miller_regs.write(&self.register.address, self.local_value.clone()).await
    }

    pub fn new(local_value: ModbusValue, register: &'static RegisterMetadata) -> Self{
        Self{local_value, value_in_mb: None, register}
    }
}

pub fn dbg_helper(val: &Option<ModbusValue>) -> String{
    match val{
        Some(ModbusValue::Bool(b)) => format!("{:?}", b),
        Some(ModbusValue::U16(u)) => format!("{:?}", u),
        None => "None".to_string(),
    }
}

impl Debug for MbDiffStub{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Register{{addr: {:?} name: {}}} modbus value: {}, local value: {}", 
               self.register.address, self.register.name, dbg_helper(&self.value_in_mb), self.local_value)
    }
}
