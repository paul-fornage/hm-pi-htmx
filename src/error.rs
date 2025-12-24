

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
    ModbusProtocolError(tokio_modbus::Error),
    #[error("Modbus exception from server: {0}")]
    ModbusException(tokio_modbus::ExceptionCode),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Tried to connect to modbus when already connected or connecting.")]
    ModbusAlreadyConnected,
    #[error("The connection state does not match the expected invariant. Author has made a mistake. \
        This could also be a VERY precise race condition, if we check connection \
        and disconnect within a couple lines on different threads")]
    ModbusInvariantBroken,
    #[error("Timed out performing modbus operation. elapsed: {0}")]
    ModbusTimedOut(String),
    #[error("Timed out waiting for lock. elapsed: {0}")]
    LockTimedOut(String),
    #[error("Failed to unwind connect attempt. State was set to connecting, connection failed, \
        and then failed to set state to disconnected. Can only arise from lock acquisition timeout")]
    FailedToUnwindConnectAttempt(),
}



