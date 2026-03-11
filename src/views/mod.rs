mod connections;
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
mod usb_transfer;
mod schedule_adjustments;

use crate::auth::AuthLevel;
use crate::AppState;
use axum::Router;
use strum::VariantArray;

// Define the available views (tabs) in the application
#[derive(PartialEq, Eq, Clone, Copy, strum::VariantArray)]
pub enum AppView {
    MillerInfo,
    Connections,
    MachineConfig,
    WelderProfile,
    MotionProfile,
    ClearcoreConfig,
    ClearcoreManualControl,
    ClearcoreLogs,
    ScheduleAdjustments,
    RunCycle,
    UsbTransfer,
    Users,
}

impl AppView {
    // Returns a slice of all views to iterate over in the template
    pub fn all() -> &'static [AppView] {
        Self::VARIANTS
    }

    // The text displayed on the tab
    pub fn label(&self) -> &'static str {
        match self {
            AppView::MillerInfo => "Miller Info",
            AppView::Connections => "Connections",
            AppView::MachineConfig => "Config",
            AppView::WelderProfile => "Welder Profile",
            AppView::MotionProfile => "Motion Profile",
            AppView::ClearcoreConfig => "ClearCore Config",
            AppView::ClearcoreManualControl => "Manual Control",
            AppView::ClearcoreLogs => "ClearCore Logs",
            AppView::ScheduleAdjustments => "Operator Adjustments",
            AppView::RunCycle => "Run Cycle",
            AppView::UsbTransfer => "USB transfer",
            AppView::Users => "Users",
        }
    }

    // The URL the tab links to
    pub const fn url(&self) -> &'static str {
        match self {
            AppView::MillerInfo => "/miller-info",
            AppView::Connections => "/connections",
            AppView::MachineConfig => "/machine-config",
            AppView::WelderProfile => "/welder-profile",
            AppView::MotionProfile => "/motion-profile",
            AppView::ClearcoreConfig => "/clearcore-config",
            AppView::ClearcoreManualControl => "/clearcore-manual-control",
            AppView::ClearcoreLogs => "/clearcore-logs",
            AppView::ScheduleAdjustments => "/schedule-adjustments",
            AppView::RunCycle => "/run-cycle",
            AppView::UsbTransfer => "/usb-transfer",
            AppView::Users => "/users",
        }
    }

    pub fn url_with_path(&self, path: &'static str) -> String {
        format!("{}{}", self.url(), path)
    }

    pub fn required_auth(&self) -> AuthLevel {
        match self {
            AppView::MillerInfo => AuthLevel::Operator,
            AppView::Connections => AuthLevel::Admin,
            AppView::MachineConfig => AuthLevel::Admin,
            AppView::WelderProfile => AuthLevel::Manager,
            AppView::MotionProfile => AuthLevel::Manager,
            AppView::ClearcoreConfig => AuthLevel::Admin,
            AppView::ClearcoreManualControl => AuthLevel::Operator,
            AppView::ClearcoreLogs => AuthLevel::Manager,
            AppView::ScheduleAdjustments => AuthLevel::Admin,
            AppView::RunCycle => AuthLevel::Operator,
            AppView::UsbTransfer => AuthLevel::Admin,
            AppView::Users => AuthLevel::Admin,
        }
    }

    pub fn from_url(url: &str) -> Option<AppView> {
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
    fn required_auth() -> AuthLevel { Self::APP_VIEW_VARIANT.required_auth() }
    const APP_VIEW_VARIANT: AppView;
    
    async fn header_context(state: &AppState) -> HeaderContext {
        build_header_context(state, Self::APP_VIEW_VARIANT).await
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(auth::routes())
        .merge(connections::routes())
        .merge(miller_info::routes())
        .merge(machine_config::routes())
        .merge(welder_profile::routes())
        .merge(motion_profile::routes())
        .merge(clearcore_static_config::routes())
        .merge(clearcore_manual_control::routes())
        .merge(run_cycle::routes())
        .merge(clearcore_logs::routes())
        .merge(usb_transfer::routes())
        .merge(schedule_adjustments::routes())
        .merge(users::routes())
}
