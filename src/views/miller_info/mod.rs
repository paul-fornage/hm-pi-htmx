



pub mod register_view;
pub mod register_details_modal;
pub mod error_list;
pub mod version_info;

use crate::miller::miller_register_definitions;
use crate::miller::miller_register_types::{ArcCycles, ArcTime, SerialNumber, SoftwareUpdateRevision, SubModuleSoftwareVersion, WeldProcess, WeldState};
use crate::modbus::{ModbusValue, RegisterAddress, RegisterMetadata};
use crate::views::{build_header_context, AppView, HeaderContext, ViewTemplate};
use crate::{debug_targeted, error_targeted, warn_targeted, AppState};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get};
use axum::{Router};
use futures::future::join_all;
use num_enum::FromPrimitive;
use register_view::{AnalogRegisterTemplate, BooleanRegisterTemplate, EnumRegisterTemplate, StatisticsBarTemplate};

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);
const BASE_URL: &str = AppView::MillerInfo.url();
pub fn routes() -> Router<AppState> {
    let page = AppView::MillerInfo;
    Router::new()
        .route(page.url(), get(show_miller_info))
        .route(&page.url_with_path("/grid"), get(show_miller_info_grid))
        .route(&page.url_with_path("/modal/{register_name}"), get(register_details_modal::modal_handler))
}

async fn mb_read_helper(state: &AppState,address: &RegisterAddress) -> Option<ModbusValue>{
    match tokio::time::timeout(READ_TIMEOUT_DURATION,
                               state.miller_registers.read(address)).await {
        Ok(Some(val)) => {
            Some(val)
        }
        Ok(None) => {
            warn_targeted!(MODBUS, "Failed to retrieve value from cache: {:?}", address);
            None
        }
        Err(_) => {
            warn_targeted!(MODBUS, "Timeout while reading register {:?}", address);
            None
        }
    }
}
async fn mb_read_bool_helper(state: &AppState,address: &RegisterAddress) -> Option<bool>{
    match mb_read_helper(state,address).await {
        Some(ModbusValue::Bool(val)) => Some(val),
        _ => {
            error_targeted!(MODBUS, "Unexpected value type for register {:?}", address);
            None
        }
    }
}
async fn mb_read_word_helper(state: &AppState,address: &RegisterAddress) -> Option<u16>{
    match mb_read_helper(state,address).await {
        Some(ModbusValue::U16(val)) => Some(val),
        _ => {
            error_targeted!(MODBUS, "Unexpected value type for register {:?}", address);
            None
        }
    }
}

async fn mb_read_dword_helper(state: &AppState,address: &RegisterAddress) -> Option<u32>{
    match tokio::time::timeout(READ_TIMEOUT_DURATION,
                               state.miller_registers.read_u32(address)).await {
        Ok(Some(val)) => {
            Some(val)
        }
        Ok(None) => {
            warn_targeted!(MODBUS, "Failed to retrieve value from cache: {:?}", address);
            None
        }
        Err(_) => {
            warn_targeted!(MODBUS, "Timeout while reading register {:?}", address);
            None
        }
    }
}



pub async fn show_miller_info(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering miller info view");
    let header = build_header_context(&state, AppView::MillerInfo).await;
    MillerInfoTemplate { header }
}

pub async fn show_miller_info_grid(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering miller info grid component");

    // Read all boolean register values from the cache
    let boolean_registers = join_all(MILLER_BOOLEAN_INFO_VIEW.iter().map(|register_meta| async {
        let value = mb_read_bool_helper(&state, &register_meta.address).await;

        BooleanRegisterTemplate {
            meta: register_meta,
            value,
        }
    })).await;

    // Read all analog register values from the cache
    let analog_registers = join_all(MILLER_ANALOG_INFO_VIEW.iter().map(|info| async {
        let raw_value = mb_read_word_helper(&state, &info.meta.address).await;

        AnalogRegisterTemplate {
            raw_value,
            register_info: info,
        }
    })).await;

    // Read weld_state enum register
    let weld_state = mb_read_word_helper(&state, &miller_register_definitions::WELD_STATE.address)
        .await.map(WeldState::from_primitive);

    // Read weld_process enum register
    let weld_process = mb_read_word_helper(&state, &miller_register_definitions::WELD_PROCESS.address)
        .await.map(WeldProcess::from_primitive);

    // Read error registers
    use crate::miller::miller_error_registers::{ErrorReg1, ErrorReg2, ErrorReg3};
    let error_reg_1 = mb_read_word_helper(&state, &miller_register_definitions::ERROR_REG_1.address)
        .await.map(|raw_val|ErrorReg1(raw_val));
    let error_reg_2 = mb_read_word_helper(&state, &miller_register_definitions::ERROR_REG_2.address)
        .await.map(|raw_val|ErrorReg2(raw_val));
    let error_reg_3 = mb_read_word_helper(&state, &miller_register_definitions::ERROR_REG_3.address)
        .await.map(|raw_val|ErrorReg3(raw_val));

    // Get the welder model from machine config
    let welder_model = state.machine_config.read().await.welder_model.clone();

    // Build the error list component
    let error_list = error_list::ErrorListTemplate {
        error_reg_1,
        error_reg_2,
        error_reg_3,
        welder_model,
    };

    // Read version information registers (32-bit dword registers)
    let software_version = mb_read_dword_helper(&state, &miller_register_definitions::SOFTWARE_VERSION.address)
        .await.map(|val| SoftwareUpdateRevision(val));
    let serial_number = mb_read_dword_helper(&state, &miller_register_definitions::SERIAL_NUMBER.address)
        .await.map(|val| SerialNumber(val));
    let app_software_version_pcb_1 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_1.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_2 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_2.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_3 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_3.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_4 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_4.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_5 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_5.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_6 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_6.address)
        .await.map(|val| SubModuleSoftwareVersion(val));
    let app_software_version_pcb_7 = mb_read_dword_helper(&state, &miller_register_definitions::APP_SOFTWARE_VERSION_PCB_7.address)
        .await.map(|val| SubModuleSoftwareVersion(val));

    // Build the version info component
    let version_info = version_info::VersionInfoTemplate {
        software_version,
        serial_number,
        app_software_version_pcb_1,
        app_software_version_pcb_2,
        app_software_version_pcb_3,
        app_software_version_pcb_4,
        app_software_version_pcb_5,
        app_software_version_pcb_6,
        app_software_version_pcb_7,
    };

    let arc_time = mb_read_dword_helper(&state, &miller_register_definitions::ARC_TIME.address)
        .await.map(|val| ArcTime(val));
    let arc_cycles = mb_read_dword_helper(&state, &miller_register_definitions::ARC_CYCLES.address)
        .await.map(|val| ArcCycles(val));

    let statistics_bar = StatisticsBarTemplate {
        arc_time,
        arc_cycles,
    };

    MillerInfoGridTemplate {
        boolean_registers,
        analog_registers,
        weld_state: EnumRegisterTemplate {
            meta: &miller_register_definitions::WELD_STATE,
            value: weld_state,
        },
        weld_process: EnumRegisterTemplate {
            meta: &miller_register_definitions::WELD_PROCESS,
            value: weld_process,
        },
        statistics_bar,
        error_list,
        version_info,
    }
}


