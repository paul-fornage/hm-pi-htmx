use super::special_case_registers::{ElectrodePolarity, PostFlowTime, TungstenPreset, WaveShape};
use crate::modbus::RegisterMetadata;
use askama::Template;
use askama_web::WebTemplate;
use strum::VariantArray;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/enum-edit-modal.html")]
pub struct EnumEditModalTemplate {
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<u16>,
    pub register_name: String,
    pub options: Vec<(u16, String)>,
}

impl EnumEditModalTemplate {
    pub fn new_tungsten(meta: &'static RegisterMetadata, current_value: Option<u16>, register_name: String) -> Self {
        let options = TungstenPreset::VARIANTS
            .iter()
            .map(|variant| (u16::from(*variant), variant.to_string()))
            .collect();
        Self { meta, current_value, register_name, options }
    }

    pub fn new_polarity(meta: &'static RegisterMetadata, current_value: Option<u16>, register_name: String) -> Self {
        let options = ElectrodePolarity::VARIANTS
            .iter()
            .map(|variant| (u16::from(*variant), variant.to_string()))
            .collect();
        Self { meta, current_value, register_name, options }
    }

    pub fn new_wave_shape(meta: &'static RegisterMetadata, current_value: Option<u16>, register_name: String) -> Self {
        let options = WaveShape::VARIANTS
            .iter()
            .map(|variant| (u16::from(*variant), variant.to_string()))
            .collect();
        Self { meta, current_value, register_name, options }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/postflow-edit-modal.html")]
pub struct PostflowEditModalTemplate {
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<u16>,
    pub register_name: String,
}

pub enum PostFlowEnum {
    Off,
    Manual(u8),
    Auto,
}
impl PostFlowEnum {
    pub fn as_u16(&self) -> u16 {
        match self {
            PostFlowEnum::Off => 0,
            PostFlowEnum::Auto => 51,
            PostFlowEnum::Manual(v) => *v as u16,
        }
    }
    
    pub fn is_off(&self) -> bool {
        matches!(self, PostFlowEnum::Off)
    }
    pub fn is_auto(&self) -> bool {
        matches!(self, PostFlowEnum::Auto)
    }
    pub fn is_manual(&self) -> bool {
        matches!(self, PostFlowEnum::Manual(_))
    }
    pub fn as_manual(&self) -> Option<u8> {
        match self {
            PostFlowEnum::Manual(v) => Some(*v),
            _ => None,
        }
    }
}

impl PostflowEditModalTemplate {
    pub fn current_display_value(&self) -> String {
        match self.current_value.and_then(|v| PostFlowTime::from_raw(v).ok()) {
            Some(raw_u16) => raw_u16.display_value(),
            None => "---".to_string(),
        }
    }
    
    pub fn as_enum(&self) -> PostFlowEnum{
        match self.current_value {
            Some(0) => PostFlowEnum::Off,
            Some(51) => PostFlowEnum::Auto,
            Some(v) if v < 51 => PostFlowEnum::Manual(v as u8),
            Some(_) | None => PostFlowEnum::Off,
        }
    }
    
    pub fn initial_manual_mem(&self) -> u8 {
        match self.current_value {
            Some(v) if v < 51 => v as u8,
            _ => 5,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/polarity-edit-modal.html")]
pub struct PolarityEditModalTemplate {
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<u16>,
    pub register_name: String,
}
