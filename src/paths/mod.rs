pub mod subdirs;
pub mod usb_drives;

use std::fs;
use std::path::{PathBuf};

pub const DEFAULT_ROOT_FOLDER: &str = "hm-pi-data";




pub fn local_data_root_ensuring_exists() -> Result<PathBuf, std::io::Error> {
    let root = match std::env::var_os("HOME").map(PathBuf::from) {
        Some(home) => home.join(DEFAULT_ROOT_FOLDER),
        None => {
            log::error!("$HOME is not set; falling back to relative path ./{DEFAULT_ROOT_FOLDER}");
            PathBuf::from(DEFAULT_ROOT_FOLDER)
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


