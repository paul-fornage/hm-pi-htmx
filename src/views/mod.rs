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

use axum::Router;
use crate::AppState;

pub use connections::ConnectionsTemplate;
pub use operations::OperationsTemplate;
pub use miller_info::MillerInfoTemplate;
pub use machine_config::MachineConfigTemplate;
pub use welder_profile::WelderProfileTemplate;
pub use motion_profile::MotionProfileTemplate;
pub use clearcore_static_config::ClearcoreConfigTemplate;
pub use clearcore_manual_control::ManualControlTemplate;
pub use run_cycle::RunCycleTemplate;

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
    RunCycle,
}

impl AppView {
    // Returns a slice of all views to iterate over in the template
    pub fn all() -> &'static [AppView] {
        &[
            AppView::RunCycle,
            AppView::ClearcoreManualControl,
            AppView::WelderProfile,
            AppView::MotionProfile,
            AppView::ClearcoreConfig,
            AppView::MillerInfo,
            AppView::Connections,
            AppView::MachineConfig,
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
            AppView::RunCycle => "Run Cycle",
        }
    }

    // The URL the tab links to
    pub fn url(&self) -> &'static str {
        match self {
            AppView::Operations => "/",
            AppView::MillerInfo => "/miller-info",
            AppView::Connections => "/connections",
            AppView::MachineConfig => "/machine-config",
            AppView::WelderProfile => "/welder-profile",
            AppView::MotionProfile => "/motion-profile",
            AppView::ClearcoreConfig => "/clearcore-config",
            AppView::ClearcoreManualControl => "/clearcore-manual-control",
            AppView::RunCycle => "/run-cycle",
        }
    }

    pub fn url_with_path(&self, path: &'static str) -> String {
        format!("{}{}", self.url(), path)
    }
}

// This struct is included in every page template to render the header
pub struct HeaderContext {
    pub tabs: &'static [AppView],
    pub active_tab: AppView,
}

impl HeaderContext {
    pub fn new(active_tab: AppView) -> Self {
        Self {
            tabs: AppView::all(),
            active_tab,
        }
    }
}


pub trait ViewTemplate{
    fn get_view() -> &'static AppView { &Self::APP_VIEW_VARIANT }
    fn all_views() -> &'static [AppView] { AppView::all() }
    const APP_VIEW_VARIANT: AppView;
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(operations::routes())
        .merge(connections::routes())
        .merge(miller_info::routes())
        .merge(machine_config::routes())
        .merge(welder_profile::routes())
        .merge(motion_profile::routes())
        .merge(clearcore_static_config::routes())
        .merge(clearcore_manual_control::routes())
        .merge(run_cycle::routes())
}
