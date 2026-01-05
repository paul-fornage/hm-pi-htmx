use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use std::fmt::Display;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-info/boolean-read-only-register.html")]
pub struct BooleanRegisterTemplate {
    pub meta: &'static RegisterMetadata,
    pub value: Option<bool>,
}


#[derive(Template, WebTemplate)]
#[template(path = "components/welder-info/enum-read-only-register.html")]
pub struct EnumRegisterTemplate<T: Display> {
    pub meta: &'static RegisterMetadata,
    pub value: Option<T>,
}
impl<T: Display> EnumRegisterTemplate<T> {
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn formatted_value(&self) -> String {
        match &self.value {
            Some(val) => val.to_string(),
            None => String::from("---")
        }
    }
}



#[derive(Template, WebTemplate)]
#[template(path = "components/welder-info/analog-read-only-register.html")]
pub struct AnalogRegisterTemplate {
    pub raw_value: Option<u16>,
    pub register_info: &'static AnalogRegisterInfo,
}
impl AnalogRegisterTemplate {

    pub fn has_value(&self) -> bool { self.raw_value.is_some() }

    pub fn formatted_value(&self) -> String { self.register_info.formatted_value(self.raw_value) }
}


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


    pub fn formatted_value(&self, raw_value: Option<u16>) -> String {
        match raw_value{
            Some(val) => format!("{:.*} {}",
                                 self.precision as usize,
                                 self.convert_from_raw(val),
                                 self.unit),
            None => String::from("---")
        }
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
}

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-info/statistics-bar.html")]
pub struct StatisticsBarTemplate {
    pub arc_time: Option<crate::miller::miller_register_types::ArcTime>,
    pub arc_cycles: Option<crate::miller::miller_register_types::ArcCycles>,
}
impl StatisticsBarTemplate {
    pub fn arc_time_display(&self) -> String {
        match &self.arc_time {
            Some(val) => val.to_string(),
            None => String::from("---")
        }
    }

    pub fn arc_cycles_display(&self) -> String {
        match &self.arc_cycles {
            Some(val) => format!("arc cycles: {}", val),
            None => String::from("arc cycles: ---")
        }
    }
}


