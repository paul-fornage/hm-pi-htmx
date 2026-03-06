use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use axum::http::{HeaderMap, HeaderValue};

use crate::hx_trigger::HxTrigger;
use crate::paths::usb_drives::usb_mountpoints;
use crate::views::{AppView, build_header_context};
use crate::{debug_targeted, error_targeted, info_targeted, warn_targeted, AppState};

use super::templates::{
    UsbTransferConfirmModalTemplate,
    UsbTransferLocalListTemplate,
    UsbTransferTemplate,
    UsbTransferToast,
    UsbTransferToastTemplate,
    UsbTransferUsbListTemplate,
};
use super::transfer::{
    destination_exists,
    execute_bulk_copy,
    execute_copy,
    list_local_sections,
    list_usb_sections,
    prepare_bulk_copy,
    prepare_copy,
    subdir_label,
};
use super::types::{ReloadTarget, UsbTransferForm};

const RELOAD_LOCAL_EVENT: HxTrigger = HxTrigger {
    event: "usb-transfer-reload-local",
    target: "#usb-local-list",
};
const RELOAD_USB_EVENT: HxTrigger = HxTrigger {
    event: "usb-transfer-reload-usb",
    target: "#usb-drive-list",
};

pub fn routes() -> Router<AppState> {
    let page = AppView::UsbTransfer;
    Router::new()
        .route(page.url(), get(show_usb_transfer))
        .route(&page.url_with_path("/local-list"), get(local_list))
        .route(&page.url_with_path("/usb-list"), get(usb_list))
        .route(&page.url_with_path("/copy"), post(copy_file))
}

async fn show_usb_transfer(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering USB transfer view");
    let header = build_header_context(&state, AppView::UsbTransfer).await;
    UsbTransferTemplate { header }
}

async fn local_list() -> impl IntoResponse {
    debug_targeted!(HTTP, "USB transfer local list requested");
    UsbTransferLocalListTemplate {
        sections: list_local_sections().await,
    }
}

async fn usb_list() -> impl IntoResponse {
    debug_targeted!(HTTP, "USB transfer USB list requested");

    let (drives, error_message) = match usb_mountpoints() {
        Ok(mut drives) => {
            drives.sort_by(|a, b| a.mountpoint.cmp(&b.mountpoint));
            (drives, None)
        }
        Err(err) => {
            warn_targeted!(FS, "Failed to read USB mounts: {}", err);
            (Vec::new(), Some(format!("Failed to scan USB devices: {err}")))
        }
    };

    let drive_count = drives.len();
    let selected_drive = drives.first().cloned();

    if drive_count > 1 {
        warn_targeted!(
            FS,
            "Multiple USB drives detected; defaulting to first: {:?}",
            drives.iter().map(|d| d.mountpoint.display().to_string()).collect::<Vec<_>>()
        );
    }

    let sections = if let Some(drive) = &selected_drive {
        list_usb_sections(&drive.mountpoint).await
    } else {
        Vec::new()
    };

    let toast = if drive_count > 1 {
        Some(UsbTransferToast::warning(
            "Multiple USB drives detected. Using the first drive; multi-drive selection is not supported.",
        ))
    } else {
        None
    };

    UsbTransferUsbListTemplate {
        selected_drive,
        drive_count,
        sections,
        error_message,
        toast,
    }
}

pub enum UsbTransferResponse {
    Success { reload_target: ReloadTarget, message: String },
    Error { message: String },
    Confirm { form: UsbTransferForm, detail: String },
}

impl IntoResponse for UsbTransferResponse {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();

        match self {
            UsbTransferResponse::Success { reload_target, message } => {
                let triggers = match reload_target {
                    ReloadTarget::Usb => vec![RELOAD_USB_EVENT],
                    ReloadTarget::Local => vec![RELOAD_LOCAL_EVENT],
                    ReloadTarget::Both => vec![RELOAD_LOCAL_EVENT, RELOAD_USB_EVENT],
                };
                apply_triggers(&mut headers, &triggers);

                let template = UsbTransferToastTemplate {
                    toast: UsbTransferToast::success(message),
                };
                render_template(template, headers)
            }
            UsbTransferResponse::Error { message } => {
                let triggers = vec![RELOAD_LOCAL_EVENT, RELOAD_USB_EVENT];
                apply_triggers(&mut headers, &triggers);

                let template = UsbTransferToastTemplate {
                    toast: UsbTransferToast::error(message),
                };
                render_template(template, headers)
            }
            UsbTransferResponse::Confirm { form, detail } => {
                let template = UsbTransferConfirmModalTemplate {
                    detail,
                    form: form.with_force(true),
                };
                render_template(template, headers)
            }
        }
    }
}

