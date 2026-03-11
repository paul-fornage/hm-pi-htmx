use crate::{error_targeted, info_targeted, AppState};
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::modbus::modbus_transaction_types::*;

// --- State Management ---



// --- Templates ---

#[derive(Template)]
#[template(path = "components/hreg.html")]
pub struct RegistersTemplate {
    pub start_address: u16,
    pub registers: Vec<(u16, u16)>,
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

    match state.clearcore_registers.manager.read_holding_registers(request).await {
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

    match state.clearcore_registers.manager.write_single_register(request).await {
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