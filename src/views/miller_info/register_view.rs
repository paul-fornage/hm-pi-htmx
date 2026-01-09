use askama::Template;
use askama_web::WebTemplate;
use crate::modbus::RegisterMetadata;
use std::fmt::Display;
pub(crate) use crate::views::shared::analog_register::AnalogRegisterInfo;

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


