use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{debug_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "views/connection-manager.html")]
pub struct ConnectionsTemplate {
    pub header: HeaderContext,
}

impl ViewTemplate for ConnectionsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Connections; }

async fn show_connections(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering connections view");
    let header = build_header_context(&state, AppView::Connections).await;
    ConnectionsTemplate { header }
}

pub fn routes() -> Router<AppState> {
    Router::new().route(AppView::Connections.url(), get(show_connections))
}
