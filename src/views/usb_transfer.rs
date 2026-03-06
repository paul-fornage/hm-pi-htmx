use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::paths::usb_drives::{UsbDrive, usb_mountpoints};
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{debug_targeted, warn_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "views/usb-transfer.html")]
pub struct UsbTransferTemplate {
    pub header: HeaderContext,
}

impl ViewTemplate for UsbTransferTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::UsbTransfer;
}

#[derive(Template, WebTemplate)]
#[template(path = "components/usb-transfer/mount-list.html")]
pub struct UsbMountListTemplate {
    pub drives: Vec<UsbDrive>,
}

fn read_usb_mounts() -> Vec<UsbDrive> {
    match usb_mountpoints() {
        Ok(list) => list,
        Err(err) => {
            warn_targeted!(FS, "Failed to read USB mounts: {}", err);
            Vec::new()
        }
    }
}

async fn show_usb_transfer(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering USB transfer view");
    let header = build_header_context(&state, AppView::UsbTransfer).await;
    UsbTransferTemplate { header }
}

async fn usb_mount_list() -> impl IntoResponse {
    UsbMountListTemplate {
        drives: read_usb_mounts(),
    }
}

pub fn routes() -> Router<AppState> {
    let page = AppView::UsbTransfer;
    Router::new()
        .route(page.url(), get(show_usb_transfer))
        .route(&page.url_with_path("/list"), get(usb_mount_list))
}
