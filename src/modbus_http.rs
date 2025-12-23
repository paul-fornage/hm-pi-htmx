use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio_modbus::prelude::*;
use askama::Template;
use crate::{debug_targeted, trace_targeted, info_targeted, warn_targeted, error_targeted};

// --- State Management ---

#[derive(Clone, Default)]
pub struct AppState {
    // We use RwLock so we can change the address at runtime
    pub config: Arc<RwLock<ModbusConfig>>,
}

#[derive(Clone, Default)]
pub struct ModbusConfig {
    pub address: Option<SocketAddr>,
}

// --- Templates ---

#[derive(Template)]
#[template(path = "components/hreg.html")]
pub struct RegistersTemplate {
    pub start_address: u16,
    pub registers: Vec<(u16, u16)>,
}

#[derive(Template)]
#[template(path = "components/connection.html")]
pub struct ConnectionTemplate {
    pub connected: bool,
    pub current_host: String,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "components/connection-status.html")]
pub struct StatusTemplate {
    pub connected: bool,
}

// --- Forms ---

#[derive(Deserialize)]
pub struct ConnectForm {
    host: String,
    port: u16,
}

#[derive(Deserialize)]
pub struct ReadForm {
    address: u16,
    count: u16,
}

#[derive(Deserialize)]
pub struct WriteForm {
    address: u16,
    value: u16,
}

// --- Handlers ---

// GET /modbus/manager - Renders the initial connection box
pub async fn get_connection_manager(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "GET /modbus/manager - rendering connection manager");
    let config = state.config.read().unwrap();

    let (connected, host) = match config.address {
        Some(addr) => (true, addr.to_string()),
        None => (false, String::new()),
    };

    debug_targeted!(MODBUS, "Connection state: connected={}, host={}", connected, host);

    let template = ConnectionTemplate {
        connected,
        current_host: host,
        error: None,
    };
    Html(template.render().unwrap())
}

// GET /modbus/status - Returns the icon, checks actual connectivity
pub async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "GET /modbus/status - checking connection status");
    let addr = {
        let config = state.config.read().unwrap();
        config.address
    };

    let mut is_online = false;

    // Attempt a quick connection to verify status
    if let Some(socket_addr) = addr {
        debug_targeted!(MODBUS, "Attempting to connect to {} for status check", socket_addr);
        // Create a context just to check connection
        match tcp::connect(socket_addr).await {
            Ok(mut ctx) => {
                // Try a dummy read of 1 register to ensure device is responsive
                match ctx.read_holding_registers(0, 1).await {
                    Ok(_) => {
                        is_online = true;
                        debug_targeted!(MODBUS, "Status check successful - device is online");
                    }
                    Err(e) => {
                        warn_targeted!(MODBUS, "Status check failed during register read: {:?}", e);
                    }
                }
                if let Err(e) = ctx.disconnect().await {
                    warn_targeted!(MODBUS, "Error disconnecting during status check: {:?}", e);
                }
            }
            Err(e) => {
                warn_targeted!(MODBUS, "Status check failed to connect: {:?}", e);
            }
        }
    } else {
        debug_targeted!(MODBUS, "No address configured for status check");
    }

    debug_targeted!(HTTP, "Status result: online={}", is_online);
    let template = StatusTemplate { connected: is_online };
    Html(template.render().unwrap())
}

// POST /modbus/connect
pub async fn connect_modbus(
    State(state): State<AppState>,
    Form(form): Form<ConnectForm>,
) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /modbus/connect - host: {}, port: {}", form.host, form.port);
    let addr_str = format!("{}:{}", form.host, form.port);

    let new_addr: Result<SocketAddr, _> = addr_str.parse();

    match new_addr {
        Ok(addr) => {
            info_targeted!(MODBUS, "Successfully parsed address: {}", addr);
            // Update state
            let mut config = state.config.write().unwrap();
            config.address = Some(addr);

            let template = ConnectionTemplate {
                connected: true,
                current_host: addr.to_string(),
                error: None,
            };
            Html(template.render().unwrap())
        }
        Err(e) => {
            error_targeted!(MODBUS, "Failed to parse address '{}': {:?}", addr_str, e);
            let template = ConnectionTemplate {
                connected: false,
                current_host: String::new(),
                error: Some("Invalid IP address or Port".to_string()),
            };
            Html(template.render().unwrap())
        }
    }
}

