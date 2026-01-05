use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-boolean-register.html")]
pub struct EditableBooleanRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<bool>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/editable-analog-register.html")]
pub struct EditableAnalogRegister {
    pub meta: &'static RegisterMetadata,
    pub value: Option<u16>,
    pub unit: &'static str,
    pub scale: u16,
    pub precision: u16,
    pub min_value: u16,
    pub max_value: u16,
}

impl EditableAnalogRegister {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        match self.value {
            Some(val) => {
                let scaled = val as f32 / self.scale as f32;
                format!("{:.*} {}", self.precision as usize, scaled, self.unit)
            }
            None => String::from("---")
        }
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
