use crate::file_io::FileIoError;
use crate::modbus::cached_modbus::CachedModbus;
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;
use crate::views::shared::result_feedback::FeedbackResult;
use crate::views::shared::{mb_read_bool_helper, StatusFeedbackTemplate};
use crate::views::{build_header_context, AppView, HeaderContext, ViewTemplate};
use crate::{warn_targeted, AppState};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;

use super::helpers::{read_bool, read_u16};

#[derive(Template, WebTemplate)]
#[template(path = "views/manual-control.html")]
pub(super) struct ManualControlTemplate {
    pub header: HeaderContext,
    pub show_y_axis: bool,
    pub show_z_axis: bool,
    pub show_w_axis: bool,
}

impl ViewTemplate for ManualControlTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::ClearcoreManualControl;
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/homing-status.html")]
pub(super) struct HomingStatusTemplate {
    is_homing: bool,
    is_homed: bool,
    error: Option<String>,
}

pub(super) async fn show_manual_control(State(state): State<AppState>) -> impl IntoResponse {
    let header = build_header_context(&state, AppView::ClearcoreManualControl).await;
    let (show_y_axis, show_z_axis, show_w_axis) = match ClearcoreConfig::load_config().await {
        Ok(config) => {
            let show_y = config
                .coils
                .get(cc_regs::USES_Y_AXIS.name)
                .copied()
                .unwrap_or(true);
            let show_z = config
                .coils
                .get(cc_regs::USES_Z_AXIS.name)
                .copied()
                .unwrap_or(true);
            let show_w = config
                .coils
                .get(cc_regs::USES_W_AXIS.name)
                .copied()
                .unwrap_or(true);
            (show_y, show_z, show_w)
        }
        Err(FileIoError::NotFound { .. }) => {
            warn_targeted!(
                FS,
                "Clearcore static config not found; showing all axes in manual control"
            );
            (true, true, true)
        }
        Err(err) => {
            warn_targeted!(
                FS,
                "Failed to load clearcore static config for manual control: {err}"
            );
            (true, true, true)
        }
    };

    ManualControlTemplate {
        header,
        show_y_axis,
        show_z_axis,
        show_w_axis,
    }
}

pub(super) async fn homing_status_handler(State(state): State<AppState>) -> impl IntoResponse {
    match get_homing_status(&state.clearcore_registers).await {
        Ok((is_homing, is_homed)) => HomingStatusTemplate {
            is_homing,
            is_homed,
            error: None,
        },
        Err(error) => HomingStatusTemplate {
            is_homing: false,
            is_homed: false,
            error: Some(error),
        },
    }
}

pub(super) async fn manual_control_status_feedback(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mandrel_latch_closed = mb_read_bool_helper(
        &state.clearcore_registers,
        &cc_regs::MANDREL_LATCH_CLOSED.address,
    )
    .await;

    StatusFeedbackTemplate {
        mandrel_latch_closed,
    }
}

pub(super) async fn home_all_axes_handler(State(state): State<AppState>) -> impl IntoResponse {
    let action_result = home_all_axes(&state.clearcore_registers).await;
    let status_result = get_homing_status(&state.clearcore_registers).await;
    if let Err(err) = &status_result {
        warn_targeted!(MODBUS, "Failed to read homing status after home command: {err}");
    }

    match (action_result, status_result) {
        (Ok(()), Ok((_is_homing, is_homed))) => HomingStatusTemplate {
            is_homing: true,
            is_homed,
            error: None,
        },
        (Ok(()), Err(_)) => HomingStatusTemplate {
            is_homing: true,
            is_homed: false,
            error: None,
        },
        (Err(action_error), Ok((is_homing, is_homed))) => HomingStatusTemplate {
            is_homing,
            is_homed,
            error: Some(action_error),
        },
        (Err(action_error), Err(_)) => HomingStatusTemplate {
            is_homing: false,
            is_homed: false,
            error: Some(action_error),
        },
    }
}

pub(super) async fn get_x_position_handler(
    State(state): State<AppState>,
) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::X)
        .await
        .into()
}

pub(super) async fn get_y_position_handler(
    State(state): State<AppState>,
) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::Y)
        .await
        .into()
}

pub(super) async fn get_z_position_handler(
    State(state): State<AppState>,
) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::Z)
        .await
        .into()
}

async fn home_all_axes(clearcore_registers: &CachedModbus) -> Result<(), String> {
    let is_homing = clearcore_registers
        .read_coil(cc_regs::IS_HOMING.address.address)
        .await;
    match is_homing {
        Some(false) => {}
        Some(true) => return Err("Homing in progress".to_string()),
        None => return Err("Error reading homing status".to_string()),
    }

    let homing_already_commanded = clearcore_registers
        .read_coil(cc_regs::HOME_LATCH.address.address)
        .await;
    match homing_already_commanded {
        Some(false) => {}
        Some(true) => return Err("Homing already commanded and clearcore isn't homing".to_string()),
        None => return Err("Error reading homing status".to_string()),
    }

    clearcore_registers
        .write_coil(cc_regs::HOME_LATCH.address.address, true)
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

async fn get_homing_status(registers: &CachedModbus) -> Result<(bool, bool), String> {
    let is_homing = registers
        .read_coil(cc_regs::IS_HOMING.address.address)
        .await
        .ok_or_else(|| "Error reading homing status".to_string())?;
    let is_homed = registers
        .read_coil(cc_regs::IS_HOMED.address.address)
        .await
        .ok_or_else(|| "Error reading homed status".to_string())?;
    Ok((is_homing, is_homed))
}

#[derive(Clone, Copy)]
enum AxisPosition {
    X,
    Y,
    Z,
}

async fn get_axis_position(
    registers: &CachedModbus,
    axis: AxisPosition,
) -> Result<String, String> {
    let (is_homed_register, axis_label) = match axis {
        AxisPosition::X => (&cc_regs::X_AXIS_IS_HOMED.address, "X axis"),
        AxisPosition::Y => (&cc_regs::Y_AXIS_IS_HOMED.address, "Y axis"),
        AxisPosition::Z => (&cc_regs::Z_AXIS_IS_HOMED.address, "Z axis"),
    };
    let is_homed = read_bool(registers, is_homed_register).await?;
    if is_homed {
        let register = match axis {
            AxisPosition::X => &cc_regs::X_AXIS_POSITION.address,
            AxisPosition::Y => &cc_regs::Y_AXIS_POSITION.address,
            AxisPosition::Z => &cc_regs::Z_AXIS_POSITION.address,
        };
        let position_hundredths = read_u16(registers, register).await?;
        let position_inches = position_hundredths as f64 / 100.0;
        Ok(format!("{position_inches:.2} in"))
    } else {
        Err(format!("{axis_label} is not homed"))
    }
}
