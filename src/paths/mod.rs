pub mod subdirs;
pub mod usb_drives;

use std::fs;
use std::path::{Path, PathBuf};
use crate::{debug_targeted, trace_targeted};
use crate::paths::subdirs::Subdir;

pub const DEFAULT_ROOT_FOLDER: &str = "hm-pi-data";




pub fn local_data_root_ensuring_exists() -> Result<PathBuf, std::io::Error> {
    let root = match std::env::var_os("HOME").map(PathBuf::from) {
        Some(home) => home.join(DEFAULT_ROOT_FOLDER),
        None => {
            let fallback = PathBuf::from("/var/lib").join(DEFAULT_ROOT_FOLDER);
            log::error!("$HOME is not set; falling back to {}", fallback.display());
            fallback
        }
    };

    fs::create_dir_all(&root).inspect_err( |err| {
        log::error!(
            "Failed to create default data directory {}: {}",
            root.display(),
            err);
    })?;

    Ok(root)
}


