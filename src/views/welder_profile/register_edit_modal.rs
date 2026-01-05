use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;

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
    pub meta: &'static RegisterMetadata,
    pub current_value: Option<u16>,
    pub register_name: String,
    pub unit: &'static str,
    pub scale: u16,
    pub precision: u16,
    pub min_value: u16,
    pub max_value: u16,
}

impl AnalogEditModalTemplate {
    pub fn current_semantic_value(&self) -> Option<f32> {
        self.current_value.map(|val| val as f32 / self.scale as f32)
    }

    pub fn semantic_min(&self) -> f32 {
        self.min_value as f32 / self.scale as f32
    }

    pub fn semantic_max(&self) -> f32 {
        self.max_value as f32 / self.scale as f32
    }

    pub fn step(&self) -> f32 {
        1.0 / self.scale as f32
    }
}
