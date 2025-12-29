use askama::Template;
use askama_web::WebTemplate;
use crate::views::{AppView, ViewTemplate};

#[derive(Template, WebTemplate)]
#[template(path = "views/connection-manager.html")]
pub struct ConnectionsTemplate {}

impl ViewTemplate for ConnectionsTemplate { const APP_VIEW_VARIANT: AppView = AppView::Connections; }