// POST /modbus/disconnect
pub async fn disconnect_modbus(State(state): State<AppState>) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /modbus/disconnect - disconnecting from Modbus");
    let mut config = state.config.write().unwrap();
    config.address = None;

    let template = ConnectionTemplate {
        connected: false,
        current_host: String::new(),
        error: None,
    };
    Html(template.render().unwrap())
}

// POST /read
pub async fn read_registers(
    State(state): State<AppState>,
    Form(form): Form<ReadForm>,
) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /read - address: {}, count: {}", form.address, form.count);
    let addr = {
        let config = state.config.read().unwrap();
        config.address
    };

    match addr {
        Some(socket_addr) => {
            debug_targeted!(MODBUS, "Reading from {}", socket_addr);
            match read_holding_registers_helper(socket_addr, form.address, form.count).await {
                Ok(values) => {
                    info_targeted!(MODBUS, "Successfully read {} registers", values.len());
                    let registers = values
                        .into_iter()
                        .enumerate()
                        .map(|(i, val)| (form.address + i as u16, val))
                        .collect();

                    let template = RegistersTemplate {
                        start_address: form.address,
                        registers,
                    };

                    match template.render() {
                        Ok(html) => Html(html).into_response(),
                        Err(e) => {
                            error_targeted!(HTTP, "Template render error: {:?}", e);
                            (StatusCode::INTERNAL_SERVER_ERROR, "Template Error").into_response()
                        }
                    }
                }
                Err(e) => {
                    error_targeted!(MODBUS, "Modbus read error: {:?}", e);
                    Html(format!("<p style='color: red;'>Modbus Error: {}</p>", e)).into_response()
                }
            }
        }
        None => {
            warn_targeted!(MODBUS, "Read attempted but not connected");
            Html("<p style='color: red;'>Not Connected. Please set host/port.</p>".to_string()).into_response()
        }
    }
}

// POST /write
pub async fn write_register(
    State(state): State<AppState>,
    Form(form): Form<WriteForm>,
) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /write - address: {}, value: {}", form.address, form.value);
    let addr = {
        let config = state.config.read().unwrap();
        config.address
    };

    match addr {
        Some(socket_addr) => {
            debug_targeted!(MODBUS, "Writing to {}", socket_addr);
            match write_single_register_helper(socket_addr, form.address, form.value).await {
                Ok(_) => {
                    info_targeted!(MODBUS, "Successfully wrote {} to register {}", form.value, form.address);
                    Html(format!(
                        "<p class='write-response green'>✓ Successfully wrote {} to register {}</p>",
                        form.value, form.address
                    )).into_response()
                }
                Err(e) => {
                    error_targeted!(MODBUS, "Modbus write error: {:?}", e);
                    Html(format!("<p class='write-response red'>Error: {}</p>", e)).into_response()
                }
            }
        }
        None => {
            warn_targeted!(MODBUS, "Write attempted but not connected");
            Html("<p style='color: red;'>Not Connected</p>".to_string()).into_response()
        }
    }
}

// --- Helpers ---

async fn read_holding_registers_helper(
    addr: SocketAddr,
    address: u16,
    count: u16,
) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    debug_targeted!(MODBUS, "Connecting to {} for read operation", addr);
    let mut ctx = tcp::connect(addr).await?;
    debug_targeted!(MODBUS, "Connected, reading {} registers starting at address {}", count, address);
    let data = ctx.read_holding_registers(address, count).await??;
    debug_targeted!(MODBUS, "Read successful, disconnecting");
    ctx.disconnect().await?;
    Ok(data)
}

async fn write_single_register_helper(
    addr: SocketAddr,
    address: u16,
    value: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    debug_targeted!(MODBUS, "Connecting to {} for write operation", addr);
    let mut ctx = tcp::connect(addr).await?;
    debug_targeted!(MODBUS, "Connected, writing value {} to register {}", value, address);
    ctx.write_single_register(address, value).await??;
    debug_targeted!(MODBUS, "Write successful, disconnecting");
    ctx.disconnect().await?;
    Ok(())
}