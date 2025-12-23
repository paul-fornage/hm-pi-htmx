use tokio_modbus::client::tcp;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard, RwLock, RwLockWriteGuard};
use tokio::time::timeout;
use tokio_modbus::client::{Client, Context, Reader, Writer};
use tokio_modbus::{ExceptionCode, Slave};
use log::{error, info, warn};
use crate::error::{Error, Result};
use crate::modbus_http::ModbusConfig;
use crate::{info_targeted, warn_targeted};


const LOCK_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub state: ModbusState,
    pub socket_addr: SocketAddr,
    pub unit_id: u8,
    pub timeout_duration: Duration,
}
impl ConnectionConfig {
    pub async fn new_with_timeout(socket_addr: SocketAddr, unit_id: u8, timeout_duration: Duration) -> Self {
        ConnectionConfig{
            state: ModbusState::Disconnected,
            socket_addr,
            unit_id,
            timeout_duration,
        }
    }
    pub async fn new(socket_addr: SocketAddr, unit_id: u8) -> Self {
        ConnectionConfig{
            state: ModbusState::Disconnected,
            socket_addr,
            unit_id,
            timeout_duration: Duration::from_millis(300),
        }
    }
}

struct ModbusManager {
    shared_ctx: Option<Arc<Mutex<Context>>>,
    info: Arc<RwLock<ConnectionConfig>>,
}

#[derive(Clone, Debug)]
enum ModbusState {
    Connected,
    Connecting,
    Disconnected,
}

fn squash_error<T>(mb_err: std::result::Result<std::result::Result<T, ExceptionCode>, Error>) -> Result<T> {
    match mb_err {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(e)) => Err(Error::ModbusException(e.to_string())),
        Err(e) => Err(e),
    }
}


impl ModbusManager{
    pub fn new(config: ConnectionConfig) -> Self {
        Self {
            shared_ctx: None,
            info: Arc::new(RwLock::new(config)),
        }
    }

    pub async fn cloned_config(&self) -> Result<ConnectionConfig> {
        Ok(timeout(LOCK_TIMEOUT, self.info.read())
            .await.map_err(|e| Error::LockTimedOut(e.to_string()))?.clone())
    }

    pub async fn write_config(&mut self) -> Result<RwLockWriteGuard<'_, ConnectionConfig>> {
        Ok(timeout(LOCK_TIMEOUT, self.info.write())
            .await.map_err(|e| Error::LockTimedOut(e.to_string()))?)
    }

    pub async fn connect_new(mut connection_config: ConnectionConfig) -> Result<Self> {
        let timeout_duration = connection_config.timeout_duration;
        let ctx_res = tcp::connect(connection_config.socket_addr);
        connection_config.state = ModbusState::Disconnected; // doesn't matter, no one else has this
        let ctx = timeout(timeout_duration, ctx_res).await.map_err(|e| {
            Error::ModbusTimedOut(e.to_string())
        })??;
        connection_config.state = ModbusState::Connected;
        let shared_ctx = Some(Arc::new(Mutex::new(ctx)));
        let info = Arc::new(RwLock::new(connection_config));

        Ok(Self { shared_ctx, info })
    }

    pub async fn connect(&mut self) -> Result<Self> {
        let config = self.write_config().await?;
        
        config.state = ModbusState::Connecting;
        let sock_addr = config.socket_addr.clone();
        let timeout_duration = config.timeout_duration.clone();
        std::mem::drop(config);

        match self.shared_ctx.clone() {
            Some(ctx_mutex) => {
                let mut ctx = timeout(timeout_duration, ctx_mutex.lock())
                    .await.map_err(|e| { Error::ModbusTimedOut(e.to_string()) })?;
                match(ctx.disconnect().await){
                    Ok(()) => {}
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error disconnecting during connect: {:?}", e);
                    }
                };
            }
            None => {

            }
        }


        maybe_ctx.

        let shared_ctx = Arc::new(Mutex::new(Some(ctx)));
        let info = Arc::new(RwLock::new(connection_config));

        Ok(Self { shared_ctx, info })
    }

    /// returns true if was connected before
    pub async fn disconnect(&mut self) -> Result<bool> {
        let config_handle = self.cloned_config().await?;
        let timeout_duration = config_handle.timeout_duration;
        let sock_addr = config_handle.socket_addr;
        drop(config_handle);
        let was_connected = match self.shared_ctx.clone() {
            Some(ctx_mutex) => {
                let mut ctx = timeout(timeout_duration, ctx_mutex.lock())
                    .await.map_err(|e| { Error::ModbusTimedOut(e.to_string()) })?;
                match ctx.disconnect().await {
                    Ok(()) => {
                        info_targeted!(MODBUS, "Disconnected from {}", sock_addr);
                    }
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error disconnecting during connect: {:?}", e);
                    }
                };
                true
            }
            None => { false }
        };
        self.write_config().await?.state = ModbusState::Disconnected;
        self.shared_ctx = None;
        Ok(was_connected)
    }
}