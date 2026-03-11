mod handlers;
mod templates;
mod transfer;
mod types;

use crate::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    handlers::routes()
}
