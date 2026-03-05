use std::future::Future;
use std::path::PathBuf;
use futures::future::join_all;
use num_enum::{IntoPrimitive};
use strum::VariantArray;
use crate::paths::{full_path_for_subdir, full_path_for_subdir_verified, local_data_root_ensuring_exists};

#[derive(Debug, Clone, Copy, IntoPrimitive, VariantArray)]
#[repr(u8)]
pub enum Subdir {
    MotionProfiles = 0,
    WeldProfiles,
    Logs,
    Users,
    Config,
    ConnectionProfiles
}

pub struct SubdirPaths {
    pub paths: [PathBuf; Subdir::VARIANTS.len()]
}
impl SubdirPaths {
    pub fn get(&self, subdir: Subdir) -> &PathBuf {
        let offset: u8 = subdir.into();
        &self.paths[offset as usize]
    }

    pub fn new_mapped<F>(mut mapper: F) -> Self
    where
        F: FnMut(Subdir) -> PathBuf,
    {
        let paths: [PathBuf; Subdir::VARIANTS.len()] = std::array::from_fn(
            |i| mapper(Subdir::VARIANTS[i]));
        Self { paths }
    }

    pub async fn new_async_mapped<F, Fut>(mut mapper: F) -> Self
    where
        F: FnMut(Subdir) -> Fut,
        Fut: Future<Output = PathBuf>,
    {
        let path_futures: [Fut; Subdir::VARIANTS.len()] = std::array::from_fn(
            |i| mapper(Subdir::VARIANTS[i]));

        let resolved_paths_vec = join_all(path_futures).await;

        // Since we know `join_all` returns exactly as many items as we gave it,
        // `unwrap()` will never panic here.
        let paths: [PathBuf; Subdir::VARIANTS.len()] = resolved_paths_vec.try_into().unwrap();

        Self { paths }
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
            Self::ConnectionProfiles => "connection-profiles"
        }
    }

    pub fn full_local_path(&self) -> PathBuf {
        full_path_for_subdir(*self)
    }

    pub fn full_local_path_ensuring_exists(&self) -> Result<PathBuf, std::io::Error> {
        full_path_for_subdir_verified(*self)
    }
}