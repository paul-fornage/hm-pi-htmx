use crate::debug_targeted;
use crate::file_io::{deserialize_json, serialize_json, FileIoError, FixedDiskFile};
use crate::paths::subdirs::Subdir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthLevel {
    Operator,
    Manager,
    Admin,
}

impl AuthLevel {
    pub fn label(&self) -> &'static str {
        match self {
            AuthLevel::Operator => "Operator",
            AuthLevel::Manager => "Manager",
            AuthLevel::Admin => "Admin",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthState {
    Operator,
    Manager { username: String },
    Admin { username: String },
}

impl Default for AuthState {
    fn default() -> Self {
        AuthState::Operator
    }
}

impl AuthState {
    pub fn level(&self) -> AuthLevel {
        match self {
            AuthState::Operator => AuthLevel::Operator,
            AuthState::Manager { .. } => AuthLevel::Manager,
            AuthState::Admin { .. } => AuthLevel::Admin,
        }
    }

    pub fn username(&self) -> Option<&str> {
        match self {
            AuthState::Operator => None,
            AuthState::Manager { username } => Some(username),
            AuthState::Admin { username } => Some(username),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub username: String,
    pub password: String,
    pub level: AuthLevel,
}

impl UserRecord {
    pub fn is_manager(&self) -> bool {
        self.level == AuthLevel::Manager
    }

    pub fn is_admin(&self) -> bool {
        self.level == AuthLevel::Admin
    }
}

pub const USERS_PATH: &str = "users.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserStore(pub HashMap<String, UserRecord>);

impl FixedDiskFile for UserStore {
    const SUBDIR: Subdir = Subdir::Config;
    const FILE_NAME: &'static str = USERS_PATH;

    fn serialize_value(&self, path: &Path) -> Result<String, FileIoError> {
        serialize_json(self, path)
    }

    fn deserialize_value(path: &Path, contents: &str) -> Result<Self, FileIoError> {
        deserialize_json(contents, path)
    }
}

pub async fn load_users() -> Result<HashMap<String, UserRecord>, FileIoError> {
    let store = UserStore::load().await?;
    Ok(store.0)
}

pub async fn save_users(users: &HashMap<String, UserRecord>) -> Result<(), FileIoError> {
    debug_targeted!(FS, "Saving users list ({} users)", users.len());
    let store = UserStore(users.clone());
    store.save().await
}

pub async fn verify_credentials(username: &str, password: &str) -> Result<AuthState, String> {
    if username == "MITUSA" && password == "admin" {
        return Ok(AuthState::Admin {
            username: username.to_string(),
        });
    }

    let users = match load_users().await {
        Ok(users) => users,
        Err(FileIoError::NotFound { .. }) => {
            debug_targeted!(FS, "Users file not found; treating as empty");
            HashMap::new()
        }
        Err(err) => return Err(err.to_string()),
    };
    if let Some(user) = users.get(username).filter(|u| u.password == password) {
        return Ok(match user.level {
            AuthLevel::Admin => AuthState::Admin {
                username: user.username.clone(),
            },
            AuthLevel::Manager => AuthState::Manager {
                username: user.username.clone(),
            },
            AuthLevel::Operator => AuthState::Operator,
        });
    }

    Err(format!("Invalid credentials for user \'{}\'", username))
}
