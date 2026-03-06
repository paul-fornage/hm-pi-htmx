use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransferDirection {
    UsbToLocal,
    LocalToUsb,
}

impl TransferDirection {
    pub fn destination_label(&self) -> &'static str {
        match self {
            TransferDirection::UsbToLocal => "Local storage",
            TransferDirection::LocalToUsb => "USB drive",
        }
    }

    pub fn form_value(&self) -> &'static str {
        match self {
            TransferDirection::UsbToLocal => "usb-to-local",
            TransferDirection::LocalToUsb => "local-to-usb",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsbTransferForm {
    pub file_name: Option<String>,
    pub subdir: String,
    pub direction: TransferDirection,
    pub usb_mountpoint: String,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub copy_all: bool,
}

impl UsbTransferForm {
    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ReloadTarget {
    Usb,
    Local,
    Both,
}
