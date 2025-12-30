use askama::Template;
use askama_web::WebTemplate;
use crate::miller::miller_register_types::{SerialNumber, SoftwareUpdateRevision, SubModuleSoftwareVersion};

/// Component that displays version and serial number information.
/// All fields are Option<T> to gracefully handle read failures.
#[derive(Template, WebTemplate)]
#[template(path = "components/version-info.html")]
pub struct VersionInfoTemplate {
    pub software_version: Option<SoftwareUpdateRevision>,
    pub serial_number: Option<SerialNumber>,
    pub app_software_version_pcb_1: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_2: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_3: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_4: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_5: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_6: Option<SubModuleSoftwareVersion>,
    pub app_software_version_pcb_7: Option<SubModuleSoftwareVersion>,
}

impl VersionInfoTemplate {
    /// Check if any version registers are unavailable (None)
    pub fn has_unavailable_registers(&self) -> bool {
        self.software_version.is_none()
            || self.serial_number.is_none()
            || self.app_software_version_pcb_1.is_none()
            || self.app_software_version_pcb_2.is_none()
            || self.app_software_version_pcb_3.is_none()
            || self.app_software_version_pcb_4.is_none()
            || self.app_software_version_pcb_5.is_none()
            || self.app_software_version_pcb_6.is_none()
            || self.app_software_version_pcb_7.is_none()
    }
}
