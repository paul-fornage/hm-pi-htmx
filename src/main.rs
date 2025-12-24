mod modbus;
mod logging;

mod modbus_http;
pub mod error;
mod clearcore_registers;
mod miller;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::logging::LogTarget;
use crate::modbus_http::AppState;

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            ).into_response(),
        }
    }
}

// Template for the Operations (Home) View
#[derive(Template)]
#[template(path = "views/raw-reg-viewer.html")]
struct OperationsTemplate {}

impl OperationsTemplate {
    fn tab_name_as_str(&self) -> &'static str {
        "operations"
    }
}

// Template for the Connections View
#[derive(Template)]
#[template(path = "views/connection-manager.html")]
struct ConnectionsTemplate {}

impl ConnectionsTemplate {
    fn tab_name_as_str(&self) -> &'static str {
        "connections"
    }
}

async fn show_operations() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering operations view");
    HtmlTemplate(OperationsTemplate {})
}

async fn show_connections() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering connections view");
    HtmlTemplate(ConnectionsTemplate {})
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
        .route("/", get(show_operations))
        .route("/connections", get(show_connections))

        // --- Modbus Management Routes - ClearCore ---
        .route("/modbus/clearcore/manager", get(modbus_http::get_clearcore_manager))
        .route("/modbus/clearcore/connect", post(modbus_http::connect_clearcore))
        .route("/modbus/clearcore/disconnect", post(modbus_http::disconnect_clearcore))

        // --- Modbus Management Routes - Welder ---
        .route("/modbus/welder/manager", get(modbus_http::get_welder_manager))
        .route("/modbus/welder/connect", post(modbus_http::connect_welder))
        .route("/modbus/welder/disconnect", post(modbus_http::disconnect_welder))

        // --- Legacy Routes (kept for compatibility) ---
        .route("/modbus/manager", get(modbus_http::get_connection_manager))
        .route("/modbus/status", get(modbus_http::get_status))

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