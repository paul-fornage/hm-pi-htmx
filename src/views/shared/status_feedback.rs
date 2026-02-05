use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/status-feedback.html")]
pub struct StatusFeedbackTemplate {
    pub mandrel_latch_closed: Option<bool>,
}
