use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use crate::analog_register::AnalogRegisterInfo;

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/boolean-edit-modal.html")]
pub struct BooleanEditModalTemplate {
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<bool>,
    pub register_name: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/analog-edit-modal.html")]
pub struct AnalogEditModalTemplate {
    pub register_info: &'static AnalogRegisterInfo,
    pub current_value: Option<u16>,
    pub register_name: String,
}

impl AnalogEditModalTemplate {
    pub fn formatted_current_value(&self) -> String {
        self.register_info.formatted_value(self.current_value)
    }

    pub fn semantic_value(&self) -> String {
        match self.current_value {
            Some(raw) => {
                let semantic = self.register_info.convert_to_semantic(raw);
                format!("{:.prec$}", semantic, prec = self.register_info.decimal_places)
            }
            None => "".to_string(),
        }
    }

    pub fn min_value(&self) -> String {
        match self.register_info.semantic_min {
            Some(min) => format!("{:.prec$}", min, prec = self.register_info.decimal_places),
            None => "".to_string(),
        }
    }

    pub fn max_value(&self) -> String {
        match self.register_info.semantic_max {
            Some(max) => format!("{:.prec$}", max, prec = self.register_info.decimal_places),
            None => "".to_string(),
        }
    }
}
