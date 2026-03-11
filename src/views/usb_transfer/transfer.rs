use std::path::{Path, PathBuf};
use std::io;

use strum::VariantArray;
use tokio::fs;

use crate::file_io::validate_filename;
use crate::paths::subdirs::Subdir;
use crate::paths::DEFAULT_ROOT_FOLDER;
use crate::{debug_targeted, error_targeted, warn_targeted, LOCAL_SUBDIR_PATHS};

use super::types::{ReloadTarget, TransferDirection, UsbTransferForm};

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
    #[error("No files to copy in {label}.")]
    EmptySource { label: String },
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
            TransferError::EmptySource { label } => format!("No files to copy in {label}."),
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

pub struct PreparedBulkCopy {
    pub pairs: Vec<(PathBuf, PathBuf)>,
    pub reload_target: ReloadTarget,
    pub subdir: Subdir,
    pub source_count: usize,
    pub conflict_count: usize,
}

pub async fn list_local_sections() -> Vec<SubdirSection> {
    let mut sections = Vec::with_capacity(Subdir::VARIANTS.len());

    for subdir in Subdir::VARIANTS {
        let dir = LOCAL_SUBDIR_PATHS.get(*subdir).clone();

        let (files, error) = match ensure_local_dir(&dir).await {
            Ok(()) => match list_files_in_dir(&dir, *subdir).await {
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

        let (files, error) = match list_files_in_dir(&dir, *subdir).await {
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
    let file_name = form
        .file_name
        .as_deref()
        .unwrap_or_default();

    validate_filename(file_name)
        .map_err(|err| TransferError::InvalidFilename(err.to_string()))?;

    let subdir = parse_subdir(&form.subdir)?;
    let mountpoint = parse_mountpoint(&form.usb_mountpoint)?;

    match form.direction {
        TransferDirection::LocalToUsb => {
            let source_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&source_dir).await?;

            let destination_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            Ok(PreparedCopy {
                source: source_dir.join(file_name_with_ext(subdir, file_name)),
                destination: destination_dir.join(file_name_with_ext(subdir, file_name)),
                reload_target: ReloadTarget::Usb,
            })
        }
        TransferDirection::UsbToLocal => {
            let source_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            let destination_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&destination_dir).await?;

            Ok(PreparedCopy {
                source: source_dir.join(file_name_with_ext(subdir, file_name)),
                destination: destination_dir.join(file_name_with_ext(subdir, file_name)),
                reload_target: ReloadTarget::Local,
            })
        }
    }
}

pub async fn prepare_bulk_copy(form: &UsbTransferForm) -> Result<PreparedBulkCopy, TransferError> {
    let subdir = parse_subdir(&form.subdir)?;
    let mountpoint = parse_mountpoint(&form.usb_mountpoint)?;

    let (source_dir, destination_dir, reload_target) = match form.direction {
        TransferDirection::LocalToUsb => {
            let source_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&source_dir).await?;
            let destination_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            (source_dir, destination_dir, ReloadTarget::Usb)
        }
        TransferDirection::UsbToLocal => {
            let source_dir = ensure_usb_subdir(&mountpoint, subdir).await?;
            let destination_dir = LOCAL_SUBDIR_PATHS.get(subdir).clone();
            ensure_local_dir(&destination_dir).await?;
            (source_dir, destination_dir, ReloadTarget::Local)
        }
    };

    let source_files = list_files_in_dir(&source_dir, subdir).await?;
    if source_files.is_empty() {
        return Err(TransferError::EmptySource {
            label: subdir_label(subdir).to_string(),
        });
    }

    let destination_files = list_files_in_dir(&destination_dir, subdir).await?;

    let conflict_count = if destination_files.is_empty() {
        0
    } else {
        let destination_set: std::collections::HashSet<_> = destination_files.into_iter().collect();
        source_files.iter().filter(|name| destination_set.contains(*name)).count()
    };

    let pairs = source_files
        .iter()
        .map(|name| {
            (
                source_dir.join(file_name_with_ext(subdir, name)),
                destination_dir.join(file_name_with_ext(subdir, name)),
            )
        })
        .collect::<Vec<_>>();

    Ok(PreparedBulkCopy {
        pairs,
        reload_target,
        subdir,
        source_count: source_files.len(),
        conflict_count,
    })
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

pub async fn execute_bulk_copy(plan: &PreparedBulkCopy) -> Result<(), TransferError> {
    for (source, destination) in &plan.pairs {
        debug_targeted!(FS, "Copying file from {} to {}", source.display(), destination.display());
        fs::copy(source, destination)
            .await
            .map_err(|e| TransferError::Copy {
                from: source.clone(),
                to: destination.clone(),
                source: e,
            })?;
    }
    Ok(())
}

fn file_name_with_ext(subdir: Subdir, name: &str) -> String {
    format!("{}.{}", name, extension_for_subdir(subdir))
}

fn parse_subdir(value: &str) -> Result<Subdir, TransferError> {
    Subdir::VARIANTS
        .iter()
        .copied()
        .find(|subdir| subdir.path() == value)
        .ok_or_else(|| TransferError::InvalidSubdir(value.to_string()))
}

pub fn subdir_label(subdir: Subdir) -> &'static str {
    match subdir {
        Subdir::MotionProfiles => "Motion profiles",
        Subdir::WeldProfiles => "Weld profiles",
        Subdir::Logs => "Logs",
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

async fn list_files_in_dir(dir: &Path, subdir: Subdir) -> Result<Vec<String>, TransferError> {
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

                let file_name = match path.file_name().and_then(|s| s.to_str()) {
                    Some(name) => name,
                    None => {
                        skipped_non_utf8 += 1;
                        continue;
                    }
                };

                if !is_allowed_file(subdir, file_name) {
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

fn extension_for_subdir(subdir: Subdir) -> &'static str {
    match subdir {
        Subdir::Logs => "txt",
        _ => "json",
    }
}

fn is_allowed_file(subdir: Subdir, file_name: &str) -> bool {
    let mut split = file_name.split('.');
    if split.next().is_none() { return false; };
    let ext = split.last();

    ext == Some(extension_for_subdir(subdir))
}
