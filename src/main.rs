use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio_modbus::prelude::*;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    modbus_addr: SocketAddr,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        modbus_addr: "192.168.1.68:502".parse().expect("Invalid Modbus address"),
    };

    let app = Router::new()
        .route("/read", post(read_registers))
        .route("/write", post(write_register))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ReadForm {
    address: u16,
    count: u16,
}

async fn read_registers(
    State(state): State<AppState>,
    Form(form): Form<ReadForm>,
) -> impl IntoResponse {
    match read_holding_registers(state.modbus_addr, form.address, form.count).await {
        Ok(values) => {
            let mut html = String::from("<h3>Register Values:</h3><ul>");
            for (i, value) in values.iter().enumerate() {
                html.push_str(&format!(
                    "<li>Register {}: {} (0x{:04X})</li>",
                    form.address + i as u16,
                    value,
                    value
                ));
            }
            html.push_str("</ul>");
            Html(html)
        }
        Err(e) => Html(format!("<p style='color: red;'>Error: {}</p>", e)),
    }
}

#[derive(Deserialize)]
struct WriteForm {
    address: u16,
    value: u16,
}

async fn write_register(
    State(state): State<AppState>,
    Form(form): Form<WriteForm>,
) -> impl IntoResponse {
    match write_single_register(state.modbus_addr, form.address, form.value).await {
        Ok(_) => Html(format!(
            "<p style='color: green;'>✓ Successfully wrote {} to register {}</p>",
            form.value, form.address
        )),
        Err(e) => Html(format!("<p style='color: red;'>Error: {}</p>", e)),
    }
}

async fn read_holding_registers(
    addr: SocketAddr,
    address: u16,
    count: u16,
) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let mut ctx = tcp::connect(addr).await?;
    let data = ctx.read_holding_registers(address, count).await??;
    ctx.disconnect().await?;
    Ok(data)
}

async fn write_single_register(
    addr: SocketAddr,
    address: u16,
    value: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = tcp::connect(addr).await?;
    ctx.write_single_register(address, value).await??;
    ctx.disconnect().await?;
    Ok(())
}