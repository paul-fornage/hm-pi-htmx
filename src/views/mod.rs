mod connections;
mod operations;
pub mod miller_info;
pub mod machine_config;
pub mod welder_profile;
mod clearcore_static_config;

pub use connections::ConnectionsTemplate;
pub use operations::OperationsTemplate;
pub use miller_info::MillerInfoTemplate;
pub use machine_config::MachineConfigTemplate;
pub use welder_profile::WelderProfileTemplate;

// Define the available views (tabs) in the application
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AppView {
    Operations,
    MillerInfo,
    Connections,
    MachineConfig,
    WelderProfile,
}

impl AppView {
    // Returns a slice of all views to iterate over in the template
    pub fn all() -> &'static [AppView] {
        &[
            AppView::Operations,
            AppView::WelderProfile,
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
        }
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