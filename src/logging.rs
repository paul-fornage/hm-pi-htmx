use std::fs;
use std::path::Path;

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

const MAX_LOG_FILE_SIZE_BYTES: u64 = 1 * 1024 * 1024;
const MAX_LOG_FILES: u32 = 100;

pub enum LogTarget {
    HTTP,
    MODBUS,
    FS,
    Clearcore
}

impl Into<&'static str> for LogTarget {
    fn into(self) -> &'static str {
        self.to_str()
    }
}

impl LogTarget {
    pub fn to_str(self) -> &'static str {
        match self {
            LogTarget::HTTP => "http",
            LogTarget::MODBUS => "modbus",
            LogTarget::FS => "fs",
            LogTarget::Clearcore => "clearcore",
        }
    }
}

pub fn init_logger(log_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(log_dir)?;

    let base_log_path = log_dir.join("log-current.txt");
    let archive_pattern = log_dir.join("log-{}.txt");

    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}][{h({l})}][{t}] {m}{n}",
        )))
        .build();

    let trigger = SizeTrigger::new(MAX_LOG_FILE_SIZE_BYTES);
    let roller = FixedWindowRoller::builder().build(
        archive_pattern
            .to_str()
            .ok_or("Invalid archive path for log4rs")?,
        MAX_LOG_FILES,
    )?;
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let rolling_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "[{d(%Y-%m-%d %H:%M:%S)}][{l}][{t}] {m}{n}",
        )))
        .build(base_log_path, Box::new(policy))?;


    let config_builder = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .appender(Appender::builder().build("file", Box::new(rolling_file)))
        .logger(Logger::builder().build(LogTarget::FS.to_str(), LevelFilter::Debug))
        .logger(Logger::builder().build(LogTarget::Clearcore.to_str(), LevelFilter::Debug))
        .logger(Logger::builder().build(LogTarget::MODBUS.to_str(), LevelFilter::Debug))
        .logger(Logger::builder().build(LogTarget::HTTP.to_str(), LevelFilter::Debug))
        .logger(Logger::builder().build("tokio_modbus::service::tcp", LevelFilter::Info));


    let config = config_builder.build(
        Root::builder()
            .appender("stderr")
            .appender("file")
            .build(LevelFilter::Debug),
    )?;

    log4rs::init_config(config)?;
    Ok(())
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
