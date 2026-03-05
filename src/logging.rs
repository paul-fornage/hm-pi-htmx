use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

const MAX_LOG_FILE_SIZE_BYTES: u64 = 1 * 1 * 1024;
const MAX_LOG_TOTAL_SIZE_BYTES: u64 = 1 * 10 * 1024;

pub enum LogTarget {
    HTTP,
    MODBUS,
    FS,
    Clearcore
}

// TODO: This whole thing is stupid, just use the damned crate and get the file name.
//  Could be nice if the use wants to view logs, but if they want logs they're allowed
//  to know that the code lives in files

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

    let file_logger = RollingFileLogger::new(log_dir)?;

    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightBlack)
        .debug(Color::Magenta)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    let stderr_dispatch = fern::Dispatch::new().format(move |out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            colors.color(record.level()),
            record.target(),
            message
        ))
    });

    let file_dispatch = fern::Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            message
        ))
    });

    let mut dispatch = fern::Dispatch::new()
        .chain(stderr_dispatch.chain(std::io::stderr()))
        .chain(file_dispatch.chain(Box::new(file_logger) as Box<dyn Write + Send>));

    if cfg!(debug_assertions) {
        dispatch = dispatch
            .level(LevelFilter::Debug)
            .level_for(LogTarget::MODBUS.to_str(), LevelFilter::Debug)
            .level_for(LogTarget::HTTP.to_str(), LevelFilter::Debug)
            .level_for("tokio_modbus::service::tcp", LevelFilter::Info);
    } else {
        dispatch = dispatch.level(LevelFilter::Info);
    }

    dispatch.apply()?;
    Ok(())
}

struct RollingFileLogger {
    state: Mutex<RollingState>,
}

impl RollingFileLogger {
    fn new(log_dir: &Path) -> io::Result<Self> {
        let base_timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
        let (file, path, index) = open_new_log_file(log_dir, &base_timestamp, 0)?;
        let current_size = file.metadata().map(|m| m.len()).unwrap_or(0);
        let mut logger = Self {
            state: Mutex::new(RollingState {
                log_dir: log_dir.to_path_buf(),
                base_timestamp,
                current_index: index,
                current_path: path,
                current_size,
                file,
            }),
        };

        logger.enforce_total_size()?;
        Ok(logger)
    }

    fn enforce_total_size(&mut self) -> io::Result<()> {
        let mut state = self.state.lock().expect("logging state lock poisoned");
        enforce_total_size(&state.log_dir, MAX_LOG_TOTAL_SIZE_BYTES, &state.current_path)
    }
}

impl Write for RollingFileLogger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut state = self.state.lock().expect("logging state lock poisoned");
        state.rotate_if_needed(buf.len() as u64)?;
        state.file.write_all(buf)?;
        state.current_size = state.current_size.saturating_add(buf.len() as u64);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut state = self.state.lock().expect("logging state lock poisoned");
        state.file.flush()
    }
}

struct RollingState {
    log_dir: PathBuf,
    base_timestamp: String,
    current_index: u32,
    current_path: PathBuf,
    current_size: u64,
    file: File,
}

impl RollingState {
    fn rotate_if_needed(&mut self, incoming_bytes: u64) -> io::Result<()> {
        if self.current_size.saturating_add(incoming_bytes) <= MAX_LOG_FILE_SIZE_BYTES {
            return Ok(());
        }

        self.current_index = self.current_index.saturating_add(1);
        let (file, path, index) =
            open_new_log_file(&self.log_dir, &self.base_timestamp, self.current_index)?;
        self.current_index = index;
        self.current_path = path;
        self.current_size = 0;
        self.file = file;

        enforce_total_size(&self.log_dir, MAX_LOG_TOTAL_SIZE_BYTES, &self.current_path)
    }
}

fn open_new_log_file(
    log_dir: &Path,
    base_timestamp: &str,
    start_index: u32,
) -> io::Result<(File, PathBuf, u32)> {
    let mut index = start_index;
    loop {
        let file_name = format!("log-{base_timestamp}-{index}.txt");
        let path = log_dir.join(file_name);
        match OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&path)
        {
            Ok(file) => return Ok((file, path, index)),
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                index = index.saturating_add(1);
            }
            Err(err) => return Err(err),
        }
    }
}

struct LogFileInfo {
    path: PathBuf,
    size: u64,
    modified: SystemTime,
}

fn enforce_total_size(
    log_dir: &Path,
    max_total_size: u64,
    keep_path: &Path,
) -> io::Result<()> {
    let mut files = Vec::new();
    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        if !file_name.starts_with("log-") || !file_name.ends_with(".txt") {
            continue;
        }

        let metadata = entry.metadata()?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        files.push(LogFileInfo {
            path,
            size: metadata.len(),
            modified,
        });
    }

    files.sort_by_key(|file| file.modified);
    let mut total_size: u64 = files.iter().map(|file| file.size).sum();
    for file in files {
        if total_size <= max_total_size {
            break;
        }
        if file.path == keep_path {
            continue;
        }
        if fs::remove_file(&file.path).is_ok() {
            total_size = total_size.saturating_sub(file.size);
        }
    }

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
