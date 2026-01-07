pub mod register_view;
pub mod register_edit_modal;
pub mod profile_metadata;
pub mod raw_weld_profile;
pub mod weld_profile;
pub mod file_operations;
mod analog_details;
mod special_case_registers;
mod write_error_modal;
mod description_edit_modal;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Form;
use serde::Deserialize;
use crate::views::{AppView, ViewTemplate};
use crate::modbus::{ModbusAddressType, ModbusValue, RegisterAddress, RegisterMetadata};
use crate::miller::miller_register_definitions;
use crate::miller::analog_register::AnalogRegisterInfo;
use crate::{debug_targeted, warn_targeted, AppState};
use register_view::{EditableBooleanRegister, EditableAnalogRegister, EditableEnumRegister, EditablePostflowRegister};
use register_edit_modal::{BooleanEditModalTemplate, AnalogEditModalTemplate, EnumEditModalTemplate, PostflowEditModalTemplate, PolarityEditModalTemplate};
use crate::views::welder_profile::write_error_modal::WriteErrorModalTemplate;
use crate::views::welder_profile::description_edit_modal::DescriptionEditModalTemplate;

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

const WELD_PROFILE_ENUM_REGISTERS: [&'static RegisterMetadata; 4] = [
    &miller_register_definitions::TUNGSTEN_PRESET,
    &miller_register_definitions::ARC_START_POLARITY_PHASE,
    &miller_register_definitions::AC_EN_WAVE_SHAPE,
    &miller_register_definitions::AC_EP_WAVE_SHAPE,
];



const WELD_PROFILE_ANALOG_REGISTERS: [AnalogRegisterInfo; 22] = [
    // Preset Amperage Minimum: Power Source AC / DC Amperage Minimum -25A(Tungsten General) Or 63A(Tungsten Disabled), Res 1A
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::PRESET_MIN_AMPERAGE, "A", 0, 0, 63, 1),
    // Arc Start Amperage: 5A - 200A, Res: 1A
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::ARC_START_AMPERAGE, "A", 0, 0, 200, 5),
    // Arc Start Time: 0(Off) - 25(x10ms), Res: 1(x10ms) -> 0.01s res
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::ARC_START_TIME, "s", 2, 0, 25, 0),
    // Arc Start Slope Time: 0(Off) - 25(x10ms), Res: 1(x10ms) -> 0.01s res
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::ARC_START_SLOPE_TIME, "s", 2, 0, 25, 0),
    // Arc Start AC Time: 0(Off) - 25(x10ms), Res: 1(x10ms) -> 0.01s res
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::ARC_START_AC_TIME, "s", 2, 0, 25, 0),
    // Hot Start Time: Range: 0(Off) -20, Resolution: 0.1 Second
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::HOT_START_TIME, "s", 1, 0, 20, 0),
    // AC EN Amperage, Preset Amps Min - PS Amps Max, Res: 1A
    AnalogRegisterInfo::new(&miller_register_definitions::AC_EN_AMPERAGE, "A", 0, 0),
    // AC EP Amperage, Preset Amps Min - PS Amps Max, Res: 1A
    AnalogRegisterInfo::new(&miller_register_definitions::AC_EP_AMPERAGE, "A", 0, 0),
    // AC Balance, 30-99%, Res: 1%
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::AC_BALANCE, "%", 0, 0, 99, 30),
    // AC Frequency, 20-400Hz, Res: 1Hz
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::AC_FREQUENCY, "Hz", 0, 0, 400, 20),
    // Weld Amperage(DC or AC), Preset Amps Min - PS Amps Max, Res: 1A
    AnalogRegisterInfo::new(&miller_register_definitions::WELD_AMPERAGE, "A", 0, 0),
    // Pulser - Pulses Per Second (PPS): Range: 0(Off) – 50000 / 5000 Power Source Dependent, Resolution: 0.1 Hertz
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::PULSER_PPS, "Hz", 1, 0, 50000, 0),
    // Pulser - Peak Time, 5-95%, Res: 1%
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::PULSER_PEAK_TIME, "%", 0, 0, 95, 5),
    // Prelow Time, 0(Off) - 250, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::PREFLOW_TIME, "s", 1, 0, 250, 0),
    // Initial Amperage, Preset Amps Min - PS Amps Max, Res: 1A
    AnalogRegisterInfo::new(&miller_register_definitions::INITIAL_AMPERAGE, "A", 0, 0),
    // Initial Time, 0(Off) - 250, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::INITIAL_TIME, "s", 1, 0, 250, 0),
    // Initial Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::INITIAL_SLOPE_TIME, "s", 1, 0, 500, 0),
    // Main Time, 0(Off) - 9990, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::MAIN_TIME, "s", 1, 0, 9990, 0),
    // Final Slope Time, 0(Off) - 500, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::FINAL_SLOPE_TIME, "s", 1, 0, 500, 0),
    // Final Amperage, Preset Amps Min - PS Amps Max, Res: 1A
    AnalogRegisterInfo::new(&miller_register_definitions::FINAL_AMPERAGE, "A", 0, 0),
    // Final Time, 0(Off) - 250, Res: 1(x0.1Sec)
    AnalogRegisterInfo::new_with_raw_bounds(&miller_register_definitions::FINAL_TIME, "s", 1, 0, 250, 0),
    // Hot Wire Voltage, 5-20, Res: 1V
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::HOT_WIRE_VOLTAGE, "V", 0, 0, 20, 5),
];

