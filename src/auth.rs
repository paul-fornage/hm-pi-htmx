use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::debug_targeted;

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

pub async fn load_users() -> Result<HashMap<String, UserRecord>, String> {
    match tokio::fs::read_to_string(USERS_PATH).await {
        Ok(contents) => serde_json::from_str(&contents).map_err(|e| e.to_string()),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok(HashMap::new())
            } else {
                Err(err.to_string())
            }
        }
    }
}

pub async fn save_users(users: &HashMap<String, UserRecord>) -> Result<(), String> {
    debug_targeted!(FS, "Saving users list ({} users)", users.len());
    let json = serde_json::to_string_pretty(users).map_err(|e| e.to_string())?;
    tokio::fs::write(USERS_PATH, json)
        .await
        .map_err(|e| e.to_string())
}

pub async fn verify_credentials(username: &str, password: &str) -> Result<AuthState, String> {
    if username == "MITUSA" && password == "admin" {
        return Ok(AuthState::Admin {
            username: username.to_string(),
        });
    }

    let users = load_users().await?;
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
