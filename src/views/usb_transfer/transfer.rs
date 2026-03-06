use std::path::{Path, PathBuf};
use std::{io};

use tokio::fs;
use strum::VariantArray;

use crate::file_io::validate_filename;
use crate::paths::subdirs::Subdir;
use crate::paths::DEFAULT_ROOT_FOLDER;
use crate::{debug_targeted, warn_targeted, error_targeted, LOCAL_SUBDIR_PATHS};

use super::types::{ReloadTarget, TransferDirection, UsbTransferForm};

pub const ALLOWED_EXTENSION: &str = "json";

#[derive(Debug, Clone)]
pub struct SubdirSection {
    pub subdir: Subdir,
    pub label: &'static str,
    pub files: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error("USB device removed before operation.")]
    DeviceRemoved,
    #[error("Invalid USB mountpoint.")]
    InvalidMountpoint,
    #[error("Invalid file name: {0}")]
    InvalidFilename(String),
    #[error("Invalid directory selection: {0}")]
    InvalidSubdir(String),
    #[error("I/O error at {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
    #[error("Failed to read directory {path}: {source}")]
    ReadDir { path: PathBuf, source: io::Error },
    #[error("Failed to copy file from {from} to {to}: {source}")]
    Copy { from: PathBuf, to: PathBuf, source: io::Error },
}

impl TransferError {
    pub fn user_message(&self) -> String {
        match self {
            TransferError::DeviceRemoved => "USB device removed before operation.".to_string(),
            TransferError::InvalidMountpoint => "Invalid or missing USB mountpoint.".to_string(),
            TransferError::InvalidFilename(msg) => format!("Invalid file name: {msg}"),
            TransferError::InvalidSubdir(msg) => format!("Invalid directory selection: {msg}"),
            TransferError::Io { source, .. } => format!("I/O error: {source}"),
            TransferError::ReadDir { source, .. } => format!("Failed to read directory: {source}"),
            TransferError::Copy { source, .. } => format!("Failed to copy file: {source}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreparedCopy {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub reload_target: ReloadTarget,
}

pub async fn list_local_sections() -> Vec<SubdirSection> {
    let mut sections = Vec::with_capacity(Subdir::VARIANTS.len());

    for subdir in Subdir::VARIANTS {
        let dir = LOCAL_SUBDIR_PATHS.get(*subdir).clone();

        let (files, error) = match ensure_local_dir(&dir).await {
            Ok(()) => match list_files_in_dir(&dir).await {
                Ok(files) => (files, None),
                Err(err) => {
                    error_targeted!(FS, "Failed to list local files for {}: {}", dir.display(), err);
                    (Vec::new(), Some(err.user_message()))
                }
            },
            Err(err) => {
                error_targeted!(FS, "Failed to ensure local directory {}: {}", dir.display(), err);
                (Vec::new(), Some(err.user_message()))
            }
        };

        sections.push(SubdirSection {
            subdir: *subdir,
            label: subdir_label(*subdir),
            files,
            error,
        });
    }

    sections
}

pub async fn list_usb_sections(mountpoint: &Path) -> Vec<SubdirSection> {
    let mut sections = Vec::with_capacity(Subdir::VARIANTS.len());

    for subdir in Subdir::VARIANTS {
        let dir = match ensure_usb_subdir(mountpoint, *subdir).await {
            Ok(path) => path,
            Err(err) => {
                error_targeted!(FS, "Failed to ensure USB directory for {}: {}", subdir.path(), err);
                sections.push(SubdirSection {
                    subdir: *subdir,
                    label: subdir_label(*subdir),
                    files: Vec::new(),
                    error: Some(err.user_message()),
                });
                continue;
            }
        };

        let (files, error) = match list_files_in_dir(&dir).await {
            Ok(files) => (files, None),
            Err(err) => {
                error_targeted!(FS, "Failed to list USB files for {}: {}", dir.display(), err);
                (Vec::new(), Some(err.user_message()))
            }
        };

        sections.push(SubdirSection {
            subdir: *subdir,
            label: subdir_label(*subdir),
            files,
            error,
        });
    }

    sections
}

pub async fn prepare_copy(form: &UsbTransferForm) -> Result<PreparedCopy, TransferError> {
    validate_filename(&form.file_name)
        .map_err(|err| TransferError::InvalidFilename(err.to_string()))?;

    let subdir = parse_subdir(&form.subdir)?;
    let mountpoint = parse_mountpoint(&form.usb_mountpoint)?;

    match form.direction {
        TransferDirection::LocalToUsb => {
            let source_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&source_dir).await?;

            let destination_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            Ok(PreparedCopy {
                source: source_dir.join(file_name_with_ext(&form.file_name)),
                destination: destination_dir.join(file_name_with_ext(&form.file_name)),
                reload_target: ReloadTarget::Usb,
            })
        }
        TransferDirection::UsbToLocal => {
            let source_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            let destination_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&destination_dir).await?;

            Ok(PreparedCopy {
                source: source_dir.join(file_name_with_ext(&form.file_name)),
                destination: destination_dir.join(file_name_with_ext(&form.file_name)),
                reload_target: ReloadTarget::Local,
            })
        }
    }
}

pub async fn destination_exists(path: &Path) -> Result<bool, TransferError> {
    fs::try_exists(path)
        .await
        .map_err(|e| TransferError::Io { path: path.to_path_buf(), source: e })
}

pub async fn execute_copy(plan: &PreparedCopy) -> Result<(), TransferError> {
    debug_targeted!(FS, "Copying file from {} to {}", plan.source.display(), plan.destination.display());
    fs::copy(&plan.source, &plan.destination)
        .await
        .map(|_| ())
        .map_err(|e| TransferError::Copy {
            from: plan.source.clone(),
            to: plan.destination.clone(),
            source: e,
        })
}

fn file_name_with_ext(name: &str) -> String {
    format!("{}.{}", name, ALLOWED_EXTENSION)
}

fn parse_subdir(value: &str) -> Result<Subdir, TransferError> {
    Subdir::VARIANTS
        .iter()
        .copied()
        .find(|subdir| subdir.path() == value)
        .ok_or_else(|| TransferError::InvalidSubdir(value.to_string()))
}

fn subdir_label(subdir: Subdir) -> &'static str {
    match subdir {
        Subdir::MotionProfiles => "Motion profiles",
        Subdir::WeldProfiles => "Weld profiles",
        Subdir::Logs => "Logs",
        Subdir::Users => "Users",
        Subdir::Config => "Config",
        Subdir::ConnectionProfiles => "Connection profiles",
    }
}

