



pub mod boolean_register_view;
pub mod register_details_modal;

use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use log::error;
use crate::views::{AppView, ViewTemplate};
use boolean_register_view::BooleanRegisterTemplate;
use crate::modbus::RegisterMetadata;
use crate::miller::miller_register_definitions;
use crate::{debug_targeted, error_targeted, trace_targeted, AppState};

pub async fn show_miller_info(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering miller info view");

    use crate::modbus::ModbusValue;

    // Read all boolean register values from the cache
    let mut boolean_registers = Vec::with_capacity(MILLER_BOOLEAN_INFO_VIEW.len());

    for register_meta in MILLER_BOOLEAN_INFO_VIEW.iter() {
        // Read the value from the cache, defaulting to false if not yet cached
        let value = match state.miller_registers.read(&register_meta.address).await {
            Some(ModbusValue::Bool(val)) => Some(val),
            Some(ModbusValue::U16(val)) => {
                error_targeted!(MODBUS, "Unexpected value type for register {}: {:?}", register_meta.name, ModbusValue::U16(val));
                None
            }
            _ => {
                debug_targeted!(MODBUS, "Failed to retrieve value from cache: {:?}", register_meta.address);
                None
            }
        };

        boolean_registers.push(BooleanRegisterTemplate {
            meta: register_meta,
            value,
        });
    }

    MillerInfoTemplate { boolean_registers }
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

#[derive(Template, WebTemplate)]
#[template(path = "views/miller-info.html")]
pub struct MillerInfoTemplate {
    pub boolean_registers: Vec<BooleanRegisterTemplate>,

}

impl ViewTemplate for MillerInfoTemplate { const APP_VIEW_VARIANT: AppView = AppView::MillerInfo; }

#[derive(Template, WebTemplate)]
#[template(path = "components/miller-info-grid.html")]
pub struct MillerInfoGridTemplate {
    pub boolean_registers: Vec<BooleanRegisterTemplate>,
}
