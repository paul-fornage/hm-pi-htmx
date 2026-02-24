mod connections;
mod operations;
pub mod miller_info;
pub mod machine_config;
pub mod welder_profile;
pub mod motion_profile;
pub mod clearcore_static_config;
pub mod shared;
pub mod clearcore_manual_control;
pub mod run_cycle;
mod clearcore_logs;
mod auth;
mod users;

use axum::Router;
use crate::AppState;
use crate::auth::AuthLevel;

pub use connections::ConnectionsTemplate;
pub use operations::OperationsTemplate;
pub use miller_info::MillerInfoTemplate;
pub use machine_config::MachineConfigTemplate;
pub use welder_profile::WelderProfileTemplate;
pub use motion_profile::MotionProfileTemplate;
pub use clearcore_static_config::ClearcoreConfigTemplate;
pub use clearcore_manual_control::ManualControlTemplate;
pub use run_cycle::RunCycleTemplate;
pub use clearcore_logs::ClearcoreLogsTemplate;
pub use users::UsersTemplate;

// Define the available views (tabs) in the application
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AppView {
    Operations,
    MillerInfo,
    Connections,
    MachineConfig,
    WelderProfile,
    MotionProfile,
    ClearcoreConfig,
    ClearcoreManualControl,
    ClearcoreLogs,
    RunCycle,
    Users,
}

impl AppView {
    // Returns a slice of all views to iterate over in the template
    pub fn all() -> &'static [AppView] {
        &[
            AppView::RunCycle,
            AppView::ClearcoreManualControl,
            AppView::ClearcoreLogs,
            AppView::WelderProfile,
            AppView::MotionProfile,
            AppView::ClearcoreConfig,
            AppView::MillerInfo,
            AppView::Connections,
            AppView::MachineConfig,
            AppView::Users,
        ]
    }

    // The text displayed on the tab
    pub fn label(&self) -> &'static str {
        match self {
            AppView::Operations => "Operations",
            AppView::MillerInfo => "Miller Info",
            AppView::Connections => "Connections",
            AppView::MachineConfig => "Config",
            AppView::WelderProfile => "Welder Profile",
            AppView::MotionProfile => "Motion Profile",
            AppView::ClearcoreConfig => "ClearCore Config",
            AppView::ClearcoreManualControl => "Manual Control",
            AppView::ClearcoreLogs => "ClearCore Logs",
            AppView::RunCycle => "Run Cycle",
            AppView::Users => "Users",
        }
    }

    // The URL the tab links to
    pub const fn url(&self) -> &'static str {
        match self {
            AppView::Operations => "/",
            AppView::MillerInfo => "/miller-info",
            AppView::Connections => "/connections",
            AppView::MachineConfig => "/machine-config",
            AppView::WelderProfile => "/welder-profile",
            AppView::MotionProfile => "/motion-profile",
            AppView::ClearcoreConfig => "/clearcore-config",
            AppView::ClearcoreManualControl => "/clearcore-manual-control",
            AppView::ClearcoreLogs => "/clearcore-logs",
            AppView::RunCycle => "/run-cycle",
            AppView::Users => "/users",
        }
    }

    pub fn url_with_path(&self, path: &'static str) -> String {
        format!("{}{}", self.url(), path)
    }

    pub fn required_auth(&self) -> AuthLevel {
        match self {
            AppView::Operations => AuthLevel::Operator,
            AppView::RunCycle => AuthLevel::Operator,
            AppView::ClearcoreManualControl => AuthLevel::Operator,
            AppView::ClearcoreLogs => AuthLevel::Manager,
            AppView::WelderProfile => AuthLevel::Manager,
            AppView::MotionProfile => AuthLevel::Manager,
            AppView::ClearcoreConfig => AuthLevel::Manager,
            AppView::MillerInfo => AuthLevel::Manager,
            AppView::Connections => AuthLevel::Admin,
            AppView::MachineConfig => AuthLevel::Admin,
            // TODO: Confirm Users page auth level (defaulting to Admin).
            AppView::Users => AuthLevel::Admin,
        }
    }

    pub fn from_url(url: &str) -> Option<AppView> {
        if url == AppView::Operations.url() {
            return Some(AppView::Operations);
        }
        AppView::all().iter().copied().find(|v| v.url() == url)
    }
}

// This struct is included in every page template to render the header
pub struct HeaderContext {
    pub tabs: Vec<AppView>,
    pub active_tab: AppView,
    pub auth_level: AuthLevel,
    pub auth_username: Option<String>,
}

impl HeaderContext {
    pub fn new(active_tab: AppView, auth_level: AuthLevel, auth_username: Option<String>) -> Self {
        let tabs = AppView::all()
            .iter()
            .copied()
            .filter(|tab| tab.required_auth() <= auth_level)
            .collect();
        Self {
            tabs,
            active_tab,
            auth_level,
            auth_username,
        }
    }

    pub fn is_signed_in(&self) -> bool {
        self.auth_username.is_some()
    }
}

pub async fn build_header_context(state: &AppState, active_tab: AppView) -> HeaderContext {
    let auth_state = state.auth_state.read().await;
    let auth_level = auth_state.level();
    let auth_username = auth_state.username().map(|s| s.to_string());
    HeaderContext::new(active_tab, auth_level, auth_username)
}

pub trait ViewTemplate{
    fn get_view() -> &'static AppView { &Self::APP_VIEW_VARIANT }
    fn all_views() -> &'static [AppView] { AppView::all() }
    fn required_auth() -> AuthLevel { Self::APP_VIEW_VARIANT.required_auth() }
    const APP_VIEW_VARIANT: AppView;
    
    async fn header_context(state: &AppState) -> HeaderContext {
        build_header_context(state, Self::APP_VIEW_VARIANT).await
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(operations::routes())
        .merge(connections::routes())
        .merge(miller_info::routes())
        .merge(machine_config::routes())
        .merge(welder_profile::routes())
        .merge(motion_profile::routes())
        .merge(clearcore_static_config::routes())
        .merge(clearcore_manual_control::routes())
        .merge(run_cycle::routes())
        .merge(clearcore_logs::routes())
        .merge(users::routes())
}
