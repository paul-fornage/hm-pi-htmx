
use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::fmt;
use crate::{debug_targeted, info_targeted, warn_targeted};
use crate::miller::miller_register_definitions::MILLER_REGISTERS;
use crate::modbus::RegisterMetadata;


fn find_metadata_by_name(name: &str) -> Option<&'static RegisterMetadata> {
    // TODO: Hashmap? IDK only ~140 registers and popup is not a hot loop
    MILLER_REGISTERS.iter().find(|reg| reg.name == name)
}


#[derive(Template)]
#[template(path = "components/register-info-modal.html")]
pub struct RegisterModalTemplate {
    pub meta: &'static RegisterMetadata,
}

pub async fn modal_handler(Path(name): Path<String>) -> impl IntoResponse {
    info_targeted!(HTTP, "Modal handler called for register: {}", name);

    match find_metadata_by_name(&name) {
        Some(meta) => {
            debug_targeted!(HTTP, "Found metadata for register: {}", name);
            let template = RegisterModalTemplate { meta };
            Html(template.render().unwrap())
        }
        None => {
            warn_targeted!(HTTP, "Register not found: {}", name);
            Html("<div>Error: Register not found</div>".to_string())
        }
    }
}