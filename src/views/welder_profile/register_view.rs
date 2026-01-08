use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use crate::analog_register::AnalogRegisterInfo;
use super::special_case_registers::{TungstenPreset, ElectrodePolarity, WaveShape, PostFlowTime};
use num_enum::TryFromPrimitive;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-boolean-register.html")]
pub struct EditableBooleanRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<bool>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-analog-register.html")]
pub struct EditableAnalogRegister {
    pub register_info: &'static AnalogRegisterInfo,
    pub value: Option<u16>,
}

impl EditableAnalogRegister {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        self.register_info.formatted_value(self.value)
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-enum-register.html")]
pub struct EditableEnumRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<u16>,
    pub display_value: String,
}

impl EditableEnumRegister {
    pub fn new_tungsten(meta: &'static RegisterMetadata, value: Option<u16>) -> Self {
        let display_value = match value.and_then(|v| TungstenPreset::try_from_primitive(v).ok()) {
            Some(preset) => preset.to_string(),
            None => "---".to_string(),
        };
        Self { meta, value, display_value }
    }

    pub fn new_polarity(meta: &'static RegisterMetadata, value: Option<u16>) -> Self {
        let display_value = match value.and_then(|v| ElectrodePolarity::try_from_primitive(v).ok()) {
            Some(polarity) => polarity.to_string(),
            None => "---".to_string(),
        };
        Self { meta, value, display_value }
    }

    pub fn new_wave_shape(meta: &'static RegisterMetadata, value: Option<u16>) -> Self {
        let display_value = match value.and_then(|v| WaveShape::try_from_primitive(v).ok()) {
            Some(shape) => shape.to_string(),
            None => "---".to_string(),
        };
        Self { meta, value, display_value }
    }

    pub fn new_postflow(meta: &'static RegisterMetadata, value: Option<u16>) -> Self {
        let display_value = match value.and_then(|v| PostFlowTime::from_raw(v).ok()) {
            Some(time) => time.display_value(),
            None => "---".to_string(),
        };
        Self { meta, value, display_value }
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-postflow-register.html")]
pub struct EditablePostflowRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<u16>,
}

impl EditablePostflowRegister {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        match self.value.and_then(|v| PostFlowTime::from_raw(v).ok()) {
            Some(time) => time.display_value(),
            None => "---".to_string(),
        }
    }
}