pub const MILLER_BOOLEAN_INFO_VIEW: [RegisterMetadata; 27] = [
    miller_register_definitions::PS_UI_DISABLE,
    miller_register_definitions::RMT_TRIGGER_DISABLE,
    miller_register_definitions::GAS_EN,
    miller_register_definitions::COOLER_EN,
    miller_register_definitions::COOLER_TIG_EN,
    miller_register_definitions::COOLER_ERROR_EN,
    miller_register_definitions::TOUCH_SENSE_EN,
    miller_register_definitions::RMS_EN,
    miller_register_definitions::COOLER_DETECTED,
    miller_register_definitions::COOLER_LOAD_DETECTED,
    miller_register_definitions::FOOT_CONTROL_DETECTED,
    miller_register_definitions::RMT_TRIGGER_ENABLED,
    miller_register_definitions::CONTACTOR_OUTPUT_ENABLED,
    miller_register_definitions::GAS_OUTPUT_ENABLED,
    miller_register_definitions::IS_VALID_ARC,
    miller_register_definitions::ARC_LENGTH_CTL_LOCKOUT,
    miller_register_definitions::TOUCH_SENSE_DETECT,
    miller_register_definitions::IS_CE_MODEL,
    miller_register_definitions::IS_STR_MODEL,
    miller_register_definitions::IS_DX_MODEL,
    miller_register_definitions::RMS_HW_PRESENT,
    miller_register_definitions::LOW_LIVE_INPUT,
    miller_register_definitions::HOT_START_SUPPORTED,
    miller_register_definitions::AC_INDEPENDANT_SUPPORTED,
    miller_register_definitions::IS_MIG_VOLT_SENSE_MODEL,
    miller_register_definitions::IS_SYNCROWAVE_MODEL,
    miller_register_definitions::NON_COOLER_SUPPLY_DETECT,
];

use register_view::AnalogRegisterInfo;

pub const MILLER_ANALOG_INFO_VIEW: [AnalogRegisterInfo; 27] = [
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::MAX_AMPS, "A", 0, 0, 1023, 0),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::MIN_DC_AMPS, "A", 0, 0, 31, 0),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::MIN_AC_AMPS, "A", 0, 0, 31, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::COMMANDED_OUTPUT_AMPERAGE, "A", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_AMPERAGE, "A", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_VOLTAGE, "V", 1, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_CURRENT_DC_PULSE_PEAK, "A", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_VOLTAGE_DC_PULSE_PEAK, "V", 1, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_CURRENT_DC_PULSE_BACK, "A", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::OUTPUT_VOLTAGE_DC_PULSE_BACK, "V", 1, 0),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::FAN_OUTPUT, "%", 0, 0, 100, 0),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_1, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_2, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_3, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_4, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_5, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_6, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new_bounded(&miller_register_definitions::TEMPERATURE_7, "°C", 0, -50, 204, -50),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_LINE_CURRENT, "A", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_LINE_VOLTAGE, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_LINE_VOLTAGE_PEAK, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_BUS_VOLTAGE, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::COOLER_OUTPUT_VOLTAGE, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::COOLER_OUTPUT_CURRENT, "A", 1, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::COOLER_BUS_VOLTAGE, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_2_LINE_VOLTAGE_PEAK, "V", 0, 0),
    AnalogRegisterInfo::new(&miller_register_definitions::PRIMARY_2_BUS_VOLTAGE_PEAK, "V", 0, 0),
];



#[derive(Template, WebTemplate)]
#[template(path = "views/miller-info.html")]
pub struct MillerInfoTemplate {
    pub header: HeaderContext,
}
impl ViewTemplate for MillerInfoTemplate { const APP_VIEW_VARIANT: AppView = AppView::MillerInfo; }

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-info/miller-info-grid.html")]
pub struct MillerInfoGridTemplate {
    pub boolean_registers: Vec<BooleanRegisterTemplate>,
    pub analog_registers: Vec<AnalogRegisterTemplate>,
    pub weld_state: EnumRegisterTemplate<WeldState>,
    pub weld_process: EnumRegisterTemplate<WeldProcess>,
    pub statistics_bar: StatisticsBarTemplate,
    pub error_list: error_list::ErrorListTemplate,
    pub version_info: version_info::VersionInfoTemplate,
}
