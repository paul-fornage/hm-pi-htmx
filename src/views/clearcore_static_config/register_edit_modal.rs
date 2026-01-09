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

    pub fn semantic_value(&self) -> String {
        self.register_info.formatted_value(self.current_value)
    }

    pub fn min_value(&self) -> String {
        self.register_info.formatted_value(Some(self.register_info.min_value))
    }

    pub fn max_value(&self) -> String {
        self.register_info.formatted_value(Some(self.register_info.max_value))
    }
}
