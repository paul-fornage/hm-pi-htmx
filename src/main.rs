mod modbus;
mod logging;

mod modbus_http;
pub mod error;
mod clearcore_registers;
mod miller;
mod connection_management;
mod views;

use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::logging::LogTarget;
use crate::modbus::ModbusManager;
use crate::views::{AppView, ConnectionsTemplate, MillerInfoTemplate, OperationsTemplate};

#[derive(Clone)]
pub struct AppState {
    pub clearcore_modbus: ModbusManager,
    pub welder_modbus: ModbusManager,
}

pub const OPERATIONS_TEMPLATE: OperationsTemplate = OperationsTemplate{};
pub const CONNECTIONS_TEMPLATE: ConnectionsTemplate = ConnectionsTemplate{};
pub const MILLER_INFO_TEMPLATE: MillerInfoTemplate = MillerInfoTemplate{};


async fn show_operations() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering operations view");
    OPERATIONS_TEMPLATE
}

async fn show_connections() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering connections view");
    CONNECTIONS_TEMPLATE
}

async fn show_miller_info() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering miller info view");
    MILLER_INFO_TEMPLATE
}


#[tokio::main]
async fn main() {

    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        builder.filter_level(log::LevelFilter::Debug);
        builder.filter(Some(LogTarget::MODBUS.into()), log::LevelFilter::Debug);
        builder.filter(Some(LogTarget::HTTP.into()), log::LevelFilter::Debug);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    info_targeted!(HTTP, "Starting Modbus HTMX application");

    // Initialize Modbus Managers - load from saved config or use defaults
    let clearcore_config = modbus::ConnectionConfig::load_from_path(modbus::CLEARCORE_CONFIG_PATH)
        .unwrap_or_else(|| {
            let addr: std::net::SocketAddr = "192.168.1.68:502".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let clearcore_modbus = modbus::ModbusManager::new(clearcore_config.clone());

    let welder_config = modbus::ConnectionConfig::load_from_path(modbus::WELDER_CONFIG_PATH)
        .unwrap_or_else(|| {
            let addr: std::net::SocketAddr = "192.168.1.104:50205".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let welder_modbus = modbus::ModbusManager::new(welder_config.clone());

    // Attempt to connect on startup
    tokio::spawn({
        let clearcore = clearcore_modbus.clone();
        async move {
            if let Err(e) = clearcore.connect(clearcore_config).await {
                info_targeted!(MODBUS, "Clearcore auto-connect failed: {:?}", e);
            }
        }
    });

    tokio::spawn({
        let welder = welder_modbus.clone();
        async move {
            if let Err(e) = welder.connect(welder_config).await {
                info_targeted!(MODBUS, "Welder auto-connect failed: {:?}", e);
            }
        }
    });

    // Initialize state with the managers
    let state = AppState {
        clearcore_modbus,
        welder_modbus,
    };

    debug_targeted!(HTTP, "Initialized application state");

    let app = Router::new()
        // --- View Routes ---
        .route(AppView::Operations.url(), get(show_operations))
        .route(AppView::Connections.url(), get(show_connections))
        .route(AppView::MillerInfo.url(), get(show_miller_info))

        // --- Modbus Management Routes - ClearCore ---
        .route("/modbus/clearcore/manager", get(connection_management::get_clearcore_manager))
        .route("/modbus/clearcore/connect", post(connection_management::connect_clearcore))
        .route("/modbus/clearcore/disconnect", post(connection_management::disconnect_clearcore))

        // --- Modbus Management Routes - Welder ---
        .route("/modbus/welder/manager", get(connection_management::get_welder_manager))
        .route("/modbus/welder/connect", post(connection_management::connect_welder))
        .route("/modbus/welder/disconnect", post(connection_management::disconnect_welder))

        .route("/modbus/manager", get(connection_management::get_connection_manager))
        .route("/modbus/status", get(connection_management::get_status))

        // --- Operation Routes ---
        .route("/read", post(modbus_http::read_registers))
        .route("/write", post(modbus_http::write_register))

        // --- Static files ---
        .fallback_service(ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("static/assets"))

        // Apply state
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info_targeted!(HTTP, "Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}