fn parse_mountpoint(value: &str) -> Result<PathBuf, TransferError> {
    if value.trim().is_empty() {
        return Err(TransferError::InvalidMountpoint);
    }

    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err(TransferError::InvalidMountpoint);
    }

    Ok(path)
}

async fn ensure_local_dir(path: &Path) -> Result<(), TransferError> {
    fs::create_dir_all(path)
        .await
        .map_err(|e| TransferError::Io { path: path.to_path_buf(), source: e })
}

async fn ensure_usb_mountpoint(path: &Path) -> Result<(), TransferError> {
    match fs::try_exists(path).await {
        Ok(true) => {}
        Ok(false) => return Err(TransferError::DeviceRemoved),
        Err(e) => {
            return Err(TransferError::Io { path: path.to_path_buf(), source: e });
        }
    }

    let metadata = fs::metadata(path)
        .await
        .map_err(|e| TransferError::Io { path: path.to_path_buf(), source: e })?;

    if !metadata.is_dir() {
        return Err(TransferError::InvalidMountpoint);
    }

    Ok(())
}

async fn ensure_usb_subdir(mountpoint: &Path, subdir: Subdir) -> Result<PathBuf, TransferError> {
    ensure_usb_mountpoint(mountpoint).await?;

    let root = mountpoint.join(DEFAULT_ROOT_FOLDER);
    create_dir_step(&root, mountpoint).await?;

    let subdir_path = root.join(subdir.path());
    create_dir_step(&subdir_path, mountpoint).await?;

    Ok(subdir_path)
}

async fn create_dir_step(path: &Path, mountpoint: &Path) -> Result<(), TransferError> {
    match fs::try_exists(path).await {
        Ok(true) => {
            let meta = fs::metadata(path)
                .await
                .map_err(|e| TransferError::Io { path: path.to_path_buf(), source: e })?;
            if meta.is_dir() {
                return Ok(());
            }
            return Err(TransferError::Io {
                path: path.to_path_buf(),
                source: io::Error::new(io::ErrorKind::Other, "path exists but is not a directory"),
            });
        }
        Ok(false) => {}
        Err(e) => {
            return Err(TransferError::Io { path: path.to_path_buf(), source: e });
        }
    }

    match fs::create_dir(path).await {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            if fs::try_exists(mountpoint).await.unwrap_or(false) {
                Err(TransferError::Io { path: path.to_path_buf(), source: e })
            } else {
                Err(TransferError::DeviceRemoved)
            }
        }
        Err(e) => Err(TransferError::Io { path: path.to_path_buf(), source: e }),
    }
}

async fn list_files_in_dir(dir: &Path) -> Result<Vec<String>, TransferError> {
    let mut entries = fs::read_dir(dir)
        .await
        .map_err(|e| TransferError::ReadDir { path: dir.to_path_buf(), source: e })?;

    let mut names = Vec::new();
    let mut skipped_non_utf8 = 0usize;
    let mut skipped_invalid = 0usize;
    let mut skipped_non_files = 0usize;
    let mut file_type_errors = 0usize;

    loop {
        match entries.next_entry().await {
            Ok(Some(entry)) => {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) != Some(ALLOWED_EXTENSION) {
                    continue;
                }

                match entry.file_type().await {
                    Ok(ft) if ft.is_file() => {}
                    Ok(_) => {
                        skipped_non_files += 1;
                        continue;
                    }
                    Err(_) => {
                        file_type_errors += 1;
                        continue;
                    }
                }

                let stem = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(stem) => stem.to_string(),
                    None => {
                        skipped_non_utf8 += 1;
                        continue;
                    }
                };

                if validate_filename(&stem).is_err() {
                    skipped_invalid += 1;
                    continue;
                }

                names.push(stem);
            }
            Ok(None) => break,
            Err(e) => {
                return Err(TransferError::ReadDir { path: dir.to_path_buf(), source: e });
            }
        }
    }

    if skipped_non_utf8 > 0 || skipped_invalid > 0 || skipped_non_files > 0 || file_type_errors > 0 {
        warn_targeted!(
            FS,
            "Skipped {} invalid/{} non-utf8/{} non-file/{} unreadable entries in {}",
            skipped_invalid,
            skipped_non_utf8,
            skipped_non_files,
            file_type_errors,
            dir.display()
        );
    }

    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    Ok(names)
}
