use crate::modbus::RegisterMetadata;

pub struct BooleanRegisterInfo {
    pub meta: &'static RegisterMetadata,
    pub true_string: &'static str,
    pub false_string: &'static str,
}

impl BooleanRegisterInfo {
    pub const fn new_custom(meta: &'static RegisterMetadata, true_string: &'static str, false_string: &'static str) -> Self {
        Self { meta, true_string, false_string }
    }
    
    pub const fn new_default(meta: &'static RegisterMetadata) -> Self {
        Self::new_custom(meta, "True", "False")
    }

    pub fn render_value(&self, value: Option<bool>) -> &'static str {
        match value {
            Some(true) => self.true_string,
            Some(false) => self.false_string,
            None => "---",
        }
    }
    
    pub fn get_meta(&self) -> &'static RegisterMetadata {
        self.meta
    }
}