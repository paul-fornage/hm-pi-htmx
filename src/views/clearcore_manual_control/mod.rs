use askama::Template;
use askama_web::WebTemplate;
use crate::views::{AppView, ViewTemplate};





#[derive(Template, WebTemplate)]
#[template(path = "views/clearcore-config.html")]
pub struct ClearcoreConfigTemplate {}
impl ViewTemplate for ClearcoreConfigTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::ClearcoreConfig;
}