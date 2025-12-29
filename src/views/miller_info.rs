
use askama::Template;
use askama_web::WebTemplate;
use crate::views::{AppView, ViewTemplate};



#[derive(Template, WebTemplate)]
#[template(path = "views/miller-info.html")]
pub struct MillerInfoTemplate {}

impl ViewTemplate for MillerInfoTemplate { const APP_VIEW_VARIANT: AppView = AppView::MillerInfo; }
