use std::fmt::{Display};
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/result-feedback.html")]
struct FeedbackResult<E: Display>{
    result: Result<String, E>,
}

