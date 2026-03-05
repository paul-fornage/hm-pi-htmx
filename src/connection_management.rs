use axum::{
    extract::{State, Form},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use askama::Template;
use crate::{debug_targeted, info_targeted, warn_targeted, error_targeted, AppState, trace_targeted};
use crate::file_io::NamedDiskFile;
use crate::modbus::{ConnectionConfig, ModbusManager, ModbusState};

#[derive(Template)]
#[template(path = "components/connection.html")]
pub struct ConnectionTemplate {
    pub name: String,
    pub connected: bool,
    pub auto_connect: bool,
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    pub timeout_ms: u64,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "components/connection-status.html")]
pub struct StatusTemplate {
    pub clearcore_state: String,
    pub welder_state: String,
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
pub struct AutoConnectForm {
    enabled: bool,
}



// Generic function to get connection manager template for a given ModbusManager
async fn get_connection_template(
    manager: &ModbusManager,
    name: &str,
    auto_connect: bool,
) -> ConnectionTemplate {
    let (connected, host, port, unit_id, timeout_ms) = match manager.cloned_config().await {
        Ok(config) => (
            matches!(config.state, ModbusState::Connected),
            config.socket_addr.ip().to_string(),
            config.socket_addr.port(),
            config.unit_id,
            config.timeout_duration.as_millis() as u64
        ),
        Err(e) => {
            error_targeted!(MODBUS, "Failed to retrieve modbus config for {}: {:?}", name, e);
            (false, "127.0.0.1".to_string(), 502, 1, 1000)
        }
    };

    trace_targeted!(MODBUS, "{} connection state: connected={}, host={}", name, connected, host);

    ConnectionTemplate {
        name: name.to_string(),
        connected,
        auto_connect,
        host,
        port,
        unit_id,
        timeout_ms,
        error: None,
    }
}

// Generic function to handle connection
async fn handle_connect(
    manager: &ModbusManager,
    name: &str,
    config_name: &str,
    form: ConnectForm,
    auto_connect: bool,
) -> ConnectionTemplate {
    info_targeted!(HTTP, "POST /modbus/{}/connect - host: {}, port: {}", name, form.host, form.port);
    let addr_str = format!("{}:{}", form.host, form.port);

    let new_addr: Result<SocketAddr, _> = addr_str.parse();
    let timeout_val = form.timeout_ms.unwrap_or(1000) as u64;

    match new_addr {
        Ok(addr) => {
            info_targeted!(MODBUS, "Successfully parsed address for {}: {}", name, addr);

            let config = match form.timeout_ms {
                Some(ms) => {
                    let timeout_duration = std::time::Duration::from_millis(ms as u64);
                    ConnectionConfig::new_with_timeout(addr, form.unit_id, timeout_duration)
                }
                None => ConnectionConfig::new(addr, form.unit_id)
            };

            // Save config before attempting connection
            if let Err(e) = ConnectionConfig::save(config_name, &config).await {
                warn_targeted!(MODBUS, "Failed to save {} config: {}", name, e);
            }

            match manager.connect(config).await {
                Ok(_) => {
                    ConnectionTemplate {
                        name: name.to_string(),
                        connected: true,
                        auto_connect,
                        host: form.host,
                        port: form.port,
                        unit_id: form.unit_id,
                        timeout_ms: timeout_val,
                        error: None,
                    }
                }
                Err(e) => {
                    error_targeted!(MODBUS, "{} connection failed: {:?}", name, e);
                    ConnectionTemplate {
                        name: name.to_string(),
                        connected: false,
                        auto_connect,
                        host: form.host,
                        port: form.port,
                        unit_id: form.unit_id,
                        timeout_ms: timeout_val,
                        error: Some(format!("Connection Failed: {:?}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error_targeted!(MODBUS, "Failed to parse address '{}' for {}: {:?}", addr_str, name, e);
            ConnectionTemplate {
                name: name.to_string(),
                connected: false,
                auto_connect,
                host: form.host,
                port: form.port,
                unit_id: form.unit_id,
                timeout_ms: timeout_val,
                error: Some("Invalid IP address or Port".to_string()),
            }
        }
    }
}

// Generic function to handle disconnection
async fn handle_disconnect(
    manager: &ModbusManager,
    name: &str,
    auto_connect: bool,
) -> ConnectionTemplate {
    info_targeted!(HTTP, "POST /modbus/{}/disconnect - disconnecting", name);

    // Capture current config before disconnect to repopulate form
    let (host, port, unit_id, timeout_ms) = match manager.cloned_config().await {
        Ok(config) => (
            config.socket_addr.ip().to_string(),
            config.socket_addr.port(),
            config.unit_id,
            config.timeout_duration.as_millis() as u64
        ),
        Err(_) => ("127.0.0.1".to_string(), 502, 1, 1000),
    };

    if let Err(e) = manager.disconnect().await {
        warn_targeted!(MODBUS, "Error during {} disconnect: {:?}", name, e);
    }

    ConnectionTemplate {
        name: name.to_string(),
        connected: false,
        auto_connect,
        host,
        port,
        unit_id,
        timeout_ms,
        error: None,
    }
}

fn render_connection_template(template: ConnectionTemplate, name: &str) -> impl IntoResponse {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            error_targeted!(HTTP, "Failed to render connection template for {}: {:?}", name, e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

// GET /modbus/manager - Renders the initial connection box
pub async fn get_connection_manager(State(state): State<AppState>) -> impl IntoResponse {
    trace_targeted!(HTTP, "GET /modbus/manager - rendering connection manager");
    let auto_connect = state.clearcore_auto_connect.load(Ordering::Acquire);
    let template = get_connection_template(&state.clearcore_registers.manager, "clearcore", auto_connect).await;
    render_connection_template(template, "clearcore")
}

// GET /modbus/status - Returns the icon, checks actual connectivity
pub async fn get_status(State(state): State<AppState>) -> impl IntoResponse {
    trace_targeted!(HTTP, "GET /modbus/status - checking connection status");

    let clearcore_state = match state.clearcore_registers.manager.get_connection_state().await {
        Ok(state) => state.to_str(),
        Err(_) => "Error",
    };

    let welder_state: &'static str = match state.miller_registers.get_connection_state().await {
        Ok(state) => state.to_str(),
        Err(_) => "Error",
    };

    trace_targeted!(HTTP, "Status result: clearcore={}, welder={}", clearcore_state, welder_state);
    let template = StatusTemplate {
        clearcore_state: clearcore_state.to_string(),
        welder_state: welder_state.to_string(),
    };
    Html(template.render().unwrap())
}

// POST /modbus/clearcore/connect
pub async fn connect_clearcore(
    State(state): State<AppState>,
    Form(form): Form<ConnectForm>,
) -> impl IntoResponse {
    let auto_connect = state.clearcore_auto_connect.load(Ordering::Acquire);
    let template = handle_connect(
        &state.clearcore_registers.manager,
        "clearcore",
        crate::modbus::CLEARCORE_CONFIG_NAME,
        form,
        auto_connect,
    ).await;
    render_connection_template(template, "clearcore")
}

// POST /modbus/clearcore/disconnect
pub async fn disconnect_clearcore(State(state): State<AppState>) -> impl IntoResponse {
    let auto_connect = state.clearcore_auto_connect.load(Ordering::Acquire);
    let template = handle_disconnect(&state.clearcore_registers.manager, "clearcore", auto_connect).await;
    render_connection_template(template, "clearcore")
}

// GET /modbus/clearcore/manager
pub async fn get_clearcore_manager(State(state): State<AppState>) -> impl IntoResponse {
    trace_targeted!(HTTP, "GET /modbus/clearcore/manager - rendering clearcore connection manager");
    let auto_connect = state.clearcore_auto_connect.load(Ordering::Acquire);
    let template = get_connection_template(&state.clearcore_registers.manager, "clearcore", auto_connect).await;
    render_connection_template(template, "clearcore")
}

// POST /modbus/welder/connect
pub async fn connect_welder(
    State(state): State<AppState>,
    Form(form): Form<ConnectForm>,
) -> impl IntoResponse {
    let auto_connect = state.welder_auto_connect.load(Ordering::Acquire);
    let template = handle_connect(
        &state.miller_registers.manager,
        "welder",
        crate::modbus::WELDER_CONFIG_NAME,
        form,
        auto_connect,
    ).await;
    render_connection_template(template, "welder")
}

// POST /modbus/welder/disconnect
pub async fn disconnect_welder(State(state): State<AppState>) -> impl IntoResponse {
    let auto_connect = state.welder_auto_connect.load(Ordering::Acquire);
    let template = handle_disconnect(&state.miller_registers.manager, "welder", auto_connect).await;
    render_connection_template(template, "welder")
}

// GET /modbus/welder/manager
pub async fn get_welder_manager(State(state): State<AppState>) -> impl IntoResponse {
    trace_targeted!(HTTP, "GET /modbus/welder/manager - rendering welder connection manager");
    let auto_connect = state.welder_auto_connect.load(Ordering::Acquire);
    let template = get_connection_template(&state.miller_registers.manager, "welder", auto_connect).await;
    render_connection_template(template, "welder")
}

// POST /modbus/clearcore/auto-connect
pub async fn set_clearcore_auto_connect(
    State(state): State<AppState>,
    Form(form): Form<AutoConnectForm>,
) -> impl IntoResponse {
    info_targeted!(MODBUS, "Clearcore auto-connect set to {}", form.enabled);
    state.clearcore_auto_connect.store(form.enabled, Ordering::Release);
    let template = get_connection_template(
        &state.clearcore_registers.manager,
        "clearcore",
        form.enabled,
    ).await;
    render_connection_template(template, "clearcore")
}

// POST /modbus/welder/auto-connect
pub async fn set_welder_auto_connect(
    State(state): State<AppState>,
    Form(form): Form<AutoConnectForm>,
) -> impl IntoResponse {
    info_targeted!(MODBUS, "Welder auto-connect set to {}", form.enabled);
    state.welder_auto_connect.store(form.enabled, Ordering::Release);
    let template = get_connection_template(
        &state.miller_registers.manager,
        "welder",
        form.enabled,
    ).await;
    render_connection_template(template, "welder")
}
