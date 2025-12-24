use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::Deserialize;
use std::net::SocketAddr;
use askama::Template;
use crate::{debug_targeted, info_targeted, warn_targeted, error_targeted};

use crate::modbus::modbus_transaction_types::*;
use crate::modbus::{ConnectionConfig, ModbusManager, ModbusState};

// --- State Management ---

#[derive(Clone)]
pub struct AppState {
    pub clearcore_modbus: ModbusManager,
    pub welder_modbus: ModbusManager,
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
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    pub timeout_ms: u64,
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
    unit_id: u8,
    timeout_ms: Option<u32>,
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

    let (connected, host, port, unit_id, timeout_ms) = match state.modbus_manager.cloned_config().await {
        Ok(config) => (
            matches!(config.state, ModbusState::Connected),
            config.socket_addr.ip().to_string(),
            config.socket_addr.port(),
            config.unit_id,
            config.timeout_duration.as_millis() as u64
        ),
        Err(e) => {
            error_targeted!(MODBUS, "Failed to retrieve modbus config: {:?}", e);
            (false, "127.0.0.1".to_string(), 502, 1, 1000)
        }
    };

    debug_targeted!(MODBUS, "Connection state: connected={}, host={}", connected, host);

    let template = ConnectionTemplate {
        connected,
        host,
        port,
        unit_id,
        timeout_ms,
        error: None,
    };
    Html(template.render().unwrap())
}

// GET /modbus/status - Returns the icon, checks actual connectivity
pub async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "GET /modbus/status - checking connection status");

    let is_online = match state.modbus_manager.get_connection_state().await {
        Ok(ModbusState::Connected) => true,
        _ => false,
    };

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
    let timeout_val = form.timeout_ms.unwrap_or(1000) as u64;

    match new_addr {
        Ok(addr) => {
            info_targeted!(MODBUS, "Successfully parsed address: {}", addr);

            let config = match form.timeout_ms {
                Some(ms) => {
                    let timeout_duration =  std::time::Duration::from_millis(ms as u64);
                    ConnectionConfig::new_with_timeout(addr, form.unit_id, timeout_duration) }
                None => ConnectionConfig::new(addr, form.unit_id)
            };

            match state.modbus_manager.connect(config).await {
                Ok(_) => {
                    let template = ConnectionTemplate {
                        connected: true,
                        host: form.host,
                        port: form.port,
                        unit_id: form.unit_id,
                        timeout_ms: timeout_val,
                        error: None,
                    };
                    Html(template.render().unwrap())
                }
                Err(e) => {
                    error_targeted!(MODBUS, "Connection failed: {:?}", e);
                    let template = ConnectionTemplate {
                        connected: false,
                        host: form.host,
                        port: form.port,
                        unit_id: form.unit_id,
                        timeout_ms: timeout_val,
                        error: Some(format!("Connection Failed: {:?}", e)),
                    };
                    Html(template.render().unwrap())
                }
            }
        }
        Err(e) => {
            error_targeted!(MODBUS, "Failed to parse address '{}': {:?}", addr_str, e);
            let template = ConnectionTemplate {
                connected: false,
                host: form.host,
                port: form.port,
                unit_id: form.unit_id,
                timeout_ms: timeout_val,
                error: Some("Invalid IP address or Port".to_string()),
            };
            Html(template.render().unwrap())
        }
    }
}

// POST /modbus/disconnect
pub async fn disconnect_modbus(State(state): State<AppState>) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /modbus/disconnect - disconnecting from Modbus");

    // Capture current config before disconnect to repopulate form
    let (host, port, unit_id, timeout_ms) = match state.modbus_manager.cloned_config().await {
        Ok(config) => (
            config.socket_addr.ip().to_string(),
            config.socket_addr.port(),
            config.unit_id,
            config.timeout_duration.as_millis() as u64
        ),
        Err(_) => ("127.0.0.1".to_string(), 502, 1, 1000),
    };

    if let Err(e) = state.modbus_manager.disconnect().await {
        warn_targeted!(MODBUS, "Error during disconnect: {:?}", e);
    }

    let template = ConnectionTemplate {
        connected: false,
        host,
        port,
        unit_id,
        timeout_ms,
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

    let request = ReadHoldingRegistersRequest {
        address: form.address,
        count: form.count,
    };

    match state.modbus_manager.read_holding_registers(request).await {
        Ok(response) => {
            info_targeted!(MODBUS, "Successfully read {} registers", response.values.len());
            let registers = response.values
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
            Html(format!("<p style='color: red;'>Modbus Error: {:?}</p>", e)).into_response()
        }
    }
}

// POST /write
pub async fn write_register(
    State(state): State<AppState>,
    Form(form): Form<WriteForm>,
) -> impl IntoResponse {
    info_targeted!(HTTP, "POST /write - address: {}, value: {}", form.address, form.value);

    let request = WriteSingleRegisterRequest {
        address: form.address,
        value: form.value,
    };

    match state.modbus_manager.write_single_register(request).await {
        Ok(_) => {
            info_targeted!(MODBUS, "Successfully wrote {} to register {}", form.value, form.address);
            Html(format!(
                "<p class='write-response green'>✓ Successfully wrote {} to register {}</p>",
                form.value, form.address
            )).into_response()
        }
        Err(e) => {
            error_targeted!(MODBUS, "Modbus write error: {:?}", e);
            Html(format!("<p class='write-response red'>Error: {:?}</p>", e)).into_response()
        }
    }
}