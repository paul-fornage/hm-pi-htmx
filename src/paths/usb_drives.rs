use std::fs;
use std::path::{Path, PathBuf};
use crate::{debug_targeted, trace_targeted};

#[derive(Debug, Clone)]
pub struct UsbDrive {
    pub name: String,
    pub mountpoint: PathBuf,
    pub size_bytes: u64,
}

impl UsbDrive {
    pub fn size_display(&self) -> String {
        size_to_string(self.size_bytes)
    }

    pub fn mountpoint_display(&self) -> String {
        self.mountpoint.display().to_string()
    }
}

pub fn get_usb_mountpoint() -> Result<Option<PathBuf>, std::io::Error> {
    let mut mounts = usb_mountpoints().inspect_err(|err| {
        log::warn!("Error looking for USB mounts ({:?})", err)
    })?;
    if mounts.is_empty() {
        Ok(None)
    } else {
        mounts.sort_by(|a, b| a.mountpoint.cmp(&b.mountpoint));
        let selected_mount = &mounts[0].mountpoint;
        if mounts.len() > 1 {
            log::warn!(
                "Multiple USB/storage mounts detected: {:?}. Using: {}",
                mounts
                    .iter()
                    .map(|drive| drive.mountpoint.display().to_string())
                    .collect::<Vec<_>>(),
                selected_mount.display()
            );
        }
        Ok(Some(selected_mount.clone()))
    }
}

/// Parse `/proc/mounts` and return mountpoints that look like removable storage.
pub fn usb_mountpoints() -> Result<Vec<UsbDrive>, std::io::Error> {
    let mounts = fs::read_to_string("/proc/mounts")?;

    debug_targeted!(FS, "Found USB mounts:\n{}", mounts);
    /*
    [2026-03-06 08:05:50][DEBUG][fs] Found USB mounts:
proc /proc proc rw,nosuid,nodev,noexec,relatime 0 0
sys /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0
dev /dev devtmpfs rw,nosuid,relatime,size=16154220k,nr_inodes=4038555,mode=755,inode64 0 0
run /run tmpfs rw,nosuid,nodev,relatime,mode=755,inode64 0 0
efivarfs /sys/firmware/efi/efivars efivarfs rw,nosuid,nodev,noexec,relatime 0 0
devpts /dev/pts devpts rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000 0 0
/dev/nvme1n1p2 / btrfs rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=256,subvol=/@ 0 0
securityfs /sys/kernel/security securityfs rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /dev/shm tmpfs rw,nosuid,nodev,inode64,usrquota 0 0
cgroup2 /sys/fs/cgroup cgroup2 rw,nosuid,nodev,noexec,relatime,nsdelegate,memory_recursiveprot,memory_hugetlb_accounting 0 0
none /sys/fs/pstore pstore rw,nosuid,nodev,noexec,relatime 0 0
bpf /sys/fs/bpf bpf rw,nosuid,nodev,noexec,relatime,mode=700 0 0
systemd-1 /proc/sys/fs/binfmt_misc autofs rw,relatime,fd=41,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=19633 0 0
none /run/credentials/systemd-journald.service tmpfs ro,nosuid,nodev,noexec,relatime,nosymfollow,size=1024k,nr_inodes=1024,mode=700,inode64,noswap 0 0
tracefs /sys/kernel/tracing tracefs rw,nosuid,nodev,noexec,relatime 0 0
hugetlbfs /dev/hugepages hugetlbfs rw,nosuid,nodev,relatime,pagesize=2M 0 0
configfs /sys/kernel/config configfs rw,nosuid,nodev,noexec,relatime 0 0
mqueue /dev/mqueue mqueue rw,nosuid,nodev,noexec,relatime 0 0
fusectl /sys/fs/fuse/connections fusectl rw,nosuid,nodev,noexec,relatime 0 0
debugfs /sys/kernel/debug debugfs rw,nosuid,nodev,noexec,relatime 0 0
none /run/credentials/systemd-resolved.service tmpfs ro,nosuid,nodev,noexec,relatime,nosymfollow,size=1024k,nr_inodes=1024,mode=700,inode64,noswap 0 0
none /run/credentials/systemd-networkd.service tmpfs ro,nosuid,nodev,noexec,relatime,nosymfollow,size=1024k,nr_inodes=1024,mode=700,inode64,noswap 0 0
tmpfs /tmp tmpfs rw,nosuid,nodev,size=16267768k,nr_inodes=1048576,inode64,usrquota 0 0
/dev/nvme1n1p2 /var/log btrfs rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=258,subvol=/@log 0 0
/dev/nvme1n1p2 /var/cache/pacman/pkg btrfs rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=259,subvol=/@pkg 0 0
/dev/nvme1n1p2 /home btrfs rw,relatime,compress=zstd:3,ssd,discard=async,space_cache=v2,subvolid=257,subvol=/@home 0 0
/dev/nvme1n1p1 /boot vfat rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,utf8,errors=remount-ro 0 0
binfmt_misc /proc/sys/fs/binfmt_misc binfmt_misc rw,nosuid,nodev,noexec,relatime 0 0
tmpfs /run/user/1000 tmpfs rw,nosuid,nodev,relatime,size=3253552k,nr_inodes=813388,mode=700,uid=1000,gid=1000,inode64 0 0
gvfsd-fuse /run/user/1000/gvfs fuse.gvfsd-fuse rw,nosuid,nodev,relatime,user_id=1000,group_id=1000 0 0
portal /run/user/1000/doc fuse.portal rw,nosuid,nodev,relatime,user_id=1000,group_id=1000 0 0
/dev/sda1 /run/media/pfornage/E6DB-5F09 vfat rw,nosuid,nodev,relatime,uid=1000,gid=1000,fmask=0022,dmask=0022,codepage=437,iocharset=ascii,shortname=mixed,showexec,utf8,flush,errors=remount-ro 0 0

     */

    let mut results = Vec::new();

    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let source = parts.next();
        let target = parts.next();
        let fstype = parts.next();

        let (Some(source), Some(target), Some(fstype)) = (source, target, fstype) else { continue };

        if !is_usb_fstype(fstype) {
            continue;
        }

        let target = target.replace(r"\040", " ");
        let target_path = PathBuf::from(target);

        if is_removable_mount_root(&target_path) {
            let size_bytes = device_size_bytes(source).unwrap_or(0);
            let name = usb_drive_name(source, &target_path);
            results.push(UsbDrive {
                name,
                mountpoint: target_path,
                size_bytes,
            });
        }
    }

    Ok(results)
}

