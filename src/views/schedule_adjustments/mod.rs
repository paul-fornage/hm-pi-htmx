mod allowed_adjustments;
mod adjustable_registers;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Form, Router};
use axum::routing::{get, post};
use crate::{debug_targeted, error_targeted, info_targeted, warn_targeted, AppState};
use crate::file_io::{FileIoError, FixedDiskFile};
use crate::sse::error_toast::ErrorToast;
use crate::views::{build_header_context, AppView, ConnectionsTemplate, HeaderContext, ViewTemplate};
use crate::views::schedule_adjustments::allowed_adjustments::AllowedAdjustments;
use crate::views::shared::result_feedback::FeedbackResult;

#[derive(Template, WebTemplate)]
#[template(path = "views/schedule-adjustments.html")]
pub struct ScheduleAdjustmentsTemplate {
    pub header: HeaderContext,
}

impl ViewTemplate for ScheduleAdjustmentsTemplate { const APP_VIEW_VARIANT: AppView = AppView::ScheduleAdjustments; }

async fn main_page(State(state): State<AppState>) -> impl IntoResponse {
    let header = build_header_context(&state, AppView::ScheduleAdjustments).await;
    ScheduleAdjustmentsTemplate { header }
}

// /allowed-adjustments-data
async fn post_form(Form(form): Form<AllowedAdjustments>) -> impl IntoResponse {
    debug_targeted!(HTTP, "received new adjustment form");
    if !form.verify_schema() {
        return FeedbackResult::new_err("Internal error: Adjustments form does not match schema".to_string());
    }
    match form.save().await {
        Ok(_) => FeedbackResult::new_ok("Saved"),
        Err(e) => FeedbackResult::new_err(format!("Failed to save! {}", e)),
    }
}

async fn get_form(State(state): State<AppState>) -> impl IntoResponse {
    match AllowedAdjustments::load().await{
        Ok(mut form) => {
            debug_targeted!(HTTP, "loaded adjustment form");
            form.conform_to_schema();
            form
        },
        Err(e) => {
            if let Err(e) = state.sse_tx.send(ErrorToast{
                msg: format!("Failed to load adjustment form: {}", e)
            }.into()){
                error_targeted!(HTTP, "Failed to send error toast: {}", e);
            }

            let default_form = AllowedAdjustments::default();

            if let FileIoError::NotFound { path } = e {
                info_targeted!(FS, "Adjustment form not found at '{}', creating default", path.display());
                if let Err(e) = default_form.save().await{
                    warn_targeted!(FS, "Failed to create default adjustment form: {}", e);
                }
            }

            default_form
        },
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(&AppView::ScheduleAdjustments.url(), get(main_page))
        .route(&AppView::ScheduleAdjustments.url_with_path("/allowed-adjustments-data"), post(post_form))
        .route(&AppView::ScheduleAdjustments.url_with_path("/allowed-adjustments-data"), get(get_form))

}
