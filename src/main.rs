mod modbus;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::modbus::AppState;

#[tokio::main]
async fn main() {
    // Initialize state (defaults to None/Disconnected)
    let state = AppState::default();

    let app = Router::new()
        // Modbus Management Routes
        .route("/modbus/manager", get(modbus::get_connection_manager))
        .route("/modbus/status", get(modbus::get_status))
        .route("/modbus/connect", post(modbus::connect_modbus))
        .route("/modbus/disconnect", post(modbus::disconnect_modbus))

        // Operation Routes
        .route("/read", post(modbus::read_registers))
        .route("/write", post(modbus::write_register))

        // Static files
        .fallback_service(ServeDir::new("static"))

        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}