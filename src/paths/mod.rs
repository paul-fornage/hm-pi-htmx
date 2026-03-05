pub mod subdirs;

use std::fs;
use std::path::{Path, PathBuf};
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

pub fn data_root() -> PathBuf {
    match get_usb_mountpoint() {
        Ok(Some(mount)) => mount, 
        Ok(None) => local_data_root_ensuring_exists(),
        Err(err) => {
            log::warn!("Failed to detect USB mounts ({}). Falling back to default.", err);
            local_data_root_ensuring_exists()
        }
    }
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

pub fn get_usb_mountpoint() -> Result<Option<PathBuf>, std::io::Error> {
    let mut mounts =  usb_mountpoints().inspect_err( |err| { 
        log::warn!("Error looking for USB mounts ({:?})", err) 
    })?;
    if mounts.is_empty() {
        Ok(None)
    } else {
        mounts.sort();
        let selected_mount = &mounts[0];
        if mounts.len() > 1 {
            log::warn!(
                "Multiple USB/storage mounts detected: {:?}. Using: {}",
                mounts,
                selected_mount.display()
            );
        }
        Ok(Some(selected_mount.clone()))
    }
}

/// Parse `/proc/mounts` and return mountpoints that look like removable storage.
fn usb_mountpoints() -> Result<Vec<PathBuf>, std::io::Error> {
    let mounts = fs::read_to_string("/proc/mounts")?;

    let mut results = Vec::new();

    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let _source = parts.next();
        let target = parts.next();
        let fstype = parts.next();

        let (Some(target), Some(fstype)) = (target, fstype) else { continue };

        if !is_usb_fstype(fstype) {
            continue;
        }

        let target = target.replace(r"\040", " ");
        let target_path = PathBuf::from(target);

        if is_removable_mount_root(&target_path) {
            results.push(target_path);
        }
    }

    Ok(results)
}

fn is_usb_fstype(fstype: &str) -> bool {
    matches!(
        fstype,
        "vfat" | "exfat" | "ntfs" | "ntfs3" | "ext2" | "ext3" | "ext4"
    )
}

fn is_removable_mount_root(p: &Path) -> bool {
    p.starts_with("/run/media") || p.starts_with("/media") || p.starts_with("/mnt")
}

