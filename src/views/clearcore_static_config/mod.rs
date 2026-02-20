pub mod config_data;
pub mod file_operations;
pub mod json_serde;
mod boot_procedure;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use crate::views::{AppView, ViewTemplate};
use crate::views::shared::{EditableBooleanRegister, EditableAnalogRegister, BooleanEditModalTemplate, AnalogEditModalTemplate, WriteErrorModalTemplate, mb_read_bool_helper, mb_read_word_helper};
use crate::modbus::{ModbusValue, RegisterAddress, RegisterMetadata};
use crate::views::shared::analog_register::AnalogRegisterInfo;
use crate::{debug_targeted, error_targeted, warn_targeted, AppState};
use crate::plc::plc_register_definitions::*;
use config_data::ClearcoreConfig;
use crate::views::shared::analog_dword_register::AnalogDwordRegisterInfo;
use crate::views::shared::boolean_register::BooleanRegisterInfo;
use crate::views::shared::modbus_helpers::mb_read_dword_helper;
use crate::views::shared::register_edit_modal::AnalogDwordEditModalTemplate;
use crate::views::shared::register_view::EditableDwordAnalogRegister;
use crate::views::shared::result_feedback::FeedbackResult;

const BASE_URL: &str = "/clearcore-config";

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(AppView::ClearcoreConfig.url(), get(show_clearcore_config))
        .route("/clearcore-config/status", get(show_clearcore_config_status))
        .route("/clearcore-config/grid", get(show_clearcore_config_grid))
        .route("/clearcore-config/edit/{register_name}", get(show_edit_modal))
        .route("/clearcore-config/write/{register_name}", post(submit_register_write))
        .route("/clearcore-config/save", get(handle_save_config))
        .route("/clearcore-config/load", get(handle_load_config))
        .route("/clearcore-config/apply", get(handle_apply_config))
}

const CLEARCORE_STATIC_CONFIG_COILS: &[BooleanRegisterInfo] = &[
    BooleanRegisterInfo::new_default(&USES_Y_AXIS),
    BooleanRegisterInfo::new_default(&USES_Z_AXIS),
    BooleanRegisterInfo::new_default(&USES_W_AXIS),
    BooleanRegisterInfo::new_custom(&AXIS_X_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
    BooleanRegisterInfo::new_custom(&AXIS_Y_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
    BooleanRegisterInfo::new_custom(&AXIS_Z_HOME_DIRECTION_POSITIVE, "Positive", "Negative"),
];

const CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS: &[AnalogRegisterInfo] = &[
    AnalogRegisterInfo::new(&UDP_LOG_PORT, "port", 0, 0),
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
    AnalogRegisterInfo::new(&AXIS_X_DEFAULT_JOG_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Y_DEFAULT_JOG_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_Z_DEFAULT_JOG_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&AXIS_W_DEFAULT_JOG_SPEED, "in/min", 2, 0),
];

const CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS: &[AnalogDwordRegisterInfo] = &[
    AnalogDwordRegisterInfo::new(
        &INCHES_PER_STEP_X_AXIS_LOWER, &INCHES_PER_STEP_X_AXIS_UPPER,
        "in/step", 9, 0),
    AnalogDwordRegisterInfo::new(
        &INCHES_PER_STEP_Y_AXIS_LOWER, &INCHES_PER_STEP_Y_AXIS_UPPER,
        "in/step", 9, 0),
    AnalogDwordRegisterInfo::new(
        &INCHES_PER_STEP_Z_AXIS_LOWER, &INCHES_PER_STEP_Z_AXIS_UPPER,
        "in/step", 9, 0),
    AnalogDwordRegisterInfo::new(
        &INCHES_PER_STEP_W_AXIS_LOWER, &INCHES_PER_STEP_W_AXIS_UPPER,
        "in/step", 9, 0),
];

fn find_boolean_register(name: &str) -> Option<&'static BooleanRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_COILS.iter().find(|reg| reg.meta.name == name)
}

fn find_analog_register(name: &str) -> Option<&'static AnalogRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_ANALOG_REGISTERS.iter().find(|reg| reg.meta.name == name)
}

fn find_dword_analog_register(name: &str) -> Option<&'static AnalogDwordRegisterInfo> {
    CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.iter().find(|reg| reg.get_meta().name == name)
}

pub async fn show_clearcore_config() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering clearcore static config view");
    ClearcoreConfigTemplate {}
}

