use askama::Template;

#[derive(Template)]
#[template(path = "components/welder-profile/write-error-modal.html")]
pub struct WriteErrorModalTemplate {
    pub title: String,
    pub message: String,
}