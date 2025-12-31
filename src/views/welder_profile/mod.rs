use askama::Template;
use askama_web::WebTemplate;
use axum::response::IntoResponse;
use crate::views::{AppView, ViewTemplate};
use crate::debug_targeted;

pub async fn show_welder_profile() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering welder profile view");
    WelderProfileTemplate {}
}

#[derive(Template, WebTemplate)]
#[template(path = "views/welder-profile.html")]
pub struct WelderProfileTemplate {}
impl ViewTemplate for WelderProfileTemplate { const APP_VIEW_VARIANT: AppView = AppView::WelderProfile; }
