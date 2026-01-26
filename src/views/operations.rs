
use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::views::{AppView, ViewTemplate};
use crate::{debug_targeted, AppState};



#[derive(Template, WebTemplate)]
#[template(path = "views/raw-reg-viewer.html")]
pub struct OperationsTemplate {}

impl ViewTemplate for OperationsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Operations; }

const OPERATIONS_TEMPLATE: OperationsTemplate = OperationsTemplate {};

async fn show_operations() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering operations view");
    OPERATIONS_TEMPLATE
}

pub fn routes() -> Router<AppState> {
    Router::new().route(AppView::Operations.url(), get(show_operations))
}