fn find_boolean_register(name: &str) -> Option<&'static RegisterMetadata> {
    WELD_PROFILE_BOOLEAN_REGISTERS.iter().find(|reg| reg.name == name).copied()
}

fn find_analog_register(name: &str) -> Option<&'static AnalogRegisterInfo> {
    WELD_PROFILE_ANALOG_REGISTERS.iter().find(|reg| reg.meta.name == name)
}

fn find_enum_register(name: &str) -> Option<&'static RegisterMetadata> {
    WELD_PROFILE_ENUM_REGISTERS.iter().find(|reg| reg.name == name).copied()
}

enum EnumRegisterType {
    TungstenPreset,
    Polarity,
    WaveShapeEN,
    WaveShapeEP,
}

fn get_enum_register_type(name: &str) -> Option<EnumRegisterType> {
    match name {
        "TUNGSTEN PRESET" => Some(EnumRegisterType::TungstenPreset),
        "ARC START POLARITY PHASE" => Some(EnumRegisterType::Polarity),
        "AC EN WAVE SHAPE" => Some(EnumRegisterType::WaveShapeEN),
        "AC EP WAVE SHAPE" => Some(EnumRegisterType::WaveShapeEP),
        _ => None,
    }
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
    // debug_targeted!(HTTP, "Rendering welder profile grid component");

    let mut boolean_registers = Vec::new();
    for meta in WELD_PROFILE_BOOLEAN_REGISTERS.iter() {
        let value = mb_read_bool_helper(&state, &meta.address).await;
        boolean_registers.push(EditableBooleanRegister {
            meta,
            value,
        });
    }

    let mut analog_registers = Vec::new();
    for info in WELD_PROFILE_ANALOG_REGISTERS.iter() {
        let value = mb_read_word_helper(&state, &info.meta.address).await;
        analog_registers.push(EditableAnalogRegister {
            register_info: info,
            value,
        });
    }

    let mut enum_registers = Vec::new();
    for meta in WELD_PROFILE_ENUM_REGISTERS.iter() {
        let value = mb_read_word_helper(&state, &meta.address).await;
        let register = match get_enum_register_type(meta.name) {
            Some(EnumRegisterType::TungstenPreset) => EditableEnumRegister::new_tungsten(meta, value),
            Some(EnumRegisterType::Polarity) => EditableEnumRegister::new_polarity(meta, value),
            Some(EnumRegisterType::WaveShapeEN) | Some(EnumRegisterType::WaveShapeEP) =>
                EditableEnumRegister::new_wave_shape(meta, value),
            None => continue,
        };
        enum_registers.push(register);
    }

    // Handle postflow time separately
    let postflow_value = mb_read_word_helper(&state, &miller_register_definitions::POSTFLOW_TIME.address).await;
    let postflow_register = EditablePostflowRegister {
        meta: &miller_register_definitions::POSTFLOW_TIME,
        value: postflow_value,
    };

    WelderProfileGridTemplate {
        boolean_registers,
        analog_registers,
        enum_registers,
        postflow_register,
    }
}

pub async fn show_profile_metadata(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let metadata_lock = state.weld_profile_metadata.lock().await;
    ProfileMetadataDisplayTemplate {
        name: metadata_lock.name.clone(),
        description: metadata_lock.description.clone(),
    }
}

pub async fn show_description_edit_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering description edit modal");

    let metadata_lock = state.weld_profile_metadata.lock().await;
    let current_description = metadata_lock.description.clone();

    DescriptionEditModalTemplate {
        current_description,
    }
}

pub async fn update_description(
    axum::extract::State(state): axum::extract::State<AppState>,
    Form(form): Form<DescriptionUpdateForm>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Updating profile description");

    let mut metadata_lock = state.weld_profile_metadata.lock().await;

    if form.description.trim().is_empty() {
        metadata_lock.description = None;
    } else {
        metadata_lock.set_description(form.description.trim().to_string());
    }

    let response = ProfileMetadataDisplayTemplate {
        name: metadata_lock.name.clone(),
        description: metadata_lock.description.clone(),
    };

    response
}

