use std::collections::HashMap;
use std::path::Path;
use askama::Template;
use askama_web::WebTemplate;
use serde::{Deserialize, Serialize};
use crate::file_io::{deserialize_json, serialize_json, FileIoError, FixedDiskFile, NamedDiskFile};
use crate::modbus::RegisterMetadata;
use crate::paths::subdirs::Subdir;
use crate::views::schedule_adjustments::adjustable_registers::ADJUSTABLE_REGISTERS;
use crate::warn_targeted;

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[template(path = "components/allowed-adjustments/adjustment-row.html")]
pub struct AdjustmentRowDisplay {
    // pub register_name: String,
    pub percent_less: u8,
    pub percent_over: u8,
}


impl AdjustmentRowDisplay {
    // pub fn name(&self) -> &str { &self.register_name }
    // pub fn display_name(&self) -> &str { &self.register_name }
    pub fn percent_less(&self) -> u8 { self.percent_less }
    pub fn percent_over(&self) -> u8 { self.percent_over }

    pub fn default_from_register(name: String) -> Self {
        Self {
            // register_name: name,
            percent_less: 0,
            percent_over: 0,
        }
    }
}



#[derive(Template, WebTemplate, Clone, Serialize, Deserialize)]
#[template(path = "components/allowed-adjustments/allowed-adjustments-form.html")]
pub struct AllowedAdjustments {
    adjustments: HashMap<String, AdjustmentRowDisplay>,
}

impl Default for AllowedAdjustments {
    fn default() -> Self {
        let mut map = HashMap::new();
        for reg in ADJUSTABLE_REGISTERS {
            let name = reg.name.to_string();
            let row = AdjustmentRowDisplay::default_from_register(name.clone());
            map.insert(name, row);
        }
        Self { adjustments: map }
    }
}


impl FixedDiskFile for AllowedAdjustments {
    const SUBDIR: Subdir = Subdir::Config;
    const FILE_NAME: &'static str = "allowed-operator-adjustments.json";

    fn serialize_value(&self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(self, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        deserialize_json(contents, path)
    }
}

impl AllowedAdjustments {
    /// Adds missing adjustments to the allowed adjustments list, and remove old ones. Used in schema changes!
    pub fn conform_to_schema(&mut self) -> bool {
        let mut changed = false;
        let mut new_map = HashMap::new();
        for reg in ADJUSTABLE_REGISTERS {
            let name = reg.name.to_string();
            match self.adjustments.remove(&name) {
                Some(row) => {
                    new_map.insert(name, row);
                },
                None => {
                    changed = true;
                    warn_targeted!(FS, "Adding missing adjustment from saved schema {}", name);
                    let row = AdjustmentRowDisplay::default_from_register(name.clone());
                    new_map.insert(name, row);
                }
            }
        }

        for (name, _row) in self.adjustments.iter() {
            changed = true;
            warn_targeted!(FS, "discarding old adjustment from saved schema {}", name);
        }

        self.adjustments = new_map;
        return changed;
    }
    
    pub fn verify_schema(&self) -> bool {
        let num_adjustments = ADJUSTABLE_REGISTERS.len();
        let num_adjustments_in_form = self.adjustments.len();
        if(num_adjustments != num_adjustments_in_form){
            warn_targeted!(FS, "Adjustments form has {} adjustments, but schema has {} adjustments", num_adjustments_in_form, num_adjustments);
            return false;
        }
        for name in ADJUSTABLE_REGISTERS.iter().map(|reg| reg.name.to_string()) {
            if !self.adjustments.contains_key(&name) {
                warn_targeted!(FS, "Adjustments form does not have adjustment for {}", name);
                return false;
            }
        }
        return true;
    }

    pub fn as_list(&self) -> Vec<(&String, &AdjustmentRowDisplay)> {
        let mut list: Vec<(&String, &AdjustmentRowDisplay)> = self.adjustments.iter().collect();
        list.sort_by(|a, b| {
            a.0.cmp(b.0)
        });
        list
    }

}

