mod modbus;
mod logging;

mod modbus_http;
pub mod error;
mod clearcore_registers;
mod miller;
mod connection_management;
mod views;
mod auth;
mod machine_config;
mod plc;
mod udp_log_listener;
pub mod sse;
mod hx_trigger;
mod estop_component;
pub mod hmi_logic;

use tokio::sync::{broadcast};
use std::sync::atomic::{AtomicBool, Ordering};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::error::HmPiError;
use crate::logging::LogTarget;
use crate::miller::miller_register_definitions::{MILLER_CHUNKS};
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusState};
use crate::hmi_logic::mb_watcher::{cc_mb_watcher_task};
use crate::hmi_logic::watched_registers;
use crate::plc::plc_register_definitions::CLEARCORE_CHUNKS;
use crate::sse::connection_status::ConnectionStatus;
use crate::sse::error_toast::ErrorToast;
use crate::sse::SseEvent;
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;

pub const MILLER_REG_READ_INTERVAL: std::time::Duration = std::time::Duration::from_millis(10);
pub const CLEARCORE_READ_INTERVAL: std::time::Duration = std::time::Duration::from_millis(5);

#[derive(Clone)]
pub struct AppState {
    pub clearcore_registers: CachedModbus,
    pub miller_registers: CachedModbus,
    pub machine_config: std::sync::Arc<tokio::sync::RwLock<machine_config::MachineConfig>>,
    pub weld_profile_metadata: std::sync::Arc<tokio::sync::Mutex<views::welder_profile::profile_metadata::WeldProfileMetadata>>,
    pub motion_profile_metadata: std::sync::Arc<tokio::sync::Mutex<views::motion_profile::profile_metadata::MotionProfileMetadata>>,
    pub auth_state: std::sync::Arc<tokio::sync::RwLock<auth::AuthState>>,
    /// Not really an atomic sync flag or something, just cheaper than a mutex
    pub clearcore_configured: std::sync::Arc<AtomicBool>,
    pub clearcore_auto_connect: std::sync::Arc<AtomicBool>,
    pub welder_auto_connect: std::sync::Arc<AtomicBool>,
    pub sse_tx: broadcast::Sender<SseEvent>,
}

