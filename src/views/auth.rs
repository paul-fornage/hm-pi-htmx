use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;

use crate::auth::{self};
use crate::hx_trigger::HxTrigger;
use crate::views::{AppView, HeaderContext, build_header_context};
use crate::{debug_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "components/header.html")]
pub struct HeaderTemplate {
    pub header: HeaderContext,
}

#[derive(Template)]
#[template(path = "components/auth/sign-in-modal.html")]
pub struct SignInModalTemplate {
    pub error: Option<String>,
}

impl SignInModalTemplate {
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

#[derive(Deserialize)]
pub struct HeaderQuery {
    pub tab: Option<String>,
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignOutForm {
    pub current_url: Option<String>,
}

const AUTH_CHANGED_EVENT: HxTrigger = HxTrigger {
    event: "auth-changed",
    target: "#app-header",
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ui/header", get(header_handler))
        .route("/auth/sign-in", get(sign_in_modal))
        .route("/auth/sign-in", post(sign_in))
        .route("/auth/sign-out", post(sign_out))
}

pub async fn header_handler(
    State(state): State<AppState>,
    Query(query): Query<HeaderQuery>,
) -> impl IntoResponse {
    let active_tab = query
        .tab
        .as_deref()
        .and_then(AppView::from_url)
        .unwrap_or(AppView::Operations);

    let header = build_header_context(&state, active_tab).await;
    HeaderTemplate {
        header,
    }
}

pub async fn sign_in_modal() -> impl IntoResponse {
    let template = SignInModalTemplate { error: None };
    Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
}

pub async fn sign_in(
    State(state): State<AppState>,
    Form(form): Form<SignInForm>,
) -> Response {
    debug_targeted!(HTTP, "Sign-in attempt for user '{}'", form.username);

    let username = form.username.trim();
    let password = form.password.trim();

    if username.is_empty() || password.is_empty() {
        debug_targeted!(HTTP, "Sign-in failed: missing username or password");
        let template = SignInModalTemplate {
            error: Some("Username and password are required.".to_string()),
        };
        return Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
            .into_response();
    }

    match auth::verify_credentials(username, password).await {
        Ok(new_state) => {
            *state.auth_state.write().await = new_state;

            let mut headers = HeaderMap::new();
            let json_trigger = AUTH_CHANGED_EVENT.to_json();
            if let Ok(header_value) = HeaderValue::from_str(&json_trigger.to_string()) {
                headers.insert("HX-Trigger", header_value);
            }
            (headers, Html("".to_string())).into_response()
        }
        Err(msg) => {
            debug_targeted!(HTTP, "Sign-in failed: {msg}");
            let template = SignInModalTemplate {
                error: Some(msg),
            };
            Html(template.render().unwrap_or_else(|_| "Template error".to_string()))
                .into_response()
        }
    }
}

pub async fn sign_out(
    State(state): State<AppState>,
    Form(form): Form<SignOutForm>,
) -> Response {
    *state.auth_state.write().await = crate::auth::AuthState::Operator;

    let mut headers = HeaderMap::new();
    let should_redirect = form
        .current_url
        .as_deref()
        .and_then(AppView::from_url)
        .map(|view| view.required_auth() > auth::AuthLevel::Operator)
        .unwrap_or(false);

    if should_redirect {
        headers.insert("HX-Redirect", HeaderValue::from_static("/clearcore-manual-control"));
    } else {
        let json_trigger = AUTH_CHANGED_EVENT.to_json();
        if let Ok(header_value) = HeaderValue::from_str(&json_trigger.to_string()) {
            headers.insert("HX-Trigger", header_value);
        }
    }
    (headers, Html("".to_string())).into_response()
}
