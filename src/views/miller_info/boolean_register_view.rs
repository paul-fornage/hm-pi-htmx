use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;

#[derive(Template, WebTemplate)]
#[template(path = "components/boolean-read-only-register.html")]
pub struct BooleanRegisterTemplate {
    pub meta: &'static RegisterMetadata,
    pub value: Option<bool>,
}

impl BooleanRegisterTemplate {}


