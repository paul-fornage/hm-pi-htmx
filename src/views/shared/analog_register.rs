use crate::modbus::RegisterMetadata;

pub struct AnalogRegisterInfo {
    // from value in register to semantic value: (value + offset) / scale
    pub meta: &'static RegisterMetadata,
    pub unit: &'static str,
    pub scale: u16,
    pub precision: u16,
    pub offset: i16,
    pub max_value: u16, // max on raw value
    pub min_value: u16, // min on raw value
}
impl AnalogRegisterInfo {
    pub fn validate_semantic_value(&self, value: f32) -> Result<(), String> {
        let raw_value = self.convert_to_raw(value);
        if raw_value < self.min_value {
            return Err(format!("Value too low. Minimum: {}", self.convert_from_raw(self.min_value)));
        }
        if raw_value > self.max_value {
            return Err(format!("Value too high. Maximum: {}", self.convert_from_raw(self.max_value)));
        }
        Ok(())
    }

    fn format_helper(&self, raw_value: Option<u16>, precision: usize) -> String {
        match raw_value{
            Some(val) => format!("{:.*}", precision, self.convert_from_raw(val)),
            None => {
                let tail = "-".repeat(precision);
                format!("-.{tail}")
            },
        }
    }

    pub fn formatted_value(&self, raw_value: Option<u16>) -> String {
        self.format_helper(raw_value, self.precision as usize)
    }

    pub fn preview_formatted_value(&self, raw_value: Option<u16>) -> String {
        let precision = self.precision as usize;
        const MAX_PREVIEW_PRECISION: usize = 4;
        let clamped_precision = precision.min(MAX_PREVIEW_PRECISION);
        self.format_helper(raw_value, clamped_precision)
    }
    
    pub fn unit_string(&self) -> &'static str {
        self.unit
    }

    pub const fn convert_from_raw_helper(scale: u16, offset: i16, raw_value: u16) -> f32 {
        ((raw_value as f32 + offset as f32) / scale as f32)
    }

    pub const fn convert_to_raw_helper(scale: u16, offset: i16, semantic_value: f32) -> u16 {
        ((semantic_value * scale as f32) - offset as f32) as u16
    }

    pub fn convert_from_raw(&self, raw_value: u16) -> f32 {
        Self::convert_from_raw_helper(self.scale, self.offset, raw_value)
    }

    pub fn convert_to_raw(&self, semantic_value: f32) -> u16 {
        Self::convert_to_raw_helper(self.scale, self.offset, semantic_value)
    }

    pub const fn new(meta: &'static RegisterMetadata, unit: &'static str,
                     decimal_places: u16, offset: i16) -> Self {
        let scale = 10_u16.pow(decimal_places as u32);
        Self {
            meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: u16::MAX,
            min_value: u16::MIN,
        }
    }

    pub const fn new_bounded(meta: &'static RegisterMetadata, unit: &'static str,
                             decimal_places: u16, offset: i16,
                             semantic_max_value: i32, semantic_min_value: i32) -> Self {
        let scale = 10_u16.pow(decimal_places as u32);
        Self {
            meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: Self::convert_to_raw_helper(scale, offset, semantic_max_value as f32),
            min_value: Self::convert_to_raw_helper(scale, offset, semantic_min_value as f32),
        }
    }
    pub const fn new_with_raw_bounds(meta: &'static RegisterMetadata, unit: &'static str,
                             decimal_places: u16, offset: i16,
                             raw_max_value: u16, raw_min_value: u16) -> Self {
        let scale = 10_u16.pow(decimal_places as u32);
        Self {
            meta,
            unit,
            scale,
            precision: decimal_places,
            offset,
            max_value: raw_max_value,
            min_value: raw_min_value,
        }
    }

    pub fn get_meta(&self) -> &'static RegisterMetadata {
        self.meta
    }
}