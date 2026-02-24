use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::collections::HashMap;

use crate::auth::{self, AuthLevel, UserRecord};
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{debug_targeted, error_targeted, info_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "views/users.html")]
pub struct UsersTemplate {
    pub header: HeaderContext,
    pub users: HashMap<String, UserRecord>,
    pub load_error: Option<String>,
}

impl UsersTemplate {
    pub fn sorted_users(&self) -> Vec<&UserRecord> {
        let mut list: Vec<_> = self.users.values().collect();
        list.sort_by_key(|u| &u.username);
        list
    }

    pub fn has_load_error(&self) -> bool {
        self.load_error.is_some()
    }
}

impl ViewTemplate for UsersTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::Users;
}

#[derive(Template, WebTemplate)]
#[template(path = "components/users/user-rows.html")]
pub struct UserRowsTemplate {
    pub users: Vec<UserRecord>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/users/action-status.html")]
pub struct ActionStatusTemplate {
    pub result: Result<String, String>,
}

impl ActionStatusTemplate {
    pub fn is_ok(&self) -> bool {
        self.result.is_ok()
    }

    pub fn message(&self) -> &str {
        match &self.result {
            Ok(msg) => msg,
            Err(err) => err,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/users/modal.html")]
pub struct UserModalTemplate {
    pub user: Option<UserRecord>, // None means we are adding
}

#[derive(Deserialize, Debug)]
pub struct UserActionForm {
    pub original_username: Option<String>,
    pub username: String,
    pub password: String,
    pub level: AuthLevel,
}

#[derive(Deserialize, Debug)]
pub struct DeleteQuery {
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct EditQuery {
    pub username: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(AppView::Users.url(), get(show_users))
        .route("/users/rows", get(user_rows))
        .route("/users/add", post(add_user))
        .route("/users/edit", post(edit_user))
        .route("/users/delete", delete(delete_user))
        .route("/users/modal/add", get(add_modal))
        .route("/users/modal/edit", get(edit_modal))
}

pub async fn show_users(State(state): State<AppState>) -> impl IntoResponse {
    info_targeted!(HTTP, "Rendering users view");

    let header = build_header_context(&state, AppView::Users).await;
    match auth::load_users().await {
        Ok(users) => UsersTemplate {
            header,
            users,
            load_error: None,
        },
        Err(err) => {
            error_targeted!(HTTP, "Failed to load users: {}", err);
            UsersTemplate {
                header,
                users: HashMap::new(),
                load_error: Some(err),
            }
        }
    }
}

pub async fn add_modal() -> impl IntoResponse {
    UserModalTemplate { user: None }
}

pub async fn edit_modal(Query(query): Query<EditQuery>) -> impl IntoResponse {
    let users = auth::load_users().await.unwrap_or_default();
    let user = users.get(&query.username).cloned();
    UserModalTemplate { user }
}

pub async fn user_rows() -> impl IntoResponse {
    let users = auth::load_users().await.unwrap_or_default();
    let mut list: Vec<_> = users.into_values().collect();
    list.sort_by(|a, b| a.username.cmp(&b.username));
    UserRowsTemplate { users: list }
}

pub async fn add_user(Form(form): Form<UserActionForm>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Adding user: {:?}", form);
    let mut users = auth::load_users().await.unwrap_or_default();

    let username = form.username.trim().to_string();
    if users.contains_key(&username) {
        return ActionStatusTemplate {
            result: Err("Username already exists!".into())
        }.into_response();
    }

    let new_user = UserRecord {
        username: username.clone(),
        password: form.password.trim().to_string(),
        level: form.level,
    };

    users.insert(username, new_user.clone());

    // Properly handle the Result
    if let Err(err) = auth::save_users(&users).await {
        error_targeted!(HTTP, "Failed to save users during add: {}", err);
        return ActionStatusTemplate {
            result: Err(format!("Server error: Could not save user ({})", err))
        }.into_response();
    }

    let status_html = ActionStatusTemplate {
        result: Ok("User added successfully".into())
    }.render().unwrap_or_default();

    (
        [("HX-Trigger", "users-updated")],
        axum::response::Html(status_html),
    ).into_response()
}

pub async fn edit_user(Form(form): Form<UserActionForm>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Editing user: {:?}", form);
    let mut users = auth::load_users().await.unwrap_or_default();

    let username = form.username.trim().to_string();
    let original = form.original_username.unwrap_or_default();

    if username != original && users.contains_key(&username) {
        return ActionStatusTemplate {
            result: Err("Cannot rename: Username already taken!".into())
        }.into_response();
    }

    let new_user = UserRecord {
        username: username.clone(),
        password: form.password.trim().to_string(),
        level: form.level,
    };

    users.remove(&original);
    users.insert(username, new_user.clone());

    // Properly handle the Result
    if let Err(err) = auth::save_users(&users).await {
        error_targeted!(HTTP, "Failed to save users during edit: {}", err);
        return ActionStatusTemplate {
            result: Err(format!("Server error: Could not save edits ({})", err))
        }.into_response();
    }

    let status_html = ActionStatusTemplate {
        result: Ok("User updated successfully".into())
    }.render().unwrap_or_default();

    (
        [("HX-Trigger", "users-updated")],
        axum::response::Html(status_html),
    ).into_response()
}

pub async fn delete_user(Query(query): Query<DeleteQuery>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Deleting user: {:?}", query);
    let mut users = auth::load_users().await.unwrap_or_default();
    users.remove(&query.username);

    // Properly handle the Result
    if let Err(err) = auth::save_users(&users).await {
        error_targeted!(HTTP, "Failed to save users during delete: {}", err);
        return ActionStatusTemplate {
            result: Err(format!("Server error: Could not delete user ({})", err))
        }.into_response();
    }

    let status_html = ActionStatusTemplate {
        result: Ok(format!("User {} deleted", query.username))
    }.render().unwrap_or_default();

    (
        [("HX-Trigger", "users-updated")],
        axum::response::Html(status_html),
    ).into_response()
}
