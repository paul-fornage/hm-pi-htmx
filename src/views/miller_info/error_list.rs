
use askama::Template;
use askama_web::WebTemplate;
use crate::miller::miller_error_registers::{ErrorReg1, ErrorReg2, ErrorReg3, MillerErrorRegister};
use crate::miller::miller_register_types::WelderModel;



/// Component that displays decoded error lists from the three error registers.
/// Decoding is based on the configured welder model.
#[derive(Template, WebTemplate)]
#[template(path = "components/error-list.html")]
pub struct ErrorListTemplate {
    pub error_reg_1: Option<ErrorReg1>,
    pub error_reg_2: Option<ErrorReg2>,
    pub error_reg_3: Option<ErrorReg3>,
    pub welder_model: WelderModel,
}

impl ErrorListTemplate {
    /// Get all active errors from all registers, decoded for the configured model
    pub fn get_all_errors(&self) -> Vec<String> {
        let mut all_errors = Vec::new();

        if let Some(reg1) = self.error_reg_1 {
            all_errors.extend(reg1.get_active_errors(self.welder_model.clone()));
        }

        if let Some(reg2) = self.error_reg_2 {
            all_errors.extend(reg2.get_active_errors(self.welder_model.clone()));
        }

        if let Some(reg3) = self.error_reg_3 {
            all_errors.extend(reg3.get_active_errors(self.welder_model.clone()));
        }

        all_errors
    }

    /// Check if any errors are present
    pub fn has_errors(&self) -> bool {
        self.error_reg_1.map(|e|{e.has_errors()}).unwrap_or(true)
            || self.error_reg_2.map(|e|{e.has_errors()}).unwrap_or(true)
            || self.error_reg_3.map(|e|{e.has_errors()}).unwrap_or(true)
    }

    /// Check if any registers are unavailable (None)
    pub fn has_unavailable_registers(&self) -> bool {
        self.error_reg_1.is_none() || self.error_reg_2.is_none() || self.error_reg_3.is_none()
    }
}
