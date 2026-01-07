use askama::Template;
use askama_web::WebTemplate;
use super::weld_profile::ProfileListEntry;

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/save-status.html")]
pub struct SaveStatusTemplate {
    pub success: bool,
    pub message: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/save-as-modal.html")]
pub struct SaveAsModalTemplate {
    pub current_name: Option<String>,
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/save-as-profile-list.html")]
pub struct SaveAsProfileListTemplate {
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-modal.html")]
pub struct LoadModalTemplate {
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-profile-list.html")]
pub struct LoadProfileListTemplate {
    pub profiles: Vec<ProfileListEntry>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/load-preview.html")]
pub struct LoadPreviewTemplate {
    pub name: String,
    pub description: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/file-system/delete-button.html")]
pub struct DeleteButtonTemplate {
    pub profile_name: String,
    pub confirm_mode: bool,
}
