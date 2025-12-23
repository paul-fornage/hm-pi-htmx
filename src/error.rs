
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Tried to perform modbus operation when not connected.")]
    NotConnected,
    #[error("Failed to connect to Modbus server: {0}")]
    ConnectionFailed(String),
    #[error("Modbus protocol/transport error: {0}")]
    ModbusProtocolError(String),
    #[error("Modbus exception from server: {0}")]
    ModbusException(String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Timed out performing modbus operation. elapsed: {0}")]
    ModbusTimedOut(String),
    #[error("Timed out waiting for lock. elapsed: {0}")]
    LockTimedOut(String),
}



