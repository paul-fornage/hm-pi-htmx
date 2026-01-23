mod modbus;
mod logging;

mod modbus_http;
pub mod error;
mod clearcore_registers;
mod miller;
mod connection_management;
mod views;
mod machine_config;
mod plc;

use std::sync::atomic::AtomicBool;
use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::error::HmPiError;
use crate::logging::LogTarget;
use crate::miller::miller_register_definitions::{MILLER_CHUNKS, MILLER_REGISTERS};
use crate::modbus::cached_modbus::{CachedModbus, ModbusChunk};
use crate::modbus::{ModbusManager, ModbusState};
use crate::plc::plc_register_definitions::CLEARCORE_CHUNKS;
use crate::views::{AppView, ClearcoreConfigTemplate, ConnectionsTemplate, MachineConfigTemplate, ManualControlTemplate, MillerInfoTemplate, OperationsTemplate, WelderProfileTemplate};
use crate::views::miller_info::register_view::BooleanRegisterTemplate;
use crate::views::miller_info::{register_details_modal, show_miller_info, show_miller_info_grid};
use crate::views::machine_config::{save_machine_config, show_machine_config};
use crate::views::welder_profile::{show_description_edit_modal, show_edit_modal, show_profile_metadata, show_welder_profile, show_welder_profile_grid, submit_register_write, update_description};
use crate::views::welder_profile::file_system_handlers::{handle_delete_profile_confirm, handle_get_profile_list, handle_load_apply, handle_load_modal, handle_load_preview, handle_save, handle_save_as_modal, handle_save_as_search, handle_save_as_submit};
use crate::views::clearcore_static_config::{self, handle_apply_config, handle_load_config, handle_save_config, show_clearcore_config, show_clearcore_config_grid};
use crate::views::clearcore_manual_control::{show_manual_control, get_x_position_handler, home_all_axes_handler};
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;

pub const MILLER_REG_READ_INTERVAL: std::time::Duration = std::time::Duration::from_millis(5);
pub const CLEARCORE_READ_INTERVAL: std::time::Duration = std::time::Duration::from_millis(5);

#[derive(Clone)]
pub struct AppState {
    pub clearcore_registers: CachedModbus,
    pub miller_registers: CachedModbus,
    pub machine_config: std::sync::Arc<tokio::sync::RwLock<machine_config::MachineConfig>>,
    pub weld_profile_metadata: std::sync::Arc<tokio::sync::Mutex<views::welder_profile::profile_metadata::WeldProfileMetadata>>,
    /// Not really an atomic sync flag or something, just cheaper than a mutex
    pub clearcore_configured: std::sync::Arc<AtomicBool>,
}

pub const OPERATIONS_TEMPLATE: OperationsTemplate = OperationsTemplate{};
pub const CONNECTIONS_TEMPLATE: ConnectionsTemplate = ConnectionsTemplate{};



async fn show_operations() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering operations view");
    OPERATIONS_TEMPLATE
}

async fn show_connections() -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering connections view");
    CONNECTIONS_TEMPLATE
}




