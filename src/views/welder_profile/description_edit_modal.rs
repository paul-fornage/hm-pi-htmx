use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "components/welder-profile/description-edit-modal.html")]
pub struct DescriptionEditModalTemplate {
    pub current_description: Option<String>,
}
