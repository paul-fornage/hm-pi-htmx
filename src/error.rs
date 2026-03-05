use crate::modbus::{ModbusAddressType, ModbusValue, RegisterAddress};
use crate::file_io::FileIoError;
use crate::views::clearcore_static_config::json_serde::CcConfigParseError;

pub type Result<T> = std::result::Result<T, HmPiError>;

#[derive(Debug, thiserror::Error)]
pub enum HmPiError {
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
    #[error("Tried to write a bool to a register that expects a u16 or vice versa. \
        value: {0:?}, address: {1:?})")]
    LocalRegisterTypeMismatch(ModbusValue, RegisterAddress),
    #[error("Tried to write to a read-only register. \
        value: {0:?}, address: {1:?})")]
    LocalRegisterTriedWriteReadOnly(ModbusValue, RegisterAddress),
    #[error("Tried to read a register from cache that has not been populated. \
        This could be because it isn't a real register, or MB isn't working. address: {0:?})")]
    ReadUnpopulatedRegister(RegisterAddress),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    FileIo(#[from] FileIoError),
    #[error("Configuration file version mismatch. Expected fields may not match. \
        This may indicate the file was created with a different version of the software.")]
    ConfigVersionMismatch,
    #[error("Invalid welder model")]
    InvalidWelderModel,
    #[error("Could not find a cached register \"{1}\" with address {0:?}.")]
    MissingExpectedRegister(RegisterAddress, String),
    #[error("failed to read file at {0}")]
    FailToReadFile(String),
    #[error("failed to write file at {0}")]
    FailToWriteFile(String),
    #[error("failed to parse file at {0}")]
    FailToParseFile(String),
    #[error("Clearcore static config error: {0}")]
    CcConfigError(#[from] CcConfigParseError),
    #[error("Clearcore static config contained register {0} that was not defined")]
    CcConfigBadRegisterKey(String),
    #[error("Failed to read back registers with the same value that was written to them.")]
    ModbusFailToReadBackWrites(),
}


