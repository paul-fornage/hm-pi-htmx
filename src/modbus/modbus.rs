use tokio_modbus::client::tcp;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockWriteGuard};
use tokio::time::timeout;
use tokio_modbus::client::{Client, Context, Reader, Writer};
use serde::{Deserialize, Serialize};
use tokio_modbus::prelude::SlaveContext;
use crate::error::{Error, Result};
use crate::{error_targeted, info_targeted, warn_targeted, trace_targeted};
use crate::modbus::modbus_transaction_types::*;

const LOCK_TIMEOUT: Duration = Duration::from_secs(2);
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

// macro for all modbus operations (function codes)
macro_rules! execute_modbus {
    ($manager:expr, |$ctx:ident| $op:expr) => {{
        let op_name = stringify!($op);

        // 1. Get config (cloned)
        let config = match $manager.cloned_config().await {
            Ok(c) => c,
            Err(e) => {
                warn_targeted!(MODBUS, "Modbus Op '{}' failed (Config Lock): {:?}", op_name, e);
                return Err(e);
            }
        };

        // 2. Acquire lock (mutex)
        let mut ctx_guard = match $manager.ctx_acquisition().await {
            Ok(g) => g,
            Err(e) => {
                warn_targeted!(MODBUS, "Modbus Op '{}' failed (Ctx Lock): {:?}", op_name, e);
                return Err(e);
            }
        };

        // 3. Check connection
        if !matches!(config.state, ModbusState::Connected) {
             warn_targeted!(MODBUS, "Modbus Op '{}' failed: Not Connected", op_name);
             Err(Error::NotConnected)
        } else {
             // 4. Unwrap context
             match ctx_guard.as_mut() {
                Some($ctx) => {
                    trace_targeted!(MODBUS, "Modbus Op '{}' started", op_name);
                    // 5. Run the specific future with timeout
                    let fut = timeout(config.timeout_duration, $op);
                    match fut.await {
                        Ok(res) => {
                            let squashed = squash_error(res);
                            match &squashed {
                                Ok(val) => trace_targeted!(MODBUS, "Modbus Op '{}' succeeded: {:?}", op_name, val),
                                Err(e) => warn_targeted!(MODBUS, "Modbus Op '{}' failed: {:?}", op_name, e),
                            }
                            squashed
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            warn_targeted!(MODBUS, "Modbus Op '{}' timed out: {}", op_name, msg);
                            Err(Error::ModbusTimedOut(msg))
                        }
                    }
                }
                None => {
                    error_targeted!(MODBUS, "Modbus Op '{}' failed: Invariant Broken (Context is None)", op_name);
                    Err(Error::ModbusInvariantBroken)
                },
            }
        }
    }};
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionConfig {
    #[serde(skip, default)]
    pub state: ModbusState,
    pub socket_addr: SocketAddr,
    pub unit_id: u8,
    #[serde(with = "duration_as_millis")]
    pub timeout_duration: Duration,
}

mod duration_as_millis {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

pub const CLEARCORE_CONFIG_PATH: &str = "clearcore_modbus_config.json";
pub const WELDER_CONFIG_PATH: &str = "welder_modbus_config.json";

impl ConnectionConfig {
    pub fn new_with_timeout(socket_addr: SocketAddr, unit_id: u8, timeout_duration: Duration) -> Self {
        ConnectionConfig{
            state: ModbusState::Disconnected,
            socket_addr,
            unit_id,
            timeout_duration,
        }
    }
    pub fn new(socket_addr: SocketAddr, unit_id: u8) -> Self {
        ConnectionConfig{
            state: ModbusState::Disconnected,
            socket_addr,
            unit_id,
            timeout_duration: DEFAULT_TIMEOUT,
        }
    }

    pub fn save_to_path(&self, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        info_targeted!(MODBUS, "Saved config to {}", path);
        Ok(())
    }

    pub fn load_from_path(path: &str) -> Option<Self> {
        let path_obj = std::path::Path::new(path);
        if !path_obj.exists() {
            info_targeted!(MODBUS, "Config file {} does not exist", path);
            return None;
        }

        match std::fs::read_to_string(path_obj) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(config) => {
                    info_targeted!(MODBUS, "Loaded config from {}", path);
                    Some(config)
                }
                Err(e) => {
                    warn_targeted!(MODBUS, "Failed to parse config from {}: {}", path, e);
                    None
                }
            },
            Err(e) => {
                warn_targeted!(MODBUS, "Failed to read config from {}: {}", path, e);
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct ModbusManager {
    // TODO: Consider one thread has lock on Context and the other on ConnectionConfig.
    //  if they are both locked at the same time, and need the other, we could deadlock.
    shared_ctx: Arc<Mutex<Option<Context>>>,
    info: Arc<RwLock<ConnectionConfig>>,
}

#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub enum ModbusState {
    Connected,
    Connecting,
    #[default]
    Disconnected,
}
impl ModbusState {
    pub fn to_str(&self) -> &'static str {
        match self {
            ModbusState::Connected => "Connected",
            ModbusState::Connecting => "Connecting",
            ModbusState::Disconnected => "Disconnected",
        }
    }
}

fn squash_error<T>(mb_err: std::result::Result<std::result::Result<T, tokio_modbus::ExceptionCode>, tokio_modbus::Error>) -> Result<T> {
    match mb_err {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(e)) => Err(Error::ModbusException(e)),
        Err(e) => Err(Error::ModbusProtocolError(e)),
    }
}


impl ModbusManager{
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            shared_ctx: Arc::new(Mutex::new(None)),
            info: Arc::new(RwLock::new(config)),
        }
    }

    pub async fn cloned_config(&self) -> Result<ConnectionConfig> {
        Ok(timeout(LOCK_TIMEOUT, self.info.read())
            .await.map_err(|e| Error::LockTimedOut(e.to_string()))?.clone())
    }

    pub async fn get_connection_state(&self) -> Result<ModbusState> {
        Ok(timeout(LOCK_TIMEOUT, self.info.read())
            .await.map_err(|e| Error::LockTimedOut(e.to_string()))?.state.clone())
    }

    pub async fn set_connection_state(&self, new_state: ModbusState) -> Result<()> {
        let mut config_write_guard = self.write_config().await?;
        config_write_guard.deref_mut().state = new_state;
        Ok(())
    }

    pub async fn write_config(&self) -> Result<RwLockWriteGuard<'_, ConnectionConfig>> {
        Ok(timeout(LOCK_TIMEOUT, self.info.write())
            .await.map_err(|e| Error::LockTimedOut(e.to_string()))?)
    }

    pub async fn ctx_acquisition(&self) -> Result<MutexGuard<'_, Option<Context>>> {
        Ok(timeout(LOCK_TIMEOUT, self.shared_ctx.as_ref().lock()).await
            .map_err(|e| Error::LockTimedOut(e.to_string()))?)
    }

    pub async fn connect_ctx(connection_config: &ConnectionConfig) -> Result<Context> {
        let ctx_res = tcp::connect(connection_config.socket_addr);
        let mut ctx = timeout(connection_config.timeout_duration, ctx_res).await.map_err(|e| {
            Error::ModbusTimedOut(e.to_string())
        })??;
        ctx.set_slave(connection_config.unit_id.into());
        Ok(ctx)
    }


    pub async fn connect_new(mut connection_config: ConnectionConfig) -> Result<Self> {
        let ctx = Self::connect_ctx(&connection_config).await?;
        connection_config.state = ModbusState::Connected;
        let shared_ctx = Arc::new(Mutex::new(Some(ctx)));
        let info = Arc::new(RwLock::new(connection_config));

        Ok(Self { shared_ctx, info })
    }

    pub async fn connect(&self, mut new_config: ConnectionConfig) -> Result<()> {
        // TODO make sure we don't get stuck in connecting on an error.
        //  Other functions need to leave alone while connecting to avoid race conditions.
        let mut config_write_guard = self.write_config().await?;
        if matches!(config_write_guard.state, ModbusState::Connecting | ModbusState::Connected) {
            return Err(Error::ModbusAlreadyConnected);
        }
        config_write_guard.state = ModbusState::Connecting;
        let old_config = config_write_guard.clone();
        drop(config_write_guard);


        let mut ctx_guard = match self.ctx_acquisition().await {
            Ok(ctx_guard) => ctx_guard,
            Err(e) => {
                warn_targeted!(MODBUS, "Failed to acquire ctx lock: {e:?}");
                self.set_connection_state(ModbusState::Disconnected).await.map_err(|e|{
                    error_targeted!(MODBUS, "Failed to set state to disconnected: {e:?}");
                    return Error::FailedToUnwindConnectAttempt();
                })?;
                return Err(e);
            }
        };

        match ctx_guard.as_mut() {
            Some(ctx_guard) => {
                warn_targeted!(MODBUS, "State tag said disconnected but ctx was some");
                match ctx_guard.disconnect().await {
                    Ok(()) => {
                        info_targeted!(MODBUS, "Disconnected from {}", old_config.socket_addr);
                    }
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error disconnecting during connect: {:?}", e);
                    }
                };
            }
            None => {}
        }

        let new_ctx = match Self::connect_ctx(&new_config).await {
            Ok(new_ctx) => new_ctx,
            Err(e) => {
                warn_targeted!(MODBUS, "Failed to connect!: {e:?}");
                self.set_connection_state(ModbusState::Disconnected).await.map_err(|e|{
                    error_targeted!(MODBUS, "Failed to set state to disconnected: {e:?}");
                    return Error::FailedToUnwindConnectAttempt();
                })?;
                return Err(e);
            }
        };

        *ctx_guard = Some(new_ctx);
        drop(ctx_guard);
        info_targeted!(
            MODBUS,
            "Connected to {} as unit {}",
            new_config.socket_addr,
            new_config.unit_id
        );
        new_config.state = ModbusState::Connected;

        match self.write_config().await{
            Ok(mut config_write_guard) => {
                *config_write_guard = new_config;
            }
            Err(e) => {
                warn_targeted!(MODBUS, "Failed to write config after connect: {e:?}");
                // not exactly an unwinding error, but it conveys that connect failed and
                //  left the state tag mismatching
                return Err(Error::FailedToUnwindConnectAttempt());
            }
        }
        Ok(())
    }

    /// returns true if was connected before
    pub async fn disconnect(&self) -> Result<bool> {
        let config_handle = self.cloned_config().await?;
        let sock_addr = config_handle.socket_addr;

        let mut ctx_guard = self.ctx_acquisition().await?;
        // TODO: If this lock acquisition fails, the UI will get stuck until reload not giving
        //  an option to disconnect.

        let ctx_option = ctx_guard.take();

        let was_connected = match ctx_option {
            Some(mut ctx) => {
                match ctx.disconnect().await {
                    Ok(()) => {
                        info_targeted!(MODBUS, "Disconnected from {}", sock_addr);
                    }
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error disconnecting: {:?}", e);
                    }
                };
                true
            }
            None => { false }
        };

        // this could be placed earlier to avoid waiting for disconnect, but I don't know if this
        //  has bad implications if a `connect` is called to the same host while disconnecting.
        drop(ctx_guard);

        self.write_config().await?.state = ModbusState::Disconnected;
        Ok(was_connected)
    }

    // unused, replaced by macro. I caved
    pub async fn execute_op<F, T>(&self, op: F) -> Result<T>
    where
        F: FnOnce(&mut tokio_modbus::client::Context) ->
            std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<std::result::Result<T, tokio_modbus::ExceptionCode>, tokio_modbus::Error>> + Send + '_>> + Send,
    {
        let config = self.cloned_config().await?;
        if !matches!(config.state, ModbusState::Connected) {
            return Err(Error::NotConnected);
        }
        let mut ctx_guard = self.ctx_acquisition().await?;
        match ctx_guard.as_mut() {
            Some(ctx) => {
                let fut = timeout(config.timeout_duration, op(ctx));
                let resolved = fut.await.map_err(|e| Error::ModbusTimedOut(e.to_string()))?;
                squash_error(resolved)
            }
            None => Err(Error::ModbusInvariantBroken),
        }
    }



    pub async fn read_coils(&self, payload: ReadCoilsRequest) -> Result<ReadCoilsResponse> {
        let coils = execute_modbus!(self, |ctx| {
            ctx.read_coils(payload.address, payload.count)
        })?;
        Ok(ReadCoilsResponse { values: coils })
    }

    pub async fn read_discrete_inputs(&self, payload: ReadDiscreteInputsRequest) -> Result<ReadDiscreteInputsResponse> {
        let discs = execute_modbus!(self, |ctx| {
            ctx.read_discrete_inputs(payload.address, payload.count)
        })?;
        Ok(ReadDiscreteInputsResponse { values: discs })
    }

    pub async fn read_holding_registers(&self, payload: ReadHoldingRegistersRequest) -> Result<ReadHoldingRegistersResponse> {
        let hregs = execute_modbus!(self, |ctx| {
            ctx.read_holding_registers(payload.address, payload.count)
        })?;
        Ok(ReadHoldingRegistersResponse { values: hregs })
    }

    pub async fn read_input_registers(&self, payload: ReadInputRegistersRequest) -> Result<ReadInputRegistersResponse> {
        let iregs = execute_modbus!(self, |ctx| {
            ctx.read_input_registers(payload.address, payload.count)
        })?;
        Ok(ReadInputRegistersResponse { values: iregs })
    }

    pub async fn write_single_coil(&self, payload: WriteSingleCoilRequest) -> Result<()> {
        execute_modbus!(self, |ctx| {
            ctx.write_single_coil(payload.address, payload.value)
        })?;
        Ok(())
    }

    pub async fn write_single_register(&self, payload: WriteSingleRegisterRequest) -> Result<()> {
        execute_modbus!(self, |ctx| {
            ctx.write_single_register(payload.address, payload.value)
        })?;
        Ok(())
    }

    pub async fn write_multiple_coils(&self, payload: WriteMultipleCoilsRequest) -> Result<()> {
        execute_modbus!(self, |ctx| {
            ctx.write_multiple_coils(payload.address, &payload.values)
        })?;
        Ok(())
    }

    pub async fn write_multiple_registers(&self, payload: WriteMultipleRegistersRequest) -> Result<()> {
        execute_modbus!(self, |ctx| {
            ctx.write_multiple_registers(payload.address, &payload.values)
        })?;
        Ok(())
    }
}


