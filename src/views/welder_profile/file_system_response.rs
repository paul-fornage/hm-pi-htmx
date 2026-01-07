use axum::response::{IntoResponse, Response};
use axum::http::{StatusCode, header};

pub enum FileSystemResponse {
    Success,
    SuccessStatus(String),
    Error(String),
    Template(String),
}

impl IntoResponse for FileSystemResponse {
    fn into_response(self) -> Response {
        match self {
            FileSystemResponse::Success => {
                (StatusCode::OK, "").into_response()
            }
            FileSystemResponse::SuccessStatus(msg) => {
                (
                    StatusCode::OK,
                    [(header::HeaderName::from_static("hx-trigger"), "profileUpdated")],
                    axum::response::Html(format!(
                        "<div class='text-green-600 text-sm'>{}</div>",
                        msg
                    ))
                ).into_response()
            }
            FileSystemResponse::Error(msg) => {
                (
                    StatusCode::OK,
                    axum::response::Html(format!(
                        "<div class='text-red-600 text-sm'>{}</div>",
                        msg
                    ))
                ).into_response()
            }
            FileSystemResponse::Template(html) => {
                (StatusCode::OK, axum::response::Html(html)).into_response()
            }
        }
    }
}

pub fn render_template<T: askama::Template>(template: T) -> FileSystemResponse {
    match template.render() {
        Ok(html) => FileSystemResponse::Template(html),
        Err(e) => {
            log::error!("Template rendering failed: {}", e);
            FileSystemResponse::Error("Internal error rendering template".to_string())
        }
    }
}
