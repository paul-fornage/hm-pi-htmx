pub enum LogTarget {
    HTTP,
    MODBUS
}

impl Into<&'static str> for LogTarget {
    fn into(self) -> &'static str {
        match self {
            LogTarget::HTTP => "http",
            LogTarget::MODBUS => "modbus"
        }
    }
}

#[macro_export]
macro_rules! error_targeted { ($target:ident, $($arg:tt)+) => {
    log::error!(target: $crate::LogTarget::$target.into(), $($arg)+)
}; }
#[macro_export]
macro_rules! warn_targeted  { ($target:ident, $($arg:tt)+) => {
    log::warn!(target: $crate::LogTarget::$target.into(), $($arg)+)
}; }
#[macro_export]
macro_rules! info_targeted  { ($target:ident, $($arg:tt)+) => { 
    log::info!(target: $crate::LogTarget::$target.into(), $($arg)+)
}; }
#[macro_export]
macro_rules! debug_targeted { ($target:ident, $($arg:tt)+) => {
    log::debug!(target: $crate::LogTarget::$target.into(), $($arg)+)
}; }
#[macro_export]
macro_rules! trace_targeted { ($target:ident, $($arg:tt)+) => {
    log::trace!(target: $crate::LogTarget::$target.into(), $($arg)+)
}; }