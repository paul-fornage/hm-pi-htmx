use crate::miller::miller_error_registers::{ErrorReg1, ErrorReg2, ErrorReg3, MillerErrorRegister};
use crate::miller::miller_register_types::WelderModel;
use askama::Template;
use askama_web::WebTemplate;

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
        let welder_model = self.welder_model.clone();

        if let Some(reg1) = self.error_reg_1 {
            all_errors.extend(reg1.get_active_errors_string(&welder_model));
        }

        if let Some(reg2) = self.error_reg_2 {
            all_errors.extend(reg2.get_active_errors_string(&welder_model));
        }

        if let Some(reg3) = self.error_reg_3 {
            all_errors.extend(reg3.get_active_errors_string(&welder_model));
        }

        all_errors
    }

    pub fn reg_1_errors(&self) -> Option<Vec<String>> {
        self.error_reg_1.as_ref()
            .map(|r| r.get_active_errors_string(&self.welder_model))
    }

    /// Helper to get errors specifically for Register 2
    pub fn reg_2_errors(&self) -> Option<Vec<String>> {
        self.error_reg_2.as_ref()
            .map(|r| r.get_active_errors_string(&self.welder_model))
    }

    /// Helper to get errors specifically for Register 3
    pub fn reg_3_errors(&self) -> Option<Vec<String>> {
        self.error_reg_3.as_ref()
            .map(|r| r.get_active_errors_string(&self.welder_model))
    }

    /// Check if any errors are present
    pub fn has_errors(&self) -> bool {
        let reg_1_has_errors = self.error_reg_1.map(|e|{e.has_errors()}).unwrap_or(true);
        let reg_2_has_errors = self.error_reg_2.map(|e|{e.has_errors()}).unwrap_or(true);
        let reg_3_has_errors = self.error_reg_3.map(|e|{e.has_errors()}).unwrap_or(true);

        reg_1_has_errors || reg_2_has_errors || reg_3_has_errors
    }

    /// Check if any registers are unavailable (None)
    pub fn has_unavailable_registers(&self) -> bool {
        self.error_reg_1.is_none() || self.error_reg_2.is_none() || self.error_reg_3.is_none()
    }
}
