use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use crate::views::shared::analog_register::AnalogRegisterInfo;
use crate::views::shared::boolean_register::BooleanRegisterInfo;

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/boolean-edit-modal.html")]
pub struct BooleanEditModalTemplate {
    pub register_info: &'static BooleanRegisterInfo,
    pub current_value: Option<bool>,
    pub register_name: String,
    pub base_url: &'static str,
}

impl BooleanEditModalTemplate {
    pub fn current_display_value(&self) -> &'static str {
        self.register_info.render_value(self.current_value)
    }
}


#[derive(Template, WebTemplate)]
#[template(path = "components/shared/analog-edit-modal.html")]
pub struct AnalogEditModalTemplate {
    pub register_info: &'static AnalogRegisterInfo,
    pub current_value: Option<u16>,
    pub register_name: String,
    pub base_url: &'static str,
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

    pub fn current_display_value(&self) -> String {
        self.register_info.formatted_value(self.current_value)
    }

    pub fn display_min_value(&self) -> String {
        self.register_info.formatted_value(Some(self.register_info.min_value))
    }

    pub fn display_max_value(&self) -> String {
        self.register_info.formatted_value(Some(self.register_info.max_value))
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/write-error-modal.html")]
pub struct WriteErrorModalTemplate {
    pub title: String,
    pub message: String,
}
