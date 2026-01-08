mod modbus;

use std::fmt::{Display, Formatter};
use std::hash::Hash;
pub use modbus::*;
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