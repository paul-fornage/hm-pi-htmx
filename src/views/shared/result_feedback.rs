use std::fmt::{Display};
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/result-feedback.html")]
pub struct FeedbackResult<T: Display, E: Display>{
    result: Result<T, E>,
}
impl<T: Display, E: Display> FeedbackResult<T, E> {
    pub fn new(result: Result<T, E>) -> Self {
        Self { result }
    }
    pub fn new_ok(value: T) -> Self {
        Self::new(Ok(value))
    }
    pub fn new_err(error: E) -> Self {
        Self::new(Err(error))
    }
}

