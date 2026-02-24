
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::{debug_targeted, AppState};



#[derive(Template, WebTemplate)]
#[template(path = "views/raw-reg-viewer.html")]
pub struct OperationsTemplate {
    pub header: HeaderContext,
}

impl ViewTemplate for OperationsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Operations; }

async fn show_operations(State(state): State<AppState>) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering operations view");
    let header = build_header_context(&state, AppView::Operations).await;
    OperationsTemplate { header }
}

pub fn routes() -> Router<AppState> {
    Router::new().route(AppView::Operations.url(), get(show_operations))
}
