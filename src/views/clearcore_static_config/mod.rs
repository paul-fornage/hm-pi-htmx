mod register_view;
mod register_edit_modal;
mod config_data;
mod file_operations;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Form;
use serde::Deserialize;
use crate::views::{AppView, ViewTemplate};
use crate::modbus::{ModbusValue, RegisterAddress, RegisterMetadata};
use crate::analog_register::AnalogRegisterInfo;
use crate::{debug_targeted, warn_targeted, AppState};
use crate::plc::plc_register_definitions::*;
use register_view::{EditableBooleanRegister, EditableAnalogRegister};
use register_edit_modal::{BooleanEditModalTemplate, AnalogEditModalTemplate};
use config_data::ClearcoreConfig;

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

const CLEARCORE_STATIC_CONFIG_COILS: &[&'static RegisterMetadata] = &[
    &AXIS_X_HOME_DIRECTION_POSITIVE,
    &AXIS_Y_HOME_DIRECTION_POSITIVE,
    &AXIS_Z_HOME_DIRECTION_POSITIVE,
    &USES_Y_AXIS,
    &USES_Z_AXIS,
    &USES_W_AXIS,
];

const CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS: &[AnalogRegisterInfo] = &[
    AnalogRegisterInfo::new(&AXIS_X_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Y_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Z_HOMING_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_X_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Y_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Z_HOMING_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_X_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_X_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_X_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_X_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_Y_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_Y_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_Y_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_Y_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MIN_POS_Z_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_POS_Z_AXIS_HUNDREDTHS, "in", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_Z_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_Z_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
    AnalogRegisterInfo::new(&MAX_VEL_W_AXIS_HUNDREDTHS_PER_MINUTE, "in/min", 2, 0),
    AnalogRegisterInfo::new(&MAX_ACCEL_W_AXIS_HUNDREDTHS_PER_MINUTE_PER_SECOND, "in/min×s", 2, 0),
];

fn find_boolean_register(name: &str) -> Option<&'static RegisterMetadata> {
    CLEARCORE_STATIC_CONFIG_COILS.iter().find(|reg| reg.name == name).copied()
}

fn find_analog_register(name: &str) -> Option<&'static AnalogRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter().find(|reg| reg.meta.name == name)
}

async fn mb_read_bool_helper(state: &AppState, address: &RegisterAddress) -> Option<bool> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, state.clearcore_registers.read(address)).await {
        Ok(Some(ModbusValue::Bool(val))) => Some(val),
        _ => None,
    }
}

async fn mb_read_word_helper(state: &AppState, address: &RegisterAddress) -> Option<u16> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, state.clearcore_registers.read(address)).await {
        Ok(Some(ModbusValue::U16(val))) => Some(val),
        _ => None,
    }
}

pub async fn show_clearcore_config() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering clearcore static config view");
    ClearcoreConfigTemplate {}
}

pub async fn show_clearcore_config_grid(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let mut boolean_registers = Vec::new();
    for meta in CLEARCORE_STATIC_CONFIG_COILS.iter() {
        let value = mb_read_bool_helper(&state, &meta.address).await;
        boolean_registers.push(EditableBooleanRegister {
            meta,
            value,
        });
    }

    let mut analog_registers = Vec::new();
    for info in CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter() {
        let value = mb_read_word_helper(&state, &info.meta.address).await;
        analog_registers.push(EditableAnalogRegister {
            register_info: info,
            value,
        });
    }

    ClearcoreConfigGridTemplate {
        boolean_registers,
        analog_registers,
    }
}

pub async fn show_edit_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(register_name): Path<String>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering edit modal for register: {}", register_name);

    if let Some(meta) = find_boolean_register(&register_name) {
        let current_value = mb_read_bool_helper(&state, &meta.address).await;
        let template = BooleanEditModalTemplate {
            meta,
            current_value,
            register_name,
        };
        return Html(template.render().unwrap());
    }

    if let Some(info) = find_analog_register(&register_name) {
        let current_value = mb_read_word_helper(&state, &info.meta.address).await;
        let template = AnalogEditModalTemplate {
            register_info: info,
            current_value,
            register_name,
        };
        return Html(template.render().unwrap());
    }

    warn_targeted!(HTTP, "Unknown register name: {}", register_name);
    Html("<div>Error: Register not found</div>".to_string())
}

#[derive(Deserialize)]
pub struct RawWriteForm {
    value: String,
}

pub async fn submit_register_write(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(register_name): Path<String>,
    Form(form): Form<RawWriteForm>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Writing register: {}", register_name);

    let render_error = |msg: &str| -> Html<String> {
        Html(format!("<div class='text-red-600 text-sm'>{}</div>", msg))
    };

    if let Some(meta) = find_boolean_register(&register_name) {
        let value = form.value == "true";
        debug_targeted!(HTTP, "Writing boolean to address {}: {}", meta.address.address, value);

        match state.clearcore_registers.write_coil(meta.address.address, value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    if let Some(info) = find_analog_register(&register_name) {
        let val_f32 = match form.value.parse::<f32>() {
            Ok(v) => v,
            Err(_) => {
                warn_targeted!(HTTP, "Invalid float format for {}: {}", register_name, form.value);
                return render_error("Invalid number format provided.").into_response();
            }
        };

        if let Err(msg) = info.validate_semantic_value(val_f32) {
            warn_targeted!(HTTP, "Validation failed for {}: {}", register_name, msg);
            return render_error(&msg).into_response();
        }

        let raw_value = info.convert_to_raw(val_f32);
        debug_targeted!(HTTP, "Writing analog to address {}: {} (raw: {})", info.meta.address.address, val_f32, raw_value);

        match state.clearcore_registers.write_hreg(info.meta.address.address, raw_value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    warn_targeted!(HTTP, "Unknown register name: {}", register_name);
    axum::http::StatusCode::NOT_FOUND.into_response()
}

pub async fn handle_save_config(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Saving clearcore config to disk");

    let config = ClearcoreConfig::from_modbus(&state.clearcore_registers).await;

    match file_operations::save_config(&config).await {
        Ok(_) => Html("<div class='text-green-600 text-sm'>Configuration saved successfully</div>".to_string()),
        Err(e) => {
            warn_targeted!(HTTP, "Failed to save config: {}", e);
            Html(format!("<div class='text-red-600 text-sm'>Failed to save: {}</div>", e))
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "views/clearcore-config.html")]
pub struct ClearcoreConfigTemplate {}
impl ViewTemplate for ClearcoreConfigTemplate { 
    const APP_VIEW_VARIANT: AppView = AppView::ClearcoreConfig; 
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/clearcore-config-grid.html")]
pub struct ClearcoreConfigGridTemplate {
    pub boolean_registers: Vec<EditableBooleanRegister>,
    pub analog_registers: Vec<EditableAnalogRegister>,
}