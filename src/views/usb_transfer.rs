mod handlers;
mod templates;
mod transfer;
mod types;

pub use templates::UsbTransferTemplate;
use axum::Router;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    handlers::routes()
}
