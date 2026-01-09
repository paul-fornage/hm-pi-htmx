mod config_data;
mod file_operations;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Form;
use serde::Deserialize;
use crate::views::{AppView, ViewTemplate};
use crate::views::shared::{EditableBooleanRegister, EditableAnalogRegister, BooleanEditModalTemplate, AnalogEditModalTemplate, WriteErrorModalTemplate, mb_read_bool_helper, mb_read_word_helper};
use crate::modbus::{ModbusValue, RegisterAddress, RegisterMetadata};
use crate::views::shared::analog_register::AnalogRegisterInfo;
use crate::{debug_targeted, warn_targeted, AppState};
use crate::plc::plc_register_definitions::*;
use config_data::ClearcoreConfig;
use crate::views::shared::boolean_register::BooleanRegisterInfo;

const BASE_URL: &str = "/clearcore-config";

const CLEARCORE_STATIC_CONFIG_COILS: &[BooleanRegisterInfo] = &[
    BooleanRegisterInfo::new_default(&USES_Y_AXIS),
    BooleanRegisterInfo::new_default(&USES_Z_AXIS),
    BooleanRegisterInfo::new_default(&USES_W_AXIS),
    BooleanRegisterInfo::new_custom(&AXIS_X_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
    BooleanRegisterInfo::new_custom(&AXIS_Y_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
    BooleanRegisterInfo::new_custom(&AXIS_Z_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
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

fn find_boolean_register(name: &str) -> Option<&'static BooleanRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_COILS.iter().find(|reg| reg.meta.name == name)
}

fn find_analog_register(name: &str) -> Option<&'static AnalogRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter().find(|reg| reg.meta.name == name)
}


pub async fn show_clearcore_config() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering clearcore static config view");
    ClearcoreConfigTemplate {}
}

pub async fn show_clearcore_config_grid(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let mut boolean_registers = Vec::new();
    for info in CLEARCORE_STATIC_CONFIG_COILS.iter() {
        let value = mb_read_bool_helper(&state.clearcore_registers, &info.meta.address).await;
        boolean_registers.push(EditableBooleanRegister {
            register_info: info,
            value,
            base_url: BASE_URL,
        });
    }

    let mut analog_registers = Vec::new();
    for info in CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter() {
        let value = mb_read_word_helper(&state.clearcore_registers, &info.meta.address).await;
        analog_registers.push(EditableAnalogRegister {
            register_info: info,
            value,
            base_url: BASE_URL,
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

    if let Some(info) = find_boolean_register(&register_name) {
        let current_value = mb_read_bool_helper(&state.clearcore_registers, &info.meta.address).await;
        let template = BooleanEditModalTemplate {
            register_info: info,
            current_value,
            register_name,
            base_url: BASE_URL,
        };
        return Html(template.render().unwrap());
    }

    if let Some(info) = find_analog_register(&register_name) {
        let current_value = mb_read_word_helper(&state.clearcore_registers, &info.meta.address).await;
        let template = AnalogEditModalTemplate {
            register_info: info,
            current_value,
            register_name,
            base_url: BASE_URL,
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
        let t = WriteErrorModalTemplate {
            title: "Write Failed".to_string(),
            message: msg.to_string(),
        };
        Html(t.render().unwrap())
    };

    if let Some(info) = find_boolean_register(&register_name) {
        let value = form.value == "true";
        debug_targeted!(HTTP, "Writing boolean to address {}: {}", info.meta.address.address, value);

        match state.clearcore_registers.write_coil(info.meta.address.address, value).await {
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