use std::path::PathBuf;
use num_enum::{IntoPrimitive};
use crate::paths::{full_path_for_subdir, full_path_for_subdir_verified, local_data_root_ensuring_exists};

#[derive(Debug, Clone, Copy, IntoPrimitive)]
#[repr(u8)]
pub enum Subdir {
    MotionProfiles = 0,
    WeldProfiles,
    Logs,
    Users,
    Config,
}

pub struct SubdirPaths {
    pub paths: [PathBuf; 5]
}
impl SubdirPaths {
    pub fn get(&self, subdir: Subdir) -> &PathBuf {
        let offset: u8 = subdir.into();
        &self.paths[offset as usize]
    }
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