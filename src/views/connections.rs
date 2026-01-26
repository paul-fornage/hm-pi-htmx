use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::views::{AppView, ViewTemplate};
use crate::{debug_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "views/connection-manager.html")]
pub struct ConnectionsTemplate {}

impl ViewTemplate for ConnectionsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Connections; }

const CONNECTIONS_TEMPLATE: ConnectionsTemplate = ConnectionsTemplate {};

async fn show_connections() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering connections view");
    CONNECTIONS_TEMPLATE
}

pub fn routes() -> Router<AppState> {
    Router::new().route(AppView::Connections.url(), get(show_connections))
}

