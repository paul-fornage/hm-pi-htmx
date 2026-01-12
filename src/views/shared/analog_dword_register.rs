use crate::modbus::RegisterMetadata;

pub struct AnalogDwordRegisterInfo {
    pub low_meta: &'static RegisterMetadata,
    pub high_meta: &'static RegisterMetadata,
    pub unit: &'static str,
    pub scale: u32,
    pub precision: u16,
    pub offset: i32,
    pub max_value: u32, // max on raw value
    pub min_value: u32, // min on raw value
}

impl AnalogDwordRegisterInfo {
    pub fn validate_semantic_value(&self, value: f64) -> Result<(), String> {
        let raw_value = self.convert_to_raw(value);
        if raw_value < self.min_value {
            return Err(format!("Value too low. Minimum: {}", self.convert_from_raw(self.min_value)));
        }
        if raw_value > self.max_value {
            return Err(format!("Value too high. Maximum: {}", self.convert_from_raw(self.max_value)));
        }
        Ok(())
    }

    pub fn formatted_value(&self, raw_value: Option<u32>) -> String {
        match raw_value{
            Some(val) => format!("{:.*}",
                                 self.precision as usize,
                                 self.convert_from_raw(val)),
            None => {
                let tail = "-".repeat(self.precision as usize);
                format!("-.{tail}")
            },
        }
    }

    pub fn unit_string(&self) -> &'static str {
        self.unit
    }

    pub const fn convert_from_raw_helper(scale: u32, offset: i32, raw_value: u32) -> f64 {
        ((raw_value as f64 + offset as f64) / scale as f64)
    }

    pub const fn convert_to_raw_helper(scale: u32, offset: i32, semantic_value: f64) -> u32 {
        ((semantic_value * scale as f64) - offset as f64) as u32
    }

    pub fn convert_from_raw(&self, raw_value: u32) -> f64 {
        Self::convert_from_raw_helper(self.scale, self.offset, raw_value)
    }

    pub fn convert_to_raw(&self, semantic_value: f64) -> u32 {
        Self::convert_to_raw_helper(self.scale, self.offset, semantic_value)
    }

    pub const fn new(low_meta: &'static RegisterMetadata, high_meta: &'static RegisterMetadata,
                     unit: &'static str, decimal_places: u16, offset: i32) -> Self {
        let scale = 10_u32.pow(decimal_places as u32);
        Self {
            low_meta,
            high_meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: u32::MAX,
            min_value: u32::MIN,
        }
    }

    pub const fn new_bounded(low_meta: &'static RegisterMetadata, high_meta: &'static RegisterMetadata,
                             unit: &'static str, decimal_places: u16, offset: i32,
                             semantic_max_value: f64, semantic_min_value: f64) -> Self {
        let scale = 10_u32.pow(decimal_places as u32);
        Self {
            low_meta,
            high_meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: Self::convert_to_raw_helper(scale, offset, semantic_max_value),
            min_value: Self::convert_to_raw_helper(scale, offset, semantic_min_value),
        }
    }
    pub const fn new_with_raw_bounds(low_meta: &'static RegisterMetadata, high_meta: &'static RegisterMetadata,
                                     unit: &'static str, decimal_places: u16, offset: i32,
                                     raw_max_value: u32, raw_min_value: u32) -> Self {
        let scale = 10_u32.pow(decimal_places as u32);
        Self {
            low_meta,
            high_meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: raw_max_value,
            min_value: raw_min_value,
        }
    }
    
    /// default to low
    pub fn get_meta(&self) -> &'static RegisterMetadata {
        self.low_meta
    }
}