pub async fn show_clearcore_config_status(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let hmi_config_uploaded = state.clearcore_configured.load(std::sync::atomic::Ordering::Acquire);
    let clearcore_config_uploaded = state
        .clearcore_registers
        .read_coil(CONFIG_READY.address.address)
        .await;

    ClearcoreConfigStatusTemplate {
        hmi_config_uploaded,
        clearcore_config_uploaded,
    }
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
    
    let mut analog_dword_registers = Vec::new();
    for info in CLEARCORE_STATIC_CONFIG_DWORD_ANALOG_REGISTERS.iter() {
        let value = mb_read_dword_helper(&state.clearcore_registers, &info.get_meta().address).await;
        analog_dword_registers.push(EditableDwordAnalogRegister{
            register_info: info,
            value,
            base_url: BASE_URL,
        })
    }

    ClearcoreConfigGridTemplate {
        boolean_registers,
        analog_registers,
        analog_dword_registers,
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

    if let Some(info) = find_dword_analog_register(&register_name) {
        let current_value = mb_read_dword_helper(&state.clearcore_registers, &info.get_meta().address).await;
        let template = AnalogDwordEditModalTemplate {
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

    if let Some(info) = find_dword_analog_register(&register_name) {
        let val_f64 = match form.value.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                warn_targeted!(HTTP, "Invalid float format for {}: {}", register_name, form.value);
                return render_error("Invalid number format provided.").into_response();
            }
        };
        if let Err(msg) = info.validate_semantic_value(val_f64) {
            warn_targeted!(HTTP, "Validation failed for {}: {}", register_name, msg);
            return render_error(&msg).into_response();
        }

        let raw_value = info.convert_to_raw(val_f64);
        debug_targeted!(HTTP, "Writing dword analog to address {}: {} (raw: {})", info.get_meta().address.address, val_f64, raw_value);

        match state.clearcore_registers.write_u32(info.get_meta().address.address, raw_value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    // TODO: Need to know what dumbass agent tried to give user feedback like this
    warn_targeted!(HTTP, "Unknown register name: {}", register_name);
    axum::http::StatusCode::NOT_FOUND.into_response()
}

pub async fn handle_save_config(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Saving clearcore config to disk");

    match ClearcoreConfig::from_modbus(&state.clearcore_registers).await{
        Ok(config) => {
            match config.save_to_file().await {
                Ok(_) => FeedbackResult::new_ok("Configuration saved successfully".to_string()),
                Err(e) => {
                    warn_targeted!(FS, "Failed to save config: {}", e);
                    FeedbackResult::new_err(e)
                }
            }
        },
        Err(e) => {
            error_targeted!(FS, "Failed to generate config from modbus: {}", e);
            FeedbackResult::new_err(e.to_string())
        }
    }
}

pub async fn handle_load_config(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Loading clearcore config from disk");

    let config = match ClearcoreConfig::load_config().await {
        Ok(Some(config)) => config,
        Ok(None) => {
            return FeedbackResult::new_err("No clearcore configuration found on disk".to_string());
        }
        Err(e) => {
            error_targeted!(FS, "Failed to load clearcore config from disk: {}", e);
            return FeedbackResult::new_err(e.to_string());
        }
    };

    match state.clearcore_registers.read_coil(CONFIG_READY.address.address).await {
        None => {
            error_targeted!(MODBUS, "Failed to read config ready status from modbus");
            FeedbackResult::new_err("Failed to read config ready status from modbus".to_string())
        }
        Some(true) => {
            FeedbackResult::new_err("Clearcore reports configuration already loaded".to_string())
        }
        Some(false) => match config.write_to_modbus(&state.clearcore_registers).await {
            Ok(_) => FeedbackResult::new_ok("Config loaded from disk".to_string()),
            Err(e) => {
                error_targeted!(MODBUS, "Failed to load config to modbus: {}", e);
                FeedbackResult::new_err(e.to_string())
            }
        },
    }
}

pub async fn handle_apply_config(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Saving clearcore config to disk");

    let config_applied_already = state.clearcore_configured.load(std::sync::atomic::Ordering::Acquire);
    let cc_says_cfg_ready = state.clearcore_registers.read_coil(CONFIG_READY.address.address).await;
    if config_applied_already {
        if matches!(cc_says_cfg_ready, Some(false)) {
            log::warn!("Clearcore configuration already applied, but mb says it is not.");
        } else {
            return FeedbackResult::new_err("Clearcore configuration can only be applied once per boot. \
                                            Last save applied by default at startup.".to_string());
        }
    }
    match state.clearcore_registers.read_coil(CONFIG_READY.address.address).await {
        None => {
            error_targeted!(MODBUS, "Failed to read config ready status from modbus");
            FeedbackResult::new_err("Failed to read config ready status from modbus".to_string())
        }
        Some(true) => {
            error_targeted!(MODBUS, "Local state.clearcore_configured reads not configured, but mb says it is.");
            FeedbackResult::new_err("Internal state invariant; see logs".to_string())
        },
        Some(false) => {
            match state.clearcore_registers.write_coil(CONFIG_READY.address.address, true).await{
                Err(e) => {
                    error_targeted!(MODBUS, "Failed to write config ready status to modbus: {}", e);
                    FeedbackResult::new_err(format!("Failed to write config ready status to modbus: {e:?}"))
                }
                Ok(_) => {
                    state.clearcore_configured.store(true, std::sync::atomic::Ordering::Release);
                    FeedbackResult::new_ok("Config applied successfully")
                }
            }
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
#[template(path = "components/clearcore-config/config-status.html")]
pub struct ClearcoreConfigStatusTemplate {
    pub hmi_config_uploaded: bool,
    pub clearcore_config_uploaded: Option<bool>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/clearcore-config-grid.html")]
pub struct ClearcoreConfigGridTemplate {
    pub boolean_registers: Vec<EditableBooleanRegister>,
    pub analog_registers: Vec<EditableAnalogRegister>,
    pub analog_dword_registers: Vec<EditableDwordAnalogRegister>,
}