fn apply_triggers(headers: &mut HeaderMap, triggers: &[HxTrigger]) {
    if triggers.is_empty() {
        return;
    }

    let json_map = HxTrigger::list_to_json(triggers);
    if let Ok(json_string) = serde_json::to_string(&json_map) {
        if let Ok(header_value) = HeaderValue::from_str(&json_string) {
            headers.insert("HX-Trigger", header_value);
        }
    }
}

fn render_template<T: askama::Template>(template: T, headers: HeaderMap) -> axum::response::Response {
    match template.render() {
        Ok(html_string) => (headers, axum::response::Html(html_string)).into_response(),
        Err(err) => {
            error_targeted!(HTTP, "Failed to render template: {}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                format!("Template error: {err}")
            ).into_response()
        }
    }
}

async fn copy_file(Form(form): Form<UsbTransferForm>) -> impl IntoResponse {
    info_targeted!(
        HTTP,
        "USB transfer requested: file='{:?}' subdir='{}' direction='{:?}' force={} copy_all={}",
        form.file_name,
        form.subdir,
        form.direction,
        form.force,
        form.copy_all
    );

    if form.copy_all {
        let plan = match prepare_bulk_copy(&form).await {
            Ok(plan) => plan,
            Err(err) => {
                error_targeted!(HTTP, "Failed to prepare bulk USB transfer: {}", err);
                return UsbTransferResponse::Error { message: err.user_message() };
            }
        };

        if plan.conflict_count > 0 && !form.force {
            warn_targeted!(HTTP, "Bulk transfer destination has collisions; requesting overwrite confirmation");
            let detail = format!(
                "{} existing file(s) found in {}. Overwriting will replace them.",
                plan.conflict_count,
                form.direction.destination_label()
            );
            return UsbTransferResponse::Confirm { form, detail };
        }

        match execute_bulk_copy(&plan).await {
            Ok(_) => {
                info_targeted!(HTTP, "Bulk USB transfer completed successfully");
                UsbTransferResponse::Success {
                    reload_target: plan.reload_target,
                    message: format!(
                        "Copied {} file(s) from {} to {}",
                        plan.source_count,
                        subdir_label(plan.subdir),
                        form.direction.destination_label()
                    ),
                }
            }
            Err(err) => {
                error_targeted!(HTTP, "Bulk USB transfer failed: {}", err);
                UsbTransferResponse::Error { message: err.user_message() }
            }
        }
    } else {
        let file_name = match form.file_name.as_deref() {
            Some(name) if !name.is_empty() => name,
            _ => {
                warn_targeted!(HTTP, "Missing file name for USB transfer");
                return UsbTransferResponse::Error { message: "Missing file name.".to_string() };
            }
        };

        let plan = match prepare_copy(&form).await {
            Ok(plan) => plan,
            Err(err) => {
                error_targeted!(HTTP, "Failed to prepare USB transfer: {}", err);
                return UsbTransferResponse::Error { message: err.user_message() };
            }
        };

        if !form.force {
            match destination_exists(&plan.destination).await {
                Ok(true) => {
                    warn_targeted!(HTTP, "Destination exists; requesting overwrite confirmation");
                    let detail = format!(
                        "The file '{}' already exists on {}.",
                        file_name,
                        form.direction.destination_label()
                    );
                    return UsbTransferResponse::Confirm { form, detail };
                }
                Ok(false) => {}
                Err(err) => {
                    error_targeted!(HTTP, "Failed to check destination path: {}", err);
                    return UsbTransferResponse::Error { message: err.user_message() };
                }
            }
        }

        match execute_copy(&plan).await {
            Ok(_) => {
                info_targeted!(HTTP, "USB transfer completed successfully");
                UsbTransferResponse::Success {
                    reload_target: plan.reload_target,
                    message: format!(
                        "Copied '{}' to {}",
                        file_name,
                        form.direction.destination_label()
                    ),
                }
            }
            Err(err) => {
                error_targeted!(HTTP, "USB transfer failed: {}", err);
                UsbTransferResponse::Error { message: err.user_message() }
            }
        }
    }
}
