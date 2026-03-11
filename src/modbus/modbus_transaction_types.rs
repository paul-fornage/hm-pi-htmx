use serde::{Deserialize, Serialize};

// Read coils (function code 0x01)
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadCoilsRequest {
    pub address: u16,
    pub count: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadCoilsResponse {
    pub values: Vec<bool>,
}

// Read discrete inputs (function code 0x02)
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadDiscreteInputsRequest {
    pub address: u16,
    pub count: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadDiscreteInputsResponse {
    pub values: Vec<bool>,
}

// Read holding registers (function code 0x03)
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadHoldingRegistersRequest {
    pub address: u16,
    pub count: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadHoldingRegistersResponse {
    pub values: Vec<u16>,
}

// Read input registers (function code 0x04)
#[derive(Debug, Deserialize, Serialize)]
pub struct ReadInputRegistersRequest {
    pub address: u16,
    pub count: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReadInputRegistersResponse {
    pub values: Vec<u16>,
}

// Write single coil (function code 0x05)
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteSingleCoilRequest {
    pub address: u16,
    pub value: bool,
}

// Write single register (function code 0x06)
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteSingleRegisterRequest {
    pub address: u16,
    pub value: u16,
}

// Write multiple coils (function code 0x0F)
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteMultipleCoilsRequest {
    pub address: u16,
    pub values: Vec<bool>,
}

// Write multiple registers (function code 0x10)
#[derive(Debug, Deserialize, Serialize)]
pub struct WriteMultipleRegistersRequest {
    pub address: u16,
    pub values: Vec<u16>,
}
