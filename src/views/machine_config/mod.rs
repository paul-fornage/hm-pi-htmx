use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::State, response::{IntoResponse, Response}, Form};
use crate::views::{AppView, ViewTemplate};
use crate::{AppState, info_targeted, error_targeted};
use crate::logging::LogTarget;
use crate::miller::miller_register_types::WelderModel;

#[derive(Template, WebTemplate)]
#[template(path = "views/machine-config.html")]
pub struct MachineConfigTemplate {
    pub current_model: WelderModel,
    pub save_status: Option<Result<(), crate::error::Error>>,
}

impl ViewTemplate for MachineConfigTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::MachineConfig;
}

#[derive(Template)]
#[template(path = "components/machine_config/status-message.html")]
pub struct StatusMessageTemplate {
    pub save_status: Option<Result<(), crate::error::Error>>,
}

impl IntoResponse for StatusMessageTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response(),
        }
    }
}

pub async fn show_machine_config(State(state): State<AppState>) -> impl IntoResponse {
    info_targeted!(HTTP, "Rendering machine config view");

    let config = state.machine_config.read().await;
    MachineConfigTemplate {
        current_model: config.welder_model.clone(),
        save_status: None,
    }
}

#[derive(serde::Deserialize)]
pub struct MachineConfigForm {
    pub welder_model: String,
}

pub async fn save_machine_config(
    State(state): State<AppState>,
    Form(form): Form<MachineConfigForm>,
) -> Response {
    info_targeted!(HTTP, "Saving machine config: {:?}", form.welder_model);

    // Parse the welder model
    let welder_model = match parse_welder_model(&form.welder_model) {
        Some(model) => model,
        None => {
            error_targeted!(HTTP, "Invalid welder model: {}", form.welder_model);
            return StatusMessageTemplate {
                save_status: Some(Err(crate::error::Error::InvalidWelderModel)),
            }.into_response();
        }
    };

    // Create new config
    let new_config = crate::machine_config::MachineConfig {
        welder_model: welder_model.clone(),
    };

    // Save to disk
    let result = match new_config.save(crate::machine_config::MACHINE_CONFIG_PATH) {
        Ok(_) => {
            info_targeted!(HTTP, "Config saved to disk");

            // Reload from disk to verify
            match crate::machine_config::MachineConfig::load(crate::machine_config::MACHINE_CONFIG_PATH) {
                Ok(loaded_config) => {
                    // Update in-memory state
                    *state.machine_config.write().await = loaded_config.clone();
                    info_targeted!(HTTP, "Config reloaded from disk and updated in memory");

                    StatusMessageTemplate {
                        save_status: Some(Ok(())),
                    }
                }
                Err(e) => {
                    error_targeted!(HTTP, "Failed to reload config: {:?}", e);
                    StatusMessageTemplate {
                        save_status: Some(Err(e)),
                    }
                }
            }
        }
        Err(e) => {
            error_targeted!(HTTP, "Failed to save config: {:?}", e);
            StatusMessageTemplate {
                save_status: Some(Err(e)),
            }
        }
    };

    result.into_response()
}

fn parse_welder_model(s: &str) -> Option<WelderModel> {
    match s {
        "Dynasty210" => Some(WelderModel::Dynasty210),
        "Dynasty280" => Some(WelderModel::Dynasty280),
        "Dynasty400" => Some(WelderModel::Dynasty400),
        "Dynasty800" => Some(WelderModel::Dynasty800),
        "Maxstar210" => Some(WelderModel::Maxstar210),
        "Maxstar280" => Some(WelderModel::Maxstar280),
        "Maxstar400" => Some(WelderModel::Maxstar400),
        "Maxstar800" => Some(WelderModel::Maxstar800),
        "Syncrowave300" => Some(WelderModel::Syncrowave300),
        "Syncrowave400" => Some(WelderModel::Syncrowave400),
        _ => None,
    }
}

pub fn welder_model_to_string(model: &WelderModel) -> &'static str {
    match model {
        WelderModel::Dynasty210 => "Dynasty210",
        WelderModel::Dynasty280 => "Dynasty280",
        WelderModel::Dynasty400 => "Dynasty400",
        WelderModel::Dynasty800 => "Dynasty800",
        WelderModel::Maxstar210 => "Maxstar210",
        WelderModel::Maxstar280 => "Maxstar280",
        WelderModel::Maxstar400 => "Maxstar400",
        WelderModel::Maxstar800 => "Maxstar800",
        WelderModel::Syncrowave300 => "Syncrowave300",
        WelderModel::Syncrowave400 => "Syncrowave400",
    }
}

pub fn welder_model_display(model: &WelderModel) -> &'static str {
    match model {
        WelderModel::Dynasty210 => "Dynasty 210",
        WelderModel::Dynasty280 => "Dynasty 280",
        WelderModel::Dynasty400 => "Dynasty 400",
        WelderModel::Dynasty800 => "Dynasty 800",
        WelderModel::Maxstar210 => "Maxstar 210",
        WelderModel::Maxstar280 => "Maxstar 280",
        WelderModel::Maxstar400 => "Maxstar 400",
        WelderModel::Maxstar800 => "Maxstar 800",
        WelderModel::Syncrowave300 => "Syncrowave 300",
        WelderModel::Syncrowave400 => "Syncrowave 400",
    }
}

pub const ALL_WELDER_MODELS: &[WelderModel] = &[
    WelderModel::Dynasty210,
    WelderModel::Dynasty280,
    WelderModel::Dynasty400,
    WelderModel::Dynasty800,
    WelderModel::Maxstar210,
    WelderModel::Maxstar280,
    WelderModel::Maxstar400,
    WelderModel::Maxstar800,
    WelderModel::Syncrowave300,
    WelderModel::Syncrowave400,
];
