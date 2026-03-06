pub mod subdirs;
pub mod usb_drives;

use std::fs;
use std::path::{Path, PathBuf};
use crate::{debug_targeted, trace_targeted};
use crate::paths::subdirs::Subdir;

pub const LOCAL_DEFAULT: &str = "hm-pi-data";


pub fn full_path_for_subdir(subdir: Subdir) -> PathBuf {
    local_data_root_ensuring_exists().join(subdir.path())
}

pub fn full_path_for_subdir_verified(subdir: Subdir) -> Result<PathBuf, std::io::Error> {
    let path = local_data_root().join(subdir.path());
    fs::create_dir_all(&path).inspect_err( |err| {
        log::warn!(
            "Failed to create default data directory {}: {}",
            path.display(),
            err
        );
    })?;
    Ok(path)
}



pub fn local_data_root() -> PathBuf {
    match home_dir() {
        Some(home) => home.join(LOCAL_DEFAULT),
        None => {
            log::error!("$HOME is not set; falling back to relative path ./{LOCAL_DEFAULT}");
            PathBuf::from(LOCAL_DEFAULT)
        }
    }
}

pub fn local_data_root_ensuring_exists() -> PathBuf {
    let root = local_data_root();

    if let Err(e) = fs::create_dir_all(&root) {
        log::warn!(
            "Failed to create default data directory {}: {}",
            root.display(),
            e
        );
    }

    root
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

