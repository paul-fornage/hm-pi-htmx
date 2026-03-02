use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::State, response::{IntoResponse, Response}, routing::{get, post}, Form, Router};
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{AppState, error_targeted, info_targeted, warn_targeted};
use crate::logging::LogTarget;
use crate::miller::miller_register_definitions::PS_UI_DISABLE;
use crate::miller::miller_register_types::WelderModel;
use crate::modbus::ModbusState;
use std::sync::atomic::Ordering;

#[derive(Template, WebTemplate)]
#[template(path = "views/machine-config.html")]
pub struct MachineConfigTemplate {
    pub header: HeaderContext,
    pub current_model: WelderModel,
    pub udp_logging_port: u16,
    pub ps_ui_disable: bool,
    pub save_status: Option<Result<(), crate::error::HmPiError>>,
}

impl ViewTemplate for MachineConfigTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::MachineConfig;
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(AppView::MachineConfig.url(), get(show_machine_config))
        .route("/machine-config/save", post(save_machine_config))
}

#[derive(Template)]
#[template(path = "components/machine_config/status-message.html")]
pub struct StatusMessageTemplate {
    pub save_status: Option<Result<(), crate::error::HmPiError>>,
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
    let header = build_header_context(&state, AppView::MachineConfig).await;
    MachineConfigTemplate {
        header,
        current_model: config.welder_model.clone(),
        udp_logging_port: config.udp_logging_port,
        ps_ui_disable: config.ps_ui_disable,
        save_status: None,
    }
}

#[derive(serde::Deserialize)]
pub struct MachineConfigForm {
    pub welder_model: WelderModel,
    pub udp_logging_port: u16,
    pub ps_ui_disable: bool,
}

pub async fn save_machine_config(
    State(state): State<AppState>,
    Form(form): Form<MachineConfigForm>,
) -> Response {
    info_targeted!(
        HTTP,
        "Saving machine config: {}, udp_logging_port: {}, ps_ui_disable: {}",
        form.welder_model,
        form.udp_logging_port,
        form.ps_ui_disable
    );

    // Create new config
    let new_config = crate::machine_config::MachineConfig {
        welder_model: form.welder_model.clone(),
        udp_logging_port: form.udp_logging_port,
        ps_ui_disable: form.ps_ui_disable,
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
                    let ui_disabled = loaded_config.ps_ui_disable;
                    state.ps_ui_disable.store(ui_disabled, Ordering::Release);
                    info_targeted!(HTTP, "Config reloaded from disk and updated in memory");

                    let connection_state = state.miller_registers
                        .get_connection_state()
                        .await
                        .unwrap_or(ModbusState::Disconnected);
                    if connection_state == ModbusState::Connected {
                        match state.miller_registers
                            .write_coil(PS_UI_DISABLE.address.address, ui_disabled)
                            .await {
                            Ok(()) => {
                                info_targeted!(HTTP, "Applied PS_UI_DISABLE {ui_disabled} to connected welder");
                            }
                            Err(e) => {
                                warn_targeted!(HTTP, "Failed to apply PS_UI_DISABLE to welder: {:?}", e);
                            }
                        }
                    } else {
                        warn_targeted!(HTTP, "Welder not connected, cannot apply PS_UI_DISABLE");
                    }

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