#[derive(Deserialize)]
pub struct DescriptionUpdateForm {
    description: String,
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

    if let Some(meta) = find_enum_register(&register_name) {
        let current_value = mb_read_word_helper(&state, &meta.address).await;
        let template = match get_enum_register_type(&register_name) {
            Some(EnumRegisterType::TungstenPreset) =>
                EnumEditModalTemplate::new_tungsten(meta, current_value, register_name),
            Some(EnumRegisterType::Polarity) => {
                let polarity_template = PolarityEditModalTemplate {
                    meta,
                    current_value,
                    register_name,
                };
                return Html(polarity_template.render().unwrap());
            },
            Some(EnumRegisterType::WaveShapeEN) | Some(EnumRegisterType::WaveShapeEP) =>
                EnumEditModalTemplate::new_wave_shape(meta, current_value, register_name),
            None => {
                warn_targeted!(HTTP, "Unknown enum register type: {}", register_name);
                return Html("<div>Error: Unknown enum register type</div>".to_string());
            }
        };
        return Html(template.render().unwrap());
    }

    // Handle postflow time specially
    if register_name == "POSTFLOW TIME" {
        let current_value = mb_read_word_helper(&state, &miller_register_definitions::POSTFLOW_TIME.address).await;
        let template = PostflowEditModalTemplate {
            meta: &miller_register_definitions::POSTFLOW_TIME,
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
        let t = WriteErrorModalTemplate {
            title: "Write Failed".to_string(),
            message: msg.to_string(),
        };
        Html(t.render().unwrap())
    };

    if let Some(meta) = find_boolean_register(&register_name) {
        let value = form.value == "true";
        debug_targeted!(HTTP, "Writing boolean to address {}: {}", meta.address.address, value);

        match state.miller_registers.write_coil(meta.address.address, value).await {
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

        match state.miller_registers.write_hreg(info.meta.address.address, raw_value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    if let Some(meta) = find_enum_register(&register_name) {
        let val_u16 = match form.value.parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                warn_targeted!(HTTP, "Invalid int format for {}: {}", register_name, form.value);
                return render_error("Invalid integer format provided.").into_response();
            }
        };

        // Validate the value based on register type
        let validation_result = match get_enum_register_type(&register_name) {
            Some(EnumRegisterType::TungstenPreset) => {
                use special_case_registers::TungstenPreset;
                use num_enum::TryFromPrimitive;
                TungstenPreset::try_from_primitive(val_u16)
                    .map(|_| ())
                    .map_err(|_| "Invalid tungsten preset value".to_string())
            },
            Some(EnumRegisterType::Polarity) => {
                use special_case_registers::ElectrodePolarity;
                use num_enum::TryFromPrimitive;
                ElectrodePolarity::try_from_primitive(val_u16)
                    .map(|_| ())
                    .map_err(|_| "Invalid polarity value".to_string())
            },
            Some(EnumRegisterType::WaveShapeEN) | Some(EnumRegisterType::WaveShapeEP) => {
                use special_case_registers::WaveShape;
                use num_enum::TryFromPrimitive;
                WaveShape::try_from_primitive(val_u16)
                    .map(|_| ())
                    .map_err(|_| "Invalid wave shape value".to_string())
            },
            None => Err("Unknown enum register type".to_string()),
        };

        if let Err(msg) = validation_result {
            warn_targeted!(HTTP, "Validation failed for {}: {}", register_name, msg);
            return render_error(&msg).into_response();
        }

        debug_targeted!(HTTP, "Writing enum to address {}: {}", meta.address.address, val_u16);

        match state.miller_registers.write_hreg(meta.address.address, val_u16).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    // Handle postflow time specially
    if register_name == "POSTFLOW TIME" {
        let val_u16 = match form.value.parse::<u16>() {
            Ok(v) => v,
            Err(_) => {
                warn_targeted!(HTTP, "Invalid int format for {}: {}", register_name, form.value);
                return render_error("Invalid integer format provided.").into_response();
            }
        };

        use special_case_registers::PostFlowTime;
        if let Err(msg) = PostFlowTime::from_raw(val_u16) {
            warn_targeted!(HTTP, "Validation failed for {}: {}", register_name, msg);
            return render_error(&msg).into_response();
        }

        debug_targeted!(HTTP, "Writing postflow time to address {}: {}", miller_register_definitions::POSTFLOW_TIME.address.address, val_u16);

        match state.miller_registers.write_hreg(miller_register_definitions::POSTFLOW_TIME.address.address, val_u16).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
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
    pub enum_registers: Vec<EditableEnumRegister>,
    pub postflow_register: EditablePostflowRegister,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/profile-metadata-display.html")]
pub struct ProfileMetadataDisplayTemplate {
    pub name: Option<String>,
    pub description: Option<String>,
}