pub fn size_to_string(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    if size >= 10.0 {
        format!("{:.0} {}", size, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

fn device_size_bytes(source: &str) -> Option<u64> {
    if !source.starts_with("/dev/") {
        return None;
    }

    let source_path = Path::new(source);
    let device_path = match fs::canonicalize(source_path) {
        Ok(path) => path,
        Err(err) => {
            trace_targeted!(FS, "Failed to canonicalize device path: {}", err);
            source_path.to_path_buf()
        }
    };
    let device_name = device_path.file_name()?.to_string_lossy();

    let sysfs_size = Path::new("/sys/class/block")
        .join(device_name.as_ref())
        .join("size");
    let size_sectors = fs::read_to_string(&sysfs_size).ok()?.trim().parse::<u64>().ok()?;
    Some(size_sectors.saturating_mul(512))
}

fn usb_drive_name(source: &str, mountpoint: &Path) -> String {
    if let Some(label) = label_from_device(source) {
        return label;
    }

    if let Some(label) = label_from_mountpoint(mountpoint) {
        return label;
    }

    mountpoint.display().to_string()
}

fn label_from_device(source: &str) -> Option<String> {
    if !source.starts_with("/dev/") {
        return None;
    }

    let device_path = Path::new(source);
    let device_path = fs::canonicalize(device_path).ok()?;
    let by_label_dir = Path::new("/dev/disk/by-label");
    let entries = fs::read_dir(by_label_dir).ok()?;

    for entry in entries {
        let entry = entry.ok()?;
        let label = entry.file_name().to_string_lossy().to_string();
        let target = fs::canonicalize(entry.path()).ok()?;
        if target == device_path {
            return Some(label);
        }
    }

    None
}

fn label_from_mountpoint(mountpoint: &Path) -> Option<String> {
    mountpoint
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.is_empty())
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
