use std::fmt::Display;
use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use serde_json::json;
use super::weld_profile::ProfileListEntry;
use axum::http::{HeaderMap, HeaderValue};
use crate::error_targeted;

pub struct HxTrigger {
    pub event: &'static str,
    pub target: &'static str,
}
impl HxTrigger {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            self.event: { "target": self.target }
        })
    }

    pub fn list_to_json(targets: &[HxTrigger]) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::with_capacity(targets.len());
        for t in targets {
            map.insert(
                t.event.to_string(), json!({ "target": t.target })
            );
        }
        map
    }
}

pub const RELOAD_LIST_EVENT: HxTrigger = HxTrigger {
    event: "reload-profile-list",
    target: "#load-profile-list",
};
pub const CLOSE_MODAL_EVENT: HxTrigger = HxTrigger {
    event: "close-modal",
    target: "#global-modal-layer"
};
pub const RELOAD_METADATA_EVENT: HxTrigger = HxTrigger {
    event: "reload-profile-metadata",
    target: "#profile-metadata"
};

#[derive(Template)]
#[template(path = "components/file-system/result-feedback.html")]
pub struct ProfileFsOpResult<E: Display> {
    pub result: Result<String, E>,
    pub close_modal: bool,
    pub reload_metadata: bool,
    pub retarget: Option<&'static str>,
}
impl<E: Display> ProfileFsOpResult<E> {
    
    pub const DEFAULT_TARGET: &'static str = "#profile-fs-op-status";
    
    pub fn new_ok_str(ok_string: String) -> Self {
        Self {
            result: Ok(ok_string),
            close_modal: true,
            reload_metadata: false,
            retarget: None,
        }
    }
    
}
impl ProfileFsOpResult<String>{
    pub fn new_err_str(err_string: String) -> Self {
        Self {
            result: Err(err_string),
            close_modal: false,
            reload_metadata: false,
            retarget: None,
        }
    }
}

impl<E: Display> IntoResponse for ProfileFsOpResult<E> {
    fn into_response(self) -> axum::response::Response {
        // 1. Prepare Headers
        let mut headers = HeaderMap::new();
        let mut targets = Vec::with_capacity(2);

        if self.close_modal {
            targets.push(CLOSE_MODAL_EVENT);
        }
        if self.reload_metadata {
            targets.push(RELOAD_METADATA_EVENT);
        }

        if !targets.is_empty() {
            let json_map = HxTrigger::list_to_json(&targets);

            if let Ok(json_string) = serde_json::to_string(&json_map) {
                if let Ok(header_value) = HeaderValue::from_str(&json_string) {
                    headers.insert("HX-Trigger", header_value);
                }
            }
        }
        
        if let Some(target) = self.retarget {
            headers.insert("HX-Retarget", HeaderValue::from_static(target));
        }


        match self.render() {
            Ok(html_string) => {
                (headers, axum::response::Html(html_string)).into_response()
            },
            Err(err) => {
                error_targeted!(HTTP, "Failed to render template: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    headers,
                    format!("Template error: {}", err)
                ).into_response()
            }
        }
    }
}


#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/save-as-modal.html")]
pub struct SaveAsModalTemplate {
    pub current_name: Option<String>,
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/save-as-profile-list.html")]
pub struct SaveAsProfileListTemplate {
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-modal.html")]
pub struct LoadModalTemplate {}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-profile-list.html")]
pub struct LoadProfileListTemplate {
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-preview.html")]
pub struct LoadPreviewTemplate(pub Result<LoadPreviewWindow, String>);

pub struct LoadPreviewWindow {
    pub name: String,
    pub description: String,
}


#[derive(Template)]
#[template(path = "components/file-system/delete-profile-feedback.html")]
pub struct ProfileDeleteTemplate {
    pub name: String,
    pub result: Result<(), String>,
}



impl IntoResponse for ProfileDeleteTemplate {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();

        if self.result.is_ok() {
            let json_trigger = HxTrigger::to_json(&RELOAD_LIST_EVENT);
            if let Ok(header_value) = HeaderValue::from_str(&json_trigger.to_string()) {
                headers.insert("HX-Trigger", header_value);
            }
        }

        match self.render() {
            Ok(html_string) => (headers, axum::response::Html(html_string)).into_response(),
            Err(err) => {
                error_targeted!(HTTP, "Failed to render delete template: {}", err);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    headers,
                    "Template error",
                )
                    .into_response()
            }
        }
    }
}