use askama::Template;
use askama_web::WebTemplate;
use axum::response::sse::Event;
use crate::sse::SseEventExt;

#[derive(Clone, Debug, Template)]
#[template(path = "components/error-toast.html")]
pub struct ErrorToast {
    pub msg: String,
}

impl SseEventExt for ErrorToast {
    fn as_axum_event(&self) -> Event {
        Event::default()
            .event("error-toast")
            .data(self.render().unwrap_or("FAILED TO RENDER".to_string()))
    }
}