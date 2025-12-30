



pub mod register_view;
pub mod register_details_modal;

use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use log::{error, warn};
use crate::views::{AppView, ViewTemplate};
use crate::modbus::RegisterMetadata;
use crate::miller::miller_register_definitions;
use crate::{debug_targeted, error_targeted, trace_targeted, warn_targeted, AppState};
use crate::miller::miller_register_types::{SerialNumber, SoftwareUpdateRevision, SubModuleSoftwareVersion, WeldProcess, WeldState};
use register_view::{BooleanRegisterTemplate, AnalogRegisterTemplate, EnumRegisterTemplate};
use futures::future::join_all;
use num_enum::FromPrimitive;

const READ_TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_millis(100);

pub async fn show_miller_info(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering miller info view");

    use crate::modbus::ModbusValue;

    // Read all boolean register values from the cache
    let boolean_registers = join_all(MILLER_BOOLEAN_INFO_VIEW.iter().map(|register_meta| async {
        let value = match tokio::time::timeout(READ_TIMEOUT_DURATION,
                                               state.miller_registers.read(&register_meta.address)).await {
            Ok(Some(ModbusValue::Bool(val))) => Some(val),
            Ok(Some(val)) => {
                error_targeted!(MODBUS, "Unexpected value type for register {}: {:?}", register_meta.name, val);
                None
            }
            Ok(_) => {
                debug_targeted!(MODBUS, "Failed to retrieve value from cache: {:?}", register_meta.address);
                None
            }
            Err(_) => {
                warn_targeted!(MODBUS, "Timeout while reading register {}", register_meta.name);
                None
            },
        };

        BooleanRegisterTemplate {
            meta: register_meta,
            value,
        }
    }))
        .await;

    // Read all analog register values from the cache
    let analog_registers = join_all(MILLER_ANALOG_INFO_VIEW.iter().map(|info| async {
        let raw_value = match tokio::time::timeout(READ_TIMEOUT_DURATION,
                                                   state.miller_registers.read(&info.meta.address)).await {
            Ok(Some(ModbusValue::U16(val))) => Some(val),
            Ok(Some(val)) => {
                error_targeted!(MODBUS, "Unexpected value type for register {}: {:?}", info.meta.name, val);
                None
            }
            Ok(_) => {
                debug_targeted!(MODBUS, "Failed to retrieve value from cache: {:?}", info.meta.address);
                None
            }
            Err(_) => {
                warn_targeted!(MODBUS, "Timeout while reading register {}", info.meta.name);
                None
            }
        };

        AnalogRegisterTemplate {
            raw_value,
            register_info: info,
        }
    }))
        .await;

    // Read weld_state enum register
    let weld_state = match tokio::time::timeout(READ_TIMEOUT_DURATION,
                                                state.miller_registers.read(&miller_register_definitions::WELD_STATE.address)).await {
        Ok(Some(ModbusValue::U16(val))) => Some(WeldState::from_primitive(val)),
        Ok(Some(val)) => {
            error_targeted!(MODBUS, "Unexpected value type for WELD_STATE: {:?}", val);
            None
        }
        Ok(_) => {
            debug_targeted!(MODBUS, "Failed to retrieve WELD_STATE from cache");
            None
        }
        Err(_) => {
            warn_targeted!(MODBUS, "Timeout while reading WELD_STATE");
            None
        }
    };

    // Read weld_process enum register
    let weld_process = match tokio::time::timeout(READ_TIMEOUT_DURATION,
                                                  state.miller_registers.read(&miller_register_definitions::WELD_PROCESS.address)).await {
        Ok(Some(ModbusValue::U16(val))) => Some(WeldProcess::from_primitive(val)),
        Ok(Some(val)) => {
            error_targeted!(MODBUS, "Unexpected value type for WELD_PROCESS: {:?}", val);
            None
        }
        Ok(_) => {
            debug_targeted!(MODBUS, "Failed to retrieve WELD_PROCESS from cache");
            None
        }
        Err(_) => {
            warn_targeted!(MODBUS, "Timeout while reading WELD_PROCESS");
            None
        }
    };

    MillerInfoTemplate {
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
    pub boolean_registers: Vec<BooleanRegisterTemplate>,
    pub analog_registers: Vec<AnalogRegisterTemplate>,
    pub weld_state: EnumRegisterTemplate<WeldState>,
    pub weld_process: EnumRegisterTemplate<WeldProcess>,

    // pub error_reg_1: MillerErrorReg,
    // pub error_reg_2: MillerErrorReg,
    // pub error_reg_3: MillerErrorReg,
    //
    // pub software_version: SoftwareUpdateRevision,
    // pub serial_number: SerialNumber,
    // pub app_software_version_pcb_1: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_2: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_3: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_4: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_5: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_6: SubModuleSoftwareVersion,
    // pub app_software_version_pcb_7: SubModuleSoftwareVersion,
}

impl ViewTemplate for MillerInfoTemplate { const APP_VIEW_VARIANT: AppView = AppView::MillerInfo; }


