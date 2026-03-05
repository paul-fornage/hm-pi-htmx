use std::path::PathBuf;
use crate::paths::{full_path_for_subdir, full_path_for_subdir_verified, local_data_root_ensuring_exists};

#[derive(Debug, Clone, Copy)]
pub enum Subdir {
    MotionProfiles,
    WeldProfiles,
    Logs,
    Users,
    Config,
}

impl Subdir {
    pub fn path(&self) -> &'static str {
        match self {
            Self::MotionProfiles => "motion-profiles",
            Self::WeldProfiles => "weld-profiles",
            Self::Logs => "logs",
            Self::Users => "users",
            Self::Config => "config",
        }
    }

    pub fn full_local_path(&self) -> PathBuf {
        full_path_for_subdir(*self)
    }

    pub fn full_local_path_ensuring_exists(&self) -> Result<PathBuf, std::io::Error> {
        full_path_for_subdir_verified(*self)
    }
}