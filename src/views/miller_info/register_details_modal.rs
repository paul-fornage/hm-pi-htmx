
use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::fmt;
use crate::{debug_targeted, info_targeted, trace_targeted, warn_targeted};
use crate::miller::miller_register_definitions::{get_register_metadata, MILLER_REGISTERS};
use crate::modbus::RegisterMetadata;



#[derive(Template)]
#[template(path = "components/register-info-modal.html")]
pub struct RegisterModalTemplate {
    pub meta: &'static RegisterMetadata,
}

pub async fn modal_handler(Path(name): Path<String>) -> impl IntoResponse {
    info_targeted!(HTTP, "Modal handler called for register: {}", name);

    match get_register_metadata(&name) {
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