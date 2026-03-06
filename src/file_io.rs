use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use tokio::fs;

use crate::paths::subdirs::Subdir;
use crate::{warn_targeted, LOCAL_SUBDIR_PATHS};

#[derive(Debug, thiserror::Error)]
pub enum FileIoError {
    #[error("I/O error at {path}: {source}")]
    Io { path: PathBuf, source: std::io::Error },
    #[error("File not found: {path}")]
    NotFound { path: PathBuf },
    #[error("Failed to parse JSON at {path}: {source}")]
    Serde { path: PathBuf, source: serde_json::Error },
    #[error("Validation error: {message}")]
    Validation { message: String },
}

pub fn validate_filename(name: &str) -> Result<(), FileIoError> {
    if name.is_empty() {
        return Err(FileIoError::Validation {
            message: "Name cannot be empty".to_string(),
        });
    }

    if name.len() > 240 {
        return Err(FileIoError::Validation {
            message: "Name too long (maximum 240 characters)".to_string(),
        });
    }

    if name != name.trim() {
        return Err(FileIoError::Validation {
            message: "Name cannot start or end with whitespace".to_string(),
        });
    }

    if name == "." || name == ".." {
        return Err(FileIoError::Validation {
            message: "Invalid name".to_string(),
        });
    }

    for c in name.chars() {
        match c {
            '/' | '\\' => {
                return Err(FileIoError::Validation {
                    message: "Name cannot contain path separators ('/' or '\\')".to_string(),
                });
            }
            '\0' => {
                return Err(FileIoError::Validation {
                    message: "Name cannot contain null bytes".to_string(),
                });
            }
            '<' | '>' | ':' | '"' | '|' | '?' | '*' => {
                return Err(FileIoError::Validation {
                    message: format!("Name cannot contain '{}'", c),
                });
            }
            c if c.is_control() => {
                return Err(FileIoError::Validation {
                    message: "Name cannot contain control characters".to_string(),
                });
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn serialize_json<T: serde::Serialize>(value: &T, path: &Path) -> Result<String, FileIoError> {
    serde_json::to_string_pretty(value).map_err(|e| FileIoError::Serde {
        path: path.to_path_buf(),
        source: e,
    })
}

pub fn deserialize_json<T: serde::de::DeserializeOwned>(
    contents: &str,
    path: &Path,
) -> Result<T, FileIoError> {
    serde_json::from_str(contents).map_err(|e| FileIoError::Serde {
        path: path.to_path_buf(),
        source: e,
    })
}

fn map_io_error(path: &Path, err: std::io::Error) -> FileIoError {
    if err.kind() == ErrorKind::NotFound {
        FileIoError::NotFound {
            path: path.to_path_buf(),
        }
    } else {
        FileIoError::Io {
            path: path.to_path_buf(),
            source: err,
        }
    }
}

async fn ensure_dir(path: &Path) -> Result<(), FileIoError> {
    fs::create_dir_all(path)
        .await
        .map_err(|e| FileIoError::Io {
            path: path.to_path_buf(),
            source: e,
        })
}

async fn read_to_string(path: &Path) -> Result<String, FileIoError> {
    fs::read_to_string(path).await.map_err(|e| map_io_error(path, e))
}

async fn write_string(path: &Path, contents: &str) -> Result<(), FileIoError> {
    fs::write(path, contents).await.map_err(|e| FileIoError::Io {
        path: path.to_path_buf(),
        source: e,
    })
}

#[async_trait]
pub trait FixedDiskFile: Sized + Send + Sync {
    const SUBDIR: Subdir;
    const FILE_NAME: &'static str;

    fn serialize_value(&self, path: &Path) -> Result<String, FileIoError>;
    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError>;

    async fn save(&self) -> Result<(), FileIoError> {
        
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        ensure_dir(&dir).await?;

        let path = dir.join(Self::FILE_NAME);
        let contents = self.serialize_value(&path)?;
        write_string(&path, &contents).await?;
        Ok(())
    }

    async fn load() -> Result<Self, FileIoError> {
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        let path = dir.join(Self::FILE_NAME);
        let contents = read_to_string(&path).await?;
        Self::deserialize_value(&path, &contents)
    }
}

#[async_trait]
pub trait NamedDiskFile: Sized + Send + Sync {
    const SUBDIR: Subdir;
    const EXT: &'static str;

    fn serialize_value(value: &Self, path: &Path) -> Result<String, FileIoError>;
    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError>;

    async fn save(name: &str, value: &Self) -> Result<(), FileIoError> {
        validate_filename(name)?;
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        ensure_dir(&dir).await?;

        let path = dir.join(format!("{}.{}", name, Self::EXT));
        let contents = Self::serialize_value(value, &path)?;
        write_string(&path, &contents).await?;
        Ok(())
    }

    async fn load(name: &str) -> Result<Self, FileIoError> {
        validate_filename(name)?;
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        let path = dir.join(format!("{}.{}", name, Self::EXT));
        let contents = read_to_string(&path).await?;
        Self::deserialize_value(&path, &contents)
    }

    async fn delete(name: &str) -> Result<(), FileIoError> {
        validate_filename(name)?;
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        let path = dir.join(format!("{}.{}", name, Self::EXT));
        fs::remove_file(&path)
            .await
            .map_err(|e| map_io_error(&path, e))
    }

    async fn list() -> Result<Vec<String>, FileIoError> {
        let dir = LOCAL_SUBDIR_PATHS.get(Self::SUBDIR);
        let mut entries = match fs::read_dir(&dir).await {
            Ok(entries) => entries,
            Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => {
                return Err(FileIoError::Io {
                    path: dir.clone(),
                    source: e,
                })
            }
        };

        let mut names = Vec::new();
        loop {
            match entries.next_entry().await {
                Ok(Some(entry)) => {
                    let path = entry.path();

                    if path.extension().and_then(|s| s.to_str()) != Some(Self::EXT) {
                        continue;
                    }

                    match entry.file_type().await {
                        Ok(ft) if ft.is_file() => {}
                        Ok(_) => continue,
                        Err(e) => {
                            warn_targeted!(FS, "Failed to read file type for {:?}: {}", path, e);
                            continue;
                        }
                    }

                    let stem = match path.file_stem().and_then(|s| s.to_str()) {
                        Some(stem) => stem.to_string(),
                        None => {
                            warn_targeted!(FS, "Skipping non-utf8 filename: {:?}", path);
                            continue;
                        }
                    };

                    if let Err(err) = validate_filename(&stem) {
                        warn_targeted!(FS, "Skipping invalid filename {:?}: {}", path, err);
                        continue;
                    }

                    names.push(stem);
                }
                Ok(None) => break,
                Err(e) => {
                    warn_targeted!(FS, "Failed to read directory entry in {:?}: {}", dir, e);
                    continue;
                }
            }
        }

        names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        Ok(names)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_validation() {
        // Valid names
        assert!(validate_filename("my_profile").is_ok());
        assert!(validate_filename("Profile-123").is_ok());
        assert!(validate_filename("Test Profile").is_ok());
        assert!(validate_filename("profile_2024_01_07").is_ok());

        // Invalid: empty
        assert!(validate_filename("").is_err());

        // Invalid: path traversal
        assert!(validate_filename("/etc/passwd").is_err());
        assert!(validate_filename("../secret").is_err());
        assert!(validate_filename("foo/bar").is_err());
        assert!(validate_filename("foo\\bar").is_err());

        // Invalid: reserved names
        assert!(validate_filename(".").is_err());
        assert!(validate_filename("..").is_err());

        // Invalid: whitespace issues
        assert!(validate_filename(" profile").is_err());
        assert!(validate_filename("profile ").is_err());
        assert!(validate_filename("  ").is_err());

        // Invalid: dangerous characters
        assert!(validate_filename("profile\0name").is_err());
        assert!(validate_filename("profile<name").is_err());
        assert!(validate_filename("profile>name").is_err());
        assert!(validate_filename("profile:name").is_err());
        assert!(validate_filename("profile\"name").is_err());
        assert!(validate_filename("profile|name").is_err());
        assert!(validate_filename("profile?name").is_err());
        assert!(validate_filename("profile*name").is_err());

        // Invalid: too long
        let long_name = "a".repeat(250);
        assert!(validate_filename(&long_name).is_err());
    }
}
