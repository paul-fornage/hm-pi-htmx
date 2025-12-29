
use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::fmt;
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

async fn modal_handler(Path(name): Path<String>) -> impl IntoResponse {
    match find_metadata_by_name(&name) {
        Some(meta) => {
            let template = RegisterModalTemplate { meta };
            Html(template.render().unwrap())
        }
        None => Html("<div>Error: Register not found</div>".to_string()),
    }
}