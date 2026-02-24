use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::udp_log_listener::{LOG_BUFFER, MAX_LOG_ENTRIES};
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{debug_targeted, AppState};

#[derive(Template, WebTemplate)]
#[template(path = "views/clearcore-logs.html")]
pub struct ClearcoreLogsTemplate {
    pub header: HeaderContext,
    pub entries: Vec<String>,
}

impl ViewTemplate for ClearcoreLogsTemplate { const APP_VIEW_VARIANT: AppView = AppView::ClearcoreLogs; }

async fn show_clearcore_logs(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering clearcore logs view");
    let entries = {
        let buffer = LOG_BUFFER.lock().await;
        let start = buffer.len().saturating_sub(MAX_LOG_ENTRIES);
        buffer
            .iter()
            .skip(start)
            .map(|log| log.as_html_list_element())
            .collect()
    };
    let header = build_header_context(&state, AppView::ClearcoreLogs).await;
    ClearcoreLogsTemplate { header, entries }
}

pub fn routes() -> Router<AppState> {
    Router::new().route(AppView::ClearcoreLogs.url(), get(show_clearcore_logs))
}
