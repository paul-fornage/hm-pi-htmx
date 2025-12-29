



pub mod boolean_register_view;
pub mod register_details_modal;

use askama::Template;
use askama_web::WebTemplate;
use crate::views::{AppView, ViewTemplate};
use boolean_register_view::BooleanRegisterTemplate;
use crate::modbus::RegisterMetadata;
use crate::miller::miller_register_definitions;

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