fn emit_connection_change(
    sse_tx: &broadcast::Sender<SseEvent>,
    connection_key: &'static str,
    connection_label: &'static str,
    last_state: &mut ModbusState,
    new_state: ModbusState,
) {
    if *last_state == new_state {
        return;
    }

    match sse_tx.send(ConnectionStatus::new(connection_key, new_state.clone()).into()) {
        Ok(_) => {}
        Err(e) => {
            warn_targeted!( MODBUS,
                "Failed to send connection status SSE event for {connection_key}: {e}");
        }
    }

    if matches!(*last_state, ModbusState::Connected) && !matches!(new_state, ModbusState::Connected) {
        let msg = format!(
            "{} connection lost ({})",
            connection_label,
            new_state.to_str()
        );
        if sse_tx.send(SseEvent::ErrorToast(ErrorToast { msg })).is_err() {
            warn_targeted!(
                MODBUS,
                "Failed to send connection error toast SSE event for {}",
                connection_key
            );
        }
    }

    *last_state = new_state;
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
        .await.unwrap_or_else(|| {
            warn_targeted!(FS, "Failed to read Clearcore modbus connection config. Using default Clearcore Modbus config");
            let addr: std::net::SocketAddr = "192.168.1.68:502".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let clearcore_modbus = modbus::ModbusManager::new(clearcore_config.clone());

    let welder_config = modbus::ConnectionConfig::load_from_path(modbus::WELDER_CONFIG_PATH)
        .await.unwrap_or_else(|| {
            warn_targeted!(FS, "Failed to read Miller modbus connection config. Using default Miller Modbus config");
            let addr: std::net::SocketAddr = "192.168.1.104:50205".parse().unwrap();
            modbus::ConnectionConfig::new(addr, 1)
        });
    let welder_modbus = modbus::ModbusManager::new(welder_config.clone());


    let (clearcore_registers, mut clearcore_updater) =
        CachedModbus::new_with_updater(clearcore_modbus.clone(), CLEARCORE_CHUNKS);

    let clearcore_configured = std::sync::Arc::new(AtomicBool::new(false));
    let clearcore_auto_connect = std::sync::Arc::new(AtomicBool::new(true));
    let welder_auto_connect = std::sync::Arc::new(AtomicBool::new(true));

    let sse_tx: broadcast::Sender<SseEvent> = broadcast::Sender::new(32);

    let clearcore = clearcore_modbus.clone();
    let thread_copy_clearcore_registers = clearcore_registers.clone();
    let thread_copy_clearcore_configured = clearcore_configured.clone();
    let thread_copy_clearcore_auto_connect = clearcore_auto_connect.clone();
    let thread_copy_clearcore_sse = sse_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(CLEARCORE_READ_INTERVAL);
        let mut last_state = ModbusState::Disconnected;


        loop {
            interval.tick().await;
            let current_state = clearcore_updater.get_connection_state().await
                .unwrap_or(ModbusState::Disconnected);
            emit_connection_change(
                &thread_copy_clearcore_sse,
                "clearcore",
                "ClearCore",
                &mut last_state,
                current_state.clone(),
            );
            if !(current_state == ModbusState::Connected) {
                clearcore_updater.clear_cache().await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                if !thread_copy_clearcore_auto_connect.load(Ordering::Acquire) {
                    debug_targeted!(MODBUS, "Clearcore auto-connect disabled; skipping connect attempt");
                    continue;
                }

                emit_connection_change(
                    &thread_copy_clearcore_sse,
                    "clearcore",
                    "ClearCore",
                    &mut last_state,
                    ModbusState::Connecting,
                );

                let clearcore_config = modbus::ConnectionConfig::load_from_path(modbus::CLEARCORE_CONFIG_PATH)
                    .await.unwrap_or_else(|| {
                        warn_targeted!(FS, "Failed to read Clearcore modbus connection config. Using default Clearcore Modbus config");
                        let addr: std::net::SocketAddr = "192.168.1.68:502".parse().unwrap();
                        modbus::ConnectionConfig::new(addr, 1)
                    });
                
                match clearcore.connect(clearcore_config).await {
                    Ok(()) => {
                        match clearcore_updater.initialize_all().await{
                            Ok(()) => {
                                trace_targeted!(MODBUS, "Initialized Clearcore registers");
                            },
                            Err(e) => {
                                warn_targeted!(MODBUS, "Error initializing Clearcore registers: {:?}", e);
                                continue;
                            },
                        }
                        info_targeted!(MODBUS, "Clearcore auto-connected");
                        let configured = ClearcoreConfig::on_boot(&thread_copy_clearcore_registers).await.unwrap_or_else(|e| {
                            warn_targeted!(MODBUS, "Error loading Clearcore config: {:?}", e);
                            false
                        });
                        thread_copy_clearcore_configured.store(configured, Ordering::Release);
                    },
                    Err(e) => {
                        info_targeted!(MODBUS, "Clearcore auto-connect failed: {:?}", e);
                    }
                }

                let post_connect_state = clearcore_updater
                    .get_connection_state()
                    .await
                    .unwrap_or(ModbusState::Disconnected);
                emit_connection_change(
                    &thread_copy_clearcore_sse,
                    "clearcore",
                    "ClearCore",
                    &mut last_state,
                    post_connect_state,
                );

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
                        let post_disconnect_state = clearcore_updater
                            .get_connection_state()
                            .await
                            .unwrap_or(ModbusState::Disconnected);
                        emit_connection_change(
                            &thread_copy_clearcore_sse,
                            "clearcore",
                            "ClearCore",
                            &mut last_state,
                            post_disconnect_state,
                        );
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
    let thread_copy_welder_auto_connect = welder_auto_connect.clone();
    let thread_copy_welder_sse = sse_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(MILLER_REG_READ_INTERVAL);
        let mut last_state = ModbusState::Disconnected;
        
        loop {
            interval.tick().await;
            let current_state = miller_updater
                .get_connection_state()
                .await
                .unwrap_or(ModbusState::Disconnected);
            emit_connection_change(
                &thread_copy_welder_sse,
                "welder",
                "Welder",
                &mut last_state,
                current_state.clone(),
            );
            if !(current_state == ModbusState::Connected) {
                miller_updater.clear_cache().await;
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                if !thread_copy_welder_auto_connect.load(Ordering::Acquire) {
                    debug_targeted!(MODBUS, "Welder auto-connect disabled; skipping connect attempt");
                    continue;
                }

                emit_connection_change(
                    &thread_copy_welder_sse,
                    "welder",
                    "Welder",
                    &mut last_state,
                    ModbusState::Connecting,
                );
                
                let welder_config = modbus::ConnectionConfig::load_from_path(modbus::WELDER_CONFIG_PATH)
                    .await.unwrap_or_else(|| {
                        warn_targeted!(FS, "Failed to read Miller modbus connection config. Using default Miller Modbus config");
                        let addr: std::net::SocketAddr = "192.168.1.104:50205".parse().unwrap();
                        modbus::ConnectionConfig::new(addr, 1)
                    });
                
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
                let post_connect_state = miller_updater
                    .get_connection_state()
                    .await
                    .unwrap_or(ModbusState::Disconnected);
                emit_connection_change(
                    &thread_copy_welder_sse,
                    "welder",
                    "Welder",
                    &mut last_state,
                    post_connect_state,
                );
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
                        let post_disconnect_state = miller_updater
                            .get_connection_state()
                            .await
                            .unwrap_or(ModbusState::Disconnected);
                        emit_connection_change(
                            &thread_copy_welder_sse,
                            "welder",
                            "Welder",
                            &mut last_state,
                            post_disconnect_state,
                        );
                    } else {
                        warn_targeted!(MODBUS, "Error updating Welder registers: {:?}", e);
                    }
                }
            }
        }
    });


    // Initialize machine config
    let machine_config = machine_config::MachineConfig::load(machine_config::MACHINE_CONFIG_PATH)
        .unwrap_or_else(|_| {
            warn_targeted!(FS, "Failed to load machine config, using default values");
            machine_config::MachineConfig::default()
        });
    let upd_log_port = machine_config.udp_logging_port;
    let machine_config = std::sync::Arc::new(tokio::sync::RwLock::new(machine_config));

    let sse_tx_clone = sse_tx.clone();
    tokio::spawn(async move{ 
        loop { 
            udp_log_listener::start_listener(upd_log_port, sse_tx_clone.clone()).await; 
        }
    });

    // Initialize weld profile metadata
    let weld_profile_metadata = std::sync::Arc::new(tokio::sync::Mutex::new(
        views::welder_profile::profile_metadata::WeldProfileMetadata::new()
    ));
    let motion_profile_metadata = std::sync::Arc::new(tokio::sync::Mutex::new(
        views::motion_profile::profile_metadata::MotionProfileMetadata::new()
    ));

    let auth_state = std::sync::Arc::new(tokio::sync::RwLock::new(auth::AuthState::default()));

    let watcher_cc_regs = clearcore_registers.clone();
    let watcher_welder_regs = miller_registers.clone();

    let miller_regs_clone = miller_registers.clone();

    let touch_retract_watcher = watched_registers::touch_retract_passthrough(miller_regs_clone);
    let estop_watcher = watched_registers::estop_emitter(sse_tx.clone());
    let cc_watcher_tasks = vec![touch_retract_watcher, estop_watcher];

    tokio::spawn(async move {
        cc_mb_watcher_task(&watcher_cc_regs, cc_watcher_tasks).await;
    });

    // Initialize state with the managers
    let state = AppState {
        clearcore_registers,
        miller_registers,
        machine_config,
        weld_profile_metadata,
        motion_profile_metadata,
        auth_state,
        clearcore_configured,
        clearcore_auto_connect,
        welder_auto_connect,
        sse_tx,
    };

    debug_targeted!(HTTP, "Initialized application state");

    let app = Router::new()
        // --- View Routes ---
        .merge(views::routes())
        .merge(estop_component::routes())

        .route("/sse", get(sse::sse_handler))

        // --- Modbus Management Routes - ClearCore ---
        .route("/modbus/clearcore/manager", get(connection_management::get_clearcore_manager))
        .route("/modbus/clearcore/connect", post(connection_management::connect_clearcore))
        .route("/modbus/clearcore/disconnect", post(connection_management::disconnect_clearcore))
        .route("/modbus/clearcore/auto-connect", post(connection_management::set_clearcore_auto_connect))

        // --- Modbus Management Routes - Welder ---
        .route("/modbus/welder/manager", get(connection_management::get_welder_manager))
        .route("/modbus/welder/connect", post(connection_management::connect_welder))
        .route("/modbus/welder/disconnect", post(connection_management::disconnect_welder))
        .route("/modbus/welder/auto-connect", post(connection_management::set_welder_auto_connect))

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
