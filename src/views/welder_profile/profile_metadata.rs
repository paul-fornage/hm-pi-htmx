use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WeldProfileMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl WeldProfileMetadata {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}
