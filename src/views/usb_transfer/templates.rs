use askama::Template;
use askama_web::WebTemplate;

use crate::paths::usb_drives::UsbDrive;
use crate::views::{AppView, HeaderContext, ViewTemplate};

use super::transfer::SubdirSection;
use super::types::UsbTransferForm;

#[derive(Template, WebTemplate)]
#[template(path = "views/usb-transfer.html")]
pub struct UsbTransferTemplate {
    pub header: HeaderContext,
}

impl ViewTemplate for UsbTransferTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::UsbTransfer;
}

#[derive(Template, WebTemplate)]
#[template(path = "components/usb-transfer/local-list.html")]
pub struct UsbTransferLocalListTemplate {
    pub sections: Vec<SubdirSection>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/usb-transfer/usb-list.html")]
pub struct UsbTransferUsbListTemplate {
    pub selected_drive: Option<UsbDrive>,
    pub drive_count: usize,
    pub sections: Vec<SubdirSection>,
    pub error_message: Option<String>,
    pub toast: Option<UsbTransferToast>,
}

#[derive(Debug, Clone)]
pub struct UsbTransferToast {
    pub kind: UsbTransferToastKind,
    pub message: String,
    pub close_modal: bool,
}

impl UsbTransferToast {
    pub fn success(message: impl Into<String>) -> Self {
        Self { kind: UsbTransferToastKind::Success, message: message.into(), close_modal: true }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self { kind: UsbTransferToastKind::Error, message: message.into(), close_modal: false }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self { kind: UsbTransferToastKind::Warning, message: message.into(), close_modal: false }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UsbTransferToastKind {
    Success,
    Error,
    Warning,
}

impl UsbTransferToastKind {
    pub fn label(&self) -> &'static str {
        match self {
            UsbTransferToastKind::Success => "Success",
            UsbTransferToastKind::Error => "Error",
            UsbTransferToastKind::Warning => "Heads up",
        }
    }

    pub fn classes(&self) -> &'static str {
        match self {
            UsbTransferToastKind::Success => "border-emerald-300 bg-emerald-50 text-emerald-800",
            UsbTransferToastKind::Error => "border-red-300 bg-red-50 text-red-800",
            UsbTransferToastKind::Warning => "border-amber-300 bg-amber-50 text-amber-800",
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/usb-transfer/toast.html")]
pub struct UsbTransferToastTemplate {
    pub toast: UsbTransferToast,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/usb-transfer/confirm-overwrite-modal.html")]
pub struct UsbTransferConfirmModalTemplate {
    pub form: UsbTransferForm,
    pub detail: String,
}
