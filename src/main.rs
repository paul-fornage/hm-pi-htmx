mod modbus;
mod logging;

mod modbus_http;
pub mod error;


use askama::Template;
use axum::{
    routing::{get, post},
    response::{IntoResponse, Html, Response},
    http::StatusCode,
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

    // Initialize Modbus Manager
    // We create a dummy config first. ConnectionConfig::new starts in Disconnected state,
    // so the IP doesn't matter yet, but we need a valid SocketAddr.
    let dummy_addr: std::net::SocketAddr = "192.168.1.68:502".parse().unwrap();
    let initial_config = modbus::ConnectionConfig::new(dummy_addr, 1);
    let modbus_manager = modbus::ModbusManager::new(initial_config);

    // Initialize state with the manager
    let state = AppState {
        modbus_manager
    };

    debug_targeted!(HTTP, "Initialized application state");

    let app = Router::new()
        // --- View Routes ---
        .route("/", get(show_operations))
        .route("/connections", get(show_connections))

        // --- Modbus Management Routes ---
        .route("/modbus/manager", get(modbus_http::get_connection_manager))
        .route("/modbus/status", get(modbus_http::get_status))
        .route("/modbus/connect", post(modbus_http::connect_modbus))
        .route("/modbus/disconnect", post(modbus_http::disconnect_modbus))

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