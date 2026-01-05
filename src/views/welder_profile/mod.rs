pub mod register_view;
pub mod register_edit_modal;
mod analog_details;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Form;
use log::warn;
use serde::Deserialize;
use crate::views::{AppView, ViewTemplate};
use crate::modbus::{ModbusAddressType, ModbusValue, RegisterAddress, RegisterMetadata};
use crate::miller::miller_register_definitions;
use crate::{debug_targeted, warn_targeted, AppState};
use register_view::{EditableBooleanRegister, EditableAnalogRegister};
use register_edit_modal::{BooleanEditModalTemplate, AnalogEditModalTemplate};

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

const WELD_PROFILE_BOOLEAN_REGISTERS: [&'static RegisterMetadata; 8] = [
    &miller_register_definitions::USE_DC_OUTPUT,
    &miller_register_definitions::USE_EP_POLARITY,
    &miller_register_definitions::BOOST_EN,
    &miller_register_definitions::DROOP_EN,
    &miller_register_definitions::USE_LOW_OCV,
    &miller_register_definitions::PULSER_EN,
    &miller_register_definitions::USE_LOW_AC_COMMUTATION_AMP,
    &miller_register_definitions::AC_INDEPENDANT_EN,
];

struct AnalogRegisterConfig {
    meta: &'static RegisterMetadata,
    unit: &'static str,
    scale: u16,
    precision: u16,
    min_value: u16,
    max_value: u16,
}

const WELD_PROFILE_ANALOG_REGISTERS: [AnalogRegisterConfig; 27] = [
    AnalogRegisterConfig { meta: &miller_register_definitions::TUNGSTEN_PRESET, unit: "", scale: 1, precision: 0, min_value: 0, max_value: 9 },
    AnalogRegisterConfig { meta: &miller_register_definitions::PRESET_MIN_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 1, max_value: 63 },
    AnalogRegisterConfig { meta: &miller_register_definitions::ARC_START_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 5, max_value: 200 },
    AnalogRegisterConfig { meta: &miller_register_definitions::ARC_START_TIME, unit: "×10ms", scale: 1, precision: 0, min_value: 0, max_value: 25 },
    AnalogRegisterConfig { meta: &miller_register_definitions::ARC_START_SLOPE_TIME, unit: "×10ms", scale: 1, precision: 0, min_value: 0, max_value: 25 },
    AnalogRegisterConfig { meta: &miller_register_definitions::ARC_START_AC_TIME, unit: "×10ms", scale: 1, precision: 0, min_value: 0, max_value: 25 },
    AnalogRegisterConfig { meta: &miller_register_definitions::ARC_START_POLARITY_PHASE, unit: "", scale: 1, precision: 0, min_value: 0, max_value: 1 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_EN_WAVE_SHAPE, unit: "", scale: 1, precision: 0, min_value: 0, max_value: 3 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_EP_WAVE_SHAPE, unit: "", scale: 1, precision: 0, min_value: 0, max_value: 3 },
    AnalogRegisterConfig { meta: &miller_register_definitions::HOT_START_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 20 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_EN_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 0, max_value: 1023 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_EP_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 0, max_value: 1023 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_BALANCE, unit: "%", scale: 1, precision: 0, min_value: 30, max_value: 99 },
    AnalogRegisterConfig { meta: &miller_register_definitions::AC_FREQUENCY, unit: "Hz", scale: 1, precision: 0, min_value: 20, max_value: 400 },
    AnalogRegisterConfig { meta: &miller_register_definitions::WELD_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 0, max_value: 1023 },
    AnalogRegisterConfig { meta: &miller_register_definitions::PULSER_PPS, unit: "Hz", scale: 10, precision: 1, min_value: 0, max_value: 50000 },
    AnalogRegisterConfig { meta: &miller_register_definitions::PULSER_PEAK_TIME, unit: "%", scale: 1, precision: 0, min_value: 5, max_value: 95 },
    AnalogRegisterConfig { meta: &miller_register_definitions::PREFLOW_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 250 },
    AnalogRegisterConfig { meta: &miller_register_definitions::INITIAL_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 0, max_value: 1023 },
    AnalogRegisterConfig { meta: &miller_register_definitions::INITIAL_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 250 },
    AnalogRegisterConfig { meta: &miller_register_definitions::INITIAL_SLOPE_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 500 },
    AnalogRegisterConfig { meta: &miller_register_definitions::MAIN_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 9990 },
    AnalogRegisterConfig { meta: &miller_register_definitions::FINAL_SLOPE_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 500 },
    AnalogRegisterConfig { meta: &miller_register_definitions::FINAL_AMPERAGE, unit: "A", scale: 1, precision: 0, min_value: 0, max_value: 1023 },
    AnalogRegisterConfig { meta: &miller_register_definitions::FINAL_TIME, unit: "×0.1s", scale: 1, precision: 0, min_value: 0, max_value: 250 },
    AnalogRegisterConfig { meta: &miller_register_definitions::POSTFLOW_TIME, unit: "s", scale: 1, precision: 0, min_value: 0, max_value: 51 },
    AnalogRegisterConfig { meta: &miller_register_definitions::HOT_WIRE_VOLTAGE, unit: "V", scale: 1, precision: 0, min_value: 5, max_value: 20 },
];

