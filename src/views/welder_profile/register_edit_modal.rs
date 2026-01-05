use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use crate::miller::analog_register::AnalogRegisterInfo;
use super::special_case_registers::{TungstenPreset, ElectrodePolarity, WaveShape, PostFlowTime};
use num_enum::TryFromPrimitive;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/boolean-edit-modal.html")]
pub struct BooleanEditModalTemplate {
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<bool>,
    pub register_name: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/analog-edit-modal.html")]
pub struct AnalogEditModalTemplate {
    pub register_info: &'static AnalogRegisterInfo,
    pub current_value: Option<u16>,
    pub register_name: String,
}

impl AnalogEditModalTemplate {
    pub fn current_semantic_value(&self) -> Option<f32> {
        self.current_value.map(|val| self.register_info.convert_from_raw(val))
    }

    pub fn semantic_min(&self) -> f32 {
        self.register_info.convert_from_raw(self.register_info.min_value)
    }

    pub fn semantic_max(&self) -> f32 {
        self.register_info.convert_from_raw(self.register_info.max_value)
    }

    pub fn step(&self) -> f32 {
        1.0 / self.register_info.scale as f32
    }
}

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
        let options = TungstenPreset::all_variants()
            .iter()
            .map(|(val, name)| (*val, name.to_string()))
            .collect();
        Self { meta, current_value, register_name, options }
    }

    pub fn new_polarity(meta: &'static RegisterMetadata, current_value: Option<u16>, register_name: String) -> Self {
        let options = ElectrodePolarity::all_variants()
            .iter()
            .map(|(val, name)| (*val, name.to_string()))
            .collect();
        Self { meta, current_value, register_name, options }
    }

    pub fn new_wave_shape(meta: &'static RegisterMetadata, current_value: Option<u16>, register_name: String) -> Self {
        let options = WaveShape::all_variants()
            .iter()
            .map(|(val, name)| (*val, name.to_string()))
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

impl PostflowEditModalTemplate {
    pub fn current_display_value(&self) -> String {
        match self.current_value.and_then(|v| PostFlowTime::from_raw(v).ok()) {
            Some(time) => time.display_value(),
            None => "---".to_string(),
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