#[tokio::main]
async fn main() {

    let mut builder = env_logger::Builder::from_default_env();
    if cfg!(debug_assertions) {
        builder.filter_level(log::LevelFilter::Debug);
        builder.filter(Some(LogTarget::MODBUS.into()), log::LevelFilter::Debug);
        builder.filter(Some(LogTarget::HTTP.into()), log::LevelFilter::Debug);
        builder.filter(Some("tokio_modbus::service::tcp"), log::LevelFilter::Info);
    } else {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    info_targeted!(HTTP, "Starting Modbus HTMX application");

    // Initialize Modbus Managers - load from saved config or use defaults
    let clearcore_config = modbus::ConnectionConfig::load_from_path(modbus::CLEARCORE_CONFIG_PATH)
        .unwrap_or_else(|| {
            let addr: std::net::SocketAddr = "192.168.1.68:502".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let clearcore_modbus = modbus::ModbusManager::new(clearcore_config.clone());

    let welder_config = modbus::ConnectionConfig::load_from_path(modbus::WELDER_CONFIG_PATH)
        .unwrap_or_else(|| {
            let addr: std::net::SocketAddr = "192.168.1.104:50205".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let welder_modbus = modbus::ModbusManager::new(welder_config.clone());


    let (clearcore_registers, mut clearcore_updater) =
        CachedModbus::new_with_updater(clearcore_modbus.clone(), CLEARCORE_CHUNKS);

    let clearcore_configured = std::sync::Arc::new(AtomicBool::new(false));

    let clearcore = clearcore_modbus.clone();
    let thread_copy_clearcore_registers = clearcore_registers.clone();
    let thread_copy_clearcore_configured = clearcore_configured.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(CLEARCORE_READ_INTERVAL);

        match clearcore.connect(clearcore_config).await {
            Ok(()) => {
                match clearcore_updater.initialize_all().await{
                    Ok(()) => {
                        trace_targeted!(MODBUS, "Initialized Clearcore registers");
                    },
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error initializing Clearcore registers: {:?}", e);
                    },
                }
                info_targeted!(MODBUS, "Clearcore auto-connected");
                let configured = ClearcoreConfig::on_boot(&thread_copy_clearcore_registers).await.unwrap_or_else(|e| {
                    warn_targeted!(MODBUS, "Error loading Clearcore config: {:?}", e);
                    false
                });
                thread_copy_clearcore_configured.store(configured, std::sync::atomic::Ordering::Release);
            },
            Err(e) => {
                info_targeted!(MODBUS, "Clearcore auto-connect failed: {:?}", e);
            }
        }
        loop {
            interval.tick().await;
            let current_state = clearcore_updater.get_connection_state().await
                .unwrap_or(ModbusState::Disconnected);
            if !(current_state == ModbusState::Connected) {
                clearcore_updater.clear_cache().await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                continue;
            }
            match clearcore_updater.update().await{
                Ok(()) => {
                    trace_targeted!(MODBUS, "Updated Clearcore registers");
                },
                Err(e) => {
                    let needs_disconnect = match &e{
                        HmPiError::ModbusTimedOut(msg) => {
                            warn_targeted!(MODBUS, "Clearcore timed out: {}", msg);
                            true
                        },
                        HmPiError::ModbusProtocolError(tokio_modbus::Error::Transport(e)) => {
                            warn_targeted!(MODBUS, "Clearcore protocol error: {}", e);
                            true
                        },
                        _ => false,
                    };
                    if needs_disconnect {
                        match clearcore.disconnect().await{
                            Ok(_was_connected) => { warn_targeted!(MODBUS, "Clearcore disconnected due to error."); },
                            Err(e) => {
                                warn_targeted!(MODBUS, "Error disconnecting Clearcore: {:?}", e);
                            }
                        }
                    } else {
                        warn_targeted!(MODBUS, "Error updating Clearcore registers: {:?}", e);
                    }
                },
            }
        }
    });




    let (miller_registers, mut miller_updater) =
        CachedModbus::new_with_updater(welder_modbus.clone(), MILLER_CHUNKS);

    let welder = welder_modbus.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(MILLER_REG_READ_INTERVAL);

        match welder.connect(welder_config).await {
            Ok(()) => {
                match miller_updater.initialize_all().await {
                    Ok(()) => {
                        trace_targeted!(MODBUS, "Initialized Welder registers");
                    }
                    Err(e) => {
                        warn_targeted!(MODBUS, "Error initializing Welder registers: {:?}", e);
                    }
                }
                info_targeted!(MODBUS, "Welder auto-connected");
            }
            Err(e) => {
                info_targeted!(MODBUS, "Welder auto-connect failed: {:?}", e);
            }
        }

        loop {
            interval.tick().await;
            let current_state = miller_updater
                .get_connection_state()
                .await
                .unwrap_or(ModbusState::Disconnected);
            if !(current_state == ModbusState::Connected) {
                miller_updater.clear_cache().await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                continue;
            }
            match miller_updater.update().await {
                Ok(()) => {
                    trace_targeted!(MODBUS, "Updated Welder registers");
                }
                Err(e) => {
                    let needs_disconnect = match &e {
                        HmPiError::ModbusTimedOut(msg) => {
                            warn_targeted!(MODBUS, "Welder timed out: {}", msg);
                            true
                        }
                        HmPiError::ModbusProtocolError(tokio_modbus::Error::Transport(e)) => {
                            warn_targeted!(MODBUS, "Welder protocol error: {}", e);
                            true
                        }
                        _ => false,
                    };
                    if needs_disconnect {
                        match welder.disconnect().await {
                            Ok(_was_connected) => {
                                warn_targeted!(MODBUS, "Welder disconnected due to error.");
                            }
                            Err(e) => {
                                warn_targeted!(MODBUS, "Error disconnecting Welder: {:?}", e);
                            }
                        }
                    } else {
                        warn_targeted!(MODBUS, "Error updating Welder registers: {:?}", e);
                    }
                }
            }
        }
    });


    // Initialize machine config
    let machine_config = machine_config::MachineConfig::load(machine_config::MACHINE_CONFIG_PATH)
        .unwrap_or_else(|_| machine_config::MachineConfig::default());
    let machine_config = std::sync::Arc::new(tokio::sync::RwLock::new(machine_config));


    // Initialize weld profile metadata
    let weld_profile_metadata = std::sync::Arc::new(tokio::sync::Mutex::new(
        views::welder_profile::profile_metadata::WeldProfileMetadata::new()
    ));

    // Initialize state with the managers
    let state = AppState {
        clearcore_registers,
        miller_registers,
        machine_config,
        weld_profile_metadata,
        clearcore_configured,
    };

    debug_targeted!(HTTP, "Initialized application state");

    let app = Router::new()
        // --- View Routes ---
        .route(AppView::Operations.url(), get(show_operations))
        .route(AppView::Connections.url(), get(show_connections))
        .route(AppView::MillerInfo.url(), get(show_miller_info))
        .route(AppView::MachineConfig.url(), get(show_machine_config))
        .route(AppView::WelderProfile.url(), get(show_welder_profile))
        .route(AppView::ClearcoreConfig.url(), get(show_clearcore_config))
        .route(AppView::ClearcoreManualControl.url(), get(show_manual_control))

        // --- Miller Info Component Routes ---
        .route("/miller-info/grid", get(show_miller_info_grid))

        // --- Welder Profile Component Routes ---
        .route("/welder-profile/grid", get(show_welder_profile_grid))
        .route("/welder-profile/metadata", get(show_profile_metadata))
        .route("/welder-profile/edit/{register_name}", get(show_edit_modal))
        .route("/welder-profile/write/{register_name}", post(submit_register_write))
        .route("/welder-profile/edit-description", get(show_description_edit_modal))
        .route("/welder-profile/update-description", post(update_description))

        // --- Welder Profile File System Routes ---
        .route("/welder-profile/fs/save", get(handle_save))
        .route("/welder-profile/fs/save_as", get(handle_save_as_modal))
        .route("/welder-profile/fs/save_as/search", post(handle_save_as_search))
        .route("/welder-profile/fs/load/list", get(handle_get_profile_list))
        .route("/welder-profile/fs/save_as/submit", post(handle_save_as_submit))
        .route("/welder-profile/fs/load", get(handle_load_modal))
        .route("/welder-profile/fs/load/preview", get(handle_load_preview))
        .route("/welder-profile/fs/load/apply", post(handle_load_apply))
        .route("/welder-profile/fs/load/delete_confirm", axum::routing::delete(handle_delete_profile_confirm))

        // --- ClearCore Config Component Routes ---
        .route("/clearcore-config/grid", get(show_clearcore_config_grid))
        .route("/clearcore-config/edit/{register_name}", get(clearcore_static_config::show_edit_modal))
        .route("/clearcore-config/write/{register_name}", post(clearcore_static_config::submit_register_write))
        .route("/clearcore-config/save", get(handle_save_config))
        .route("/clearcore-config/load", get(handle_load_config))
        .route("/clearcore-config/apply", get(handle_apply_config))

        // --- ClearCore Manual Control Component Routes ---
        .route("/clearcore-manual-control/home-axes", post(home_all_axes_handler))
        .route("/clearcore-manual-control/x-position", get(get_x_position_handler))

        // --- Machine Config Routes ---
        .route("/machine-config/save", post(save_machine_config))

        // --- UI Component Routes ---
        .route("/ui/modal/{register_name}", get(register_details_modal::modal_handler))

        // --- Modbus Management Routes - ClearCore ---
        .route("/modbus/clearcore/manager", get(connection_management::get_clearcore_manager))
        .route("/modbus/clearcore/connect", post(connection_management::connect_clearcore))
        .route("/modbus/clearcore/disconnect", post(connection_management::disconnect_clearcore))

        // --- Modbus Management Routes - Welder ---
        .route("/modbus/welder/manager", get(connection_management::get_welder_manager))
        .route("/modbus/welder/connect", post(connection_management::connect_welder))
        .route("/modbus/welder/disconnect", post(connection_management::disconnect_welder))

        .route("/modbus/manager", get(connection_management::get_connection_manager))
        .route("/modbus/status", get(connection_management::get_status))

        // --- Operation Routes ---
        .route("/read", post(modbus_http::read_registers))
        .route("/write", post(modbus_http::write_register))

        // --- Static files ---
        .fallback_service(ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("static/assets"))

        // Apply state
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    info_targeted!(HTTP, "Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
