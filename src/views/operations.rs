
use askama::Template;
use askama_web::WebTemplate;
use crate::views::{AppView, ViewTemplate};



#[derive(Template, WebTemplate)]
#[template(path = "views/raw-reg-viewer.html")]
pub struct OperationsTemplate {}

impl ViewTemplate for OperationsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Operations; }
