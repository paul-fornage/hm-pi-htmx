use crate::views::shared::analog_dword_register::AnalogDwordRegisterInfo;
use crate::views::shared::analog_register::AnalogRegisterInfo;
use crate::views::shared::boolean_register::BooleanRegisterInfo;
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/editable-analog-register.html")]
pub struct EditableAnalogRegister {
    pub register_info: &'static AnalogRegisterInfo,
    pub value: Option<u16>,
    pub base_url: &'static str,
}

impl EditableAnalogRegister {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        self.register_info.formatted_value(self.value)
    }
    pub fn preview_formatted_value(&self) -> String {
        self.register_info.preview_formatted_value(self.value)
    }
    pub fn open_modal_url(&self) -> String { format!("{}/edit/{}", self.base_url, self.register_info.meta.name) }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/editable-analog-register.html")]
pub struct EditableDwordAnalogRegister {
    pub register_info: &'static AnalogDwordRegisterInfo,
    pub value: Option<u32>,
    pub base_url: &'static str,
}

impl EditableDwordAnalogRegister {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        self.register_info.formatted_value(self.value)
    }
    pub fn preview_formatted_value(&self) -> String {
        self.register_info.preview_formatted_value(self.value)
    }
    pub fn open_modal_url(&self) -> String { format!("{}/edit/{}", self.base_url, self.register_info.get_meta().name) }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/editable-boolean-register.html")]
pub struct EditableBooleanRegister {
    pub register_info: &'static BooleanRegisterInfo,
    pub value: Option<bool>,
    pub base_url: &'static str,
}
impl EditableBooleanRegister {
    pub fn as_string(&self) -> &'static str {
        self.register_info.render_value(self.value)
    }
    pub fn open_modal_url(&self) -> String { format!("{}/edit/{}", self.base_url, self.register_info.meta.name) }
}