fn find_boolean_register(name: &str) -> Option<&'static RegisterMetadata> {
    WELD_PROFILE_BOOLEAN_REGISTERS.iter().find(|reg| reg.name == name).copied()
}

fn find_analog_register(name: &str) -> Option<&AnalogRegisterConfig> {
    WELD_PROFILE_ANALOG_REGISTERS.iter().find(|reg| reg.meta.name == name)
}

async fn mb_read_bool_helper(state: &AppState, address: &RegisterAddress) -> Option<bool> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, state.miller_registers.read(address)).await {
        Ok(Some(ModbusValue::Bool(val))) => Some(val),
        _ => None,
    }
}

async fn mb_read_word_helper(state: &AppState, address: &RegisterAddress) -> Option<u16> {
    match tokio::time::timeout(READ_TIMEOUT_DURATION, state.miller_registers.read(address)).await {
        Ok(Some(ModbusValue::U16(val))) => Some(val),
        _ => None,
    }
}

pub async fn show_welder_profile() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering welder profile view");
    WelderProfileTemplate {}
}

pub async fn show_welder_profile_grid(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering welder profile grid component");

    let mut boolean_registers = Vec::new();
    for meta in WELD_PROFILE_BOOLEAN_REGISTERS.iter() {
        let value = mb_read_bool_helper(&state, &meta.address).await;
        boolean_registers.push(EditableBooleanRegister {
            meta,
            value,
        });
    }

    let mut analog_registers = Vec::new();
    for config in WELD_PROFILE_ANALOG_REGISTERS.iter() {
        let value = mb_read_word_helper(&state, &config.meta.address).await;
        analog_registers.push(EditableAnalogRegister {
            meta: config.meta,
            value,
            unit: config.unit,
            scale: config.scale,
            precision: config.precision,
            min_value: config.min_value,
            max_value: config.max_value,
        });
    }

    WelderProfileGridTemplate {
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

    if let Some(config) = find_analog_register(&register_name) {
        let current_value = mb_read_word_helper(&state, &config.meta.address).await;
        let template = AnalogEditModalTemplate {
            meta: config.meta,
            current_value,
            register_name,
            unit: config.unit,
            scale: config.scale,
            precision: config.precision,
            min_value: config.min_value,
            max_value: config.max_value,
        };
        return Html(template.render().unwrap());
    }

    warn_targeted!(HTTP, "Unknown register name: {}", register_name);
    Html("<div>Error: Register not found</div>".to_string())
}

#[derive(Deserialize)]
pub struct BooleanWriteForm {
    value: String,
}

#[derive(Deserialize)]
pub struct AnalogWriteForm {
    value: f32,
}

pub async fn submit_register_write(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(register_name): Path<String>,
    Form(form): Form<serde_json::Value>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Writing register: {}", register_name);

    if let Some(meta) = find_boolean_register(&register_name) {
        let form: BooleanWriteForm = serde_json::from_value(form).unwrap();
        let value = form.value == "true";
        let result = state.miller_registers.manager.write_single_coil(crate::modbus::modbus_transaction_types::WriteSingleCoilRequest {
            address: meta.address.address,
            value,
        }).await;

        match result {
            Ok(_) => debug_targeted!(HTTP, "Successfully wrote boolean register: {}", register_name),
            Err(e) => warn!("Failed to write boolean register {}: {:?}", register_name, e),
        }
        return axum::http::StatusCode::OK.into_response();
    }

    if let Some(config) = find_analog_register(&register_name) {
        let form: AnalogWriteForm = serde_json::from_value(form).unwrap();
        let raw_value = (form.value * config.scale as f32) as u16;
        let result = state.miller_registers.manager.write_single_register(crate::modbus::modbus_transaction_types::WriteSingleRegisterRequest {
            address: config.meta.address.address,
            value: raw_value,
        }).await;

        match result {
            Ok(_) => debug_targeted!(HTTP, "Successfully wrote analog register: {}", register_name),
            Err(e) => warn!("Failed to write analog register {}: {:?}", register_name, e),
        }
        return axum::http::StatusCode::OK.into_response();
    }

    warn_targeted!(HTTP, "Unknown register name: {}", register_name);
    axum::http::StatusCode::NOT_FOUND.into_response()
}

#[derive(Template, WebTemplate)]
#[template(path = "views/welder-profile.html")]
pub struct WelderProfileTemplate {}
impl ViewTemplate for WelderProfileTemplate { const APP_VIEW_VARIANT: AppView = AppView::WelderProfile; }

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/welder-profile-grid.html")]
pub struct WelderProfileGridTemplate {
    pub boolean_registers: Vec<EditableBooleanRegister>,
    pub analog_registers: Vec<EditableAnalogRegister>,
}
