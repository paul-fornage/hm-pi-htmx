use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use crate::analog_register::AnalogRegisterInfo;

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/editable-boolean-register.html")]
pub struct EditableBooleanRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<bool>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-config/editable-analog-register.html")]
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
