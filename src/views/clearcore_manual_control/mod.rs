use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterAddress};
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;
use crate::views::shared::finger_status::{finger_status_handler, FingerSide};
use crate::views::shared::result_feedback::FeedbackResult;
use crate::views::shared::{mb_read_bool_helper, StatusFeedbackTemplate};
use crate::views::{build_header_context, AppView, HeaderContext, ViewTemplate};
use crate::{warn_targeted, AppState};
use crate::file_io::FileIoError;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::extract::{Form, Path};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;

const GAS_PURGE_MIN_SECONDS: f64 = 0.0;
const GAS_PURGE_MAX_SECONDS: f64 = 30.0;
const GAS_PURGE_DEFAULT_SECONDS: f64 = 5.0;
const GAS_PURGE_STEP_SECONDS: f64 = 0.1;

macro_rules! read_or_bail {
    ($reg:ident) => {{
        match state
            .clearcore_registers
            .read(cc_regs::$reg.address)
            .await
        {
            Some(value) => value,
            None => {
                return FeedbackResult::new_err(
                    format!("{}{}",
                        concat!("Failed to read ", stringify!($reg), " holding register at "),
                        cc_regs::$reg.address.address).into(),
                );
            }
        }
    }};
}


#[derive(Template, WebTemplate)]
#[template(path = "views/manual-control.html")]
pub struct ManualControlTemplate {
    pub header: HeaderContext,
    pub show_y_axis: bool,
    pub show_z_axis: bool,
    pub show_w_axis: bool,
}
impl ViewTemplate for ManualControlTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::ClearcoreManualControl;
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(AppView::ClearcoreManualControl.url(), get(show_manual_control))
        .route(&AppView::ClearcoreManualControl.url_with_path("/home-axes"), post(home_all_axes_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/homing-status"), get(homing_status_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/finger-status/{side}"), get(finger_status_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/finger/{side}/{action}"), post(finger_command_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/status-feedback"), get(manual_control_status_feedback))
        .route(&AppView::ClearcoreManualControl.url_with_path("/x-position"), get(get_x_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/y-position"), get(get_y_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/z-position"), get(get_z_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"), get(jog_speed_modal_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"), post(jog_speed_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed-display/{axis}"), get(jog_speed_display_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog/{axis}/{direction}"), post(jog_command_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/relative-move/{axis}"), get(relative_move_modal_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/relative-move/{axis}"), post(relative_move_submit_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/gas-purge"), get(gas_purge_modal_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/gas-purge"), post(gas_purge_submit_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/go-to-position/{axis}"), get(go_to_position_modal_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/go-to-position/{axis}"), post(go_to_position_submit_handler))
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/homing-status.html")]
pub struct HomingStatusTemplate {
    is_homing: bool,
    is_homed: bool,
    error: Option<String>,
}



pub async fn show_manual_control(State(state): State<AppState>) -> impl IntoResponse {
    let header = build_header_context(&state, AppView::ClearcoreManualControl).await;
    let (show_y_axis, show_z_axis, show_w_axis) = match ClearcoreConfig::load_config().await {
        Ok(config) => {
            let show_y = config.coils.get(cc_regs::USES_Y_AXIS.name).copied().unwrap_or(true);
            let show_z = config.coils.get(cc_regs::USES_Z_AXIS.name).copied().unwrap_or(true);
            let show_w = config.coils.get(cc_regs::USES_W_AXIS.name).copied().unwrap_or(true);
            (show_y, show_z, show_w)
        }
        Err(FileIoError::NotFound { .. }) => {
            warn_targeted!(FS, "Clearcore static config not found; showing all axes in manual control");
            (true, true, true)
        }
        Err(err) => {
            warn_targeted!(FS, "Failed to load clearcore static config for manual control: {}", err);
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

pub async fn homing_status_handler(State(state): State<AppState>) -> impl IntoResponse {
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



pub async fn manual_control_status_feedback(State(state): State<AppState>) -> impl IntoResponse {
    let mandrel_latch_closed = mb_read_bool_helper(
        &state.clearcore_registers,
        &cc_regs::MANDREL_LATCH_CLOSED.address,
    )
    .await;

    StatusFeedbackTemplate {
        mandrel_latch_closed,
    }
}

pub async fn home_all_axes_handler(State(state): State<AppState>) -> impl IntoResponse {
    let action_result = home_all_axes(&state.clearcore_registers).await;
    let status_result = get_homing_status(&state.clearcore_registers).await;
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

pub async fn home_all_axes(clearcore_registers: &CachedModbus) -> Result<(), String> {
    let is_homing = clearcore_registers.read_coil(cc_regs::IS_HOMING.address.address).await;
    match is_homing {
        Some(false) => {},
        Some(true) => return Err("Homing in progress".into()),
        None => return Err("Error reading homing status".into()),
    }
    let homing_already_commanded = clearcore_registers.read_coil(cc_regs::HOME_LATCH.address.address).await;
    match homing_already_commanded {
        Some(false) => {},
        Some(true) => return Err("Homing already commanded and clearcore isn't homing".into()),
        None => return Err("Error reading homing status".into()),
    }
    clearcore_registers.write_coil(cc_regs::HOME_LATCH.address.address, true).await.map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_x_position_handler(State(state): State<AppState>) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::X).await.into()
}

pub async fn get_y_position_handler(State(state): State<AppState>) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::Y).await.into()
}

pub async fn get_z_position_handler(State(state): State<AppState>) -> FeedbackResult<String, String> {
    get_axis_position(&state.clearcore_registers, AxisPosition::Z).await.into()
}


#[derive(Clone, Copy)]
enum FingerCommand {
    Up,
    Down,
}

impl FingerCommand {
    fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
        }
    }

    fn latch_for(&self, side: FingerSide) -> &'static RegisterAddress {
        match (side, self) {
            (FingerSide::Left, Self::Up) => &cc_regs::COMMAND_LF_UP_LATCH.address,
            (FingerSide::Left, Self::Down) => &cc_regs::COMMAND_LF_DOWN_LATCH.address,
            (FingerSide::Right, Self::Up) => &cc_regs::COMMAND_RF_UP_LATCH.address,
            (FingerSide::Right, Self::Down) => &cc_regs::COMMAND_RF_DOWN_LATCH.address,
        }
    }
}

pub async fn finger_command_handler(
    State(state): State<AppState>,
    Path((side, action)): Path<(String, String)>,
) -> FeedbackResult<String, String> {
    let side = match FingerSide::from_str(&side) {
        Some(side) => side,
        None => return FeedbackResult::new_err(format!("Unknown finger side: {}", side)),
    };

    let command = match FingerCommand::from_str(&action) {
        Some(command) => command,
        None => return FeedbackResult::new_err(format!("Unknown finger command: {}", action)),
    };

    if let Err(err) = state
        .clearcore_registers
        .write_coil(command.latch_for(side).address, true)
        .await
    {
        return FeedbackResult::new_err(err.to_string());
    }

    FeedbackResult::new_ok(format!(
        "Commanded {} fingers {}.",
        side.label(),
        command.label()
    ))
}

#[derive(Clone, Copy)]
enum JogAxis {
    X,
    Y,
    Z,
    W,
}

impl JogAxis {
    fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "x" => Some(Self::X),
            "y" => Some(Self::Y),
            "z" => Some(Self::Z),
            "w" => Some(Self::W),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::X => "X Axis",
            Self::Y => "Y Axis",
            Self::Z => "Z Axis",
            Self::W => "W Axis",
        }
    }

    fn slug(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::W => "w",
        }
    }

    fn speed_hreg(&self) -> &'static RegisterAddress {
        match self {
            Self::X => &cc_regs::AXIS_X_COMMANDED_JOG_SPEED.address,
            Self::Y => &cc_regs::AXIS_Y_COMMANDED_JOG_SPEED.address,
            Self::Z => &cc_regs::AXIS_Z_COMMANDED_JOG_SPEED.address,
            Self::W => &cc_regs::AXIS_W_COMMANDED_JOG_SPEED.address,
        }
    }

    fn jog_positive_coil(&self) -> &'static RegisterAddress {
        match self {
            Self::X => &cc_regs::JOG_X_AXIS_POSITIVE.address,
            Self::Y => &cc_regs::JOG_Y_AXIS_POSITIVE.address,
            Self::Z => &cc_regs::JOG_Z_AXIS_POSITIVE.address,
            Self::W => &cc_regs::JOG_W_AXIS_POSITIVE.address,
        }
    }

    fn jog_negative_coil(&self) -> &'static RegisterAddress {
        match self {
            Self::X => &cc_regs::JOG_X_AXIS_NEGATIVE.address,
            Self::Y => &cc_regs::JOG_Y_AXIS_NEGATIVE.address,
            Self::Z => &cc_regs::JOG_Z_AXIS_NEGATIVE.address,
            Self::W => &cc_regs::JOG_W_AXIS_NEGATIVE.address,
        }
    }
}

#[derive(Clone, Copy)]
enum JogDirection {
    Positive,
    Negative,
}

impl JogDirection {
    fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "positive" => Some(Self::Positive),
            "negative" => Some(Self::Negative),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
        }
    }

    fn target_coil(&self, axis: JogAxis) -> &'static RegisterAddress {
        match self {
            Self::Positive => axis.jog_positive_coil(),
            Self::Negative => axis.jog_negative_coil(),
        }
    }

    fn opposite_coil(&self, axis: JogAxis) -> &'static RegisterAddress {
        match self {
            Self::Positive => axis.jog_negative_coil(),
            Self::Negative => axis.jog_positive_coil(),
        }
    }
}

pub enum AxisPosition {
    X,
    Y,
    Z,
}

pub async fn get_axis_position(registers: &CachedModbus, axis: AxisPosition) -> Result<String, String> {
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
        Ok(format!("{:.2} in", position_inches))
    } else {
        Err(format!("{axis_label} is not homed"))
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-display.html")]
pub struct JogSpeedDisplayTemplate {
    axis_label: String,
    axis_slug: String,
    speed_display: String,
    error: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-modal.html")]
pub struct JogSpeedModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
    current_speed: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-feedback.html")]
pub struct JogFeedbackTemplate {
    result: Result<String, String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-feedback.html")]
pub struct JogSpeedFeedbackTemplate {
    result: Result<String, String>,
}

impl JogFeedbackTemplate {
    fn ok(message: String) -> Self {
        Self { result: Ok(message) }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

impl JogSpeedFeedbackTemplate {
    fn ok(message: String) -> Self {
        Self { result: Ok(message) }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

#[derive(Deserialize)]
pub struct JogSpeedForm {
    value: String,
}

#[derive(Deserialize)]
pub struct JogCommandForm {
    active: bool,
}

pub async fn jog_speed_display_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
) -> JogSpeedDisplayTemplate {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => {
            return JogSpeedDisplayTemplate {
                axis_label: "Unknown Axis".to_string(),
                axis_slug: axis,
                speed_display: "--".to_string(),
                error: Some("Unknown axis".to_string()),
            };
        }
    };

    match read_u16(&state.clearcore_registers, axis.speed_hreg()).await {
        Ok(speed_raw) => {
            let speed_in_min = speed_raw as f64 / 100.0;
            JogSpeedDisplayTemplate {
                axis_label: axis.label().to_string(),
                axis_slug: axis.slug().to_string(),
                speed_display: format!("{:.2}", speed_in_min),
                error: None,
            }
        }
        Err(err) => JogSpeedDisplayTemplate {
            axis_label: axis.label().to_string(),
            axis_slug: axis.slug().to_string(),
            speed_display: "--".to_string(),
            error: Some(err),
        },
    }
}

pub async fn jog_speed_modal_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
) -> Html<String> {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => {
            let html = FeedbackResult::<String, String>::new_err(format!("Unknown axis: {}", axis))
                .render()
                .unwrap();
            return Html(html);
        }
    };

    let current_speed = match read_u16(&state.clearcore_registers, axis.speed_hreg()).await {
        Ok(speed_raw) => format!("{:.2}", speed_raw as f64 / 100.0),
        Err(_) => "0.00".to_string(),
    };

    let html = JogSpeedModalTemplate {
        axis_label: axis.label().to_string(),
        post_url: format!("/clearcore-manual-control/jog-speed/{}", axis.slug()),
        feedback_target: format!("#jog-feedback-{}", axis.slug()),
        current_speed,
    }
    .render()
    .unwrap();
    Html(html)
}

pub async fn jog_speed_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<JogSpeedForm>,
) -> JogSpeedFeedbackTemplate {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return JogSpeedFeedbackTemplate::err(format!("Unknown axis: {}", axis)),
    };

    let speed_in_min = match form.value.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return JogSpeedFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    if speed_in_min < 0.0 {
        return JogSpeedFeedbackTemplate::err("Speed must be zero or greater.".to_string());
    }

    let speed_hundredths = (speed_in_min * 100.0).round();
    if speed_hundredths > u16::MAX as f64 {
        return JogSpeedFeedbackTemplate::err("Speed exceeds register range.".to_string());
    }

    let speed_raw = speed_hundredths as u16;
    if let Err(err) = state
        .clearcore_registers
        .write_hreg(axis.speed_hreg().address, speed_raw)
        .await
    {
        return JogSpeedFeedbackTemplate::err(err.to_string());
    }

    JogSpeedFeedbackTemplate::ok(format!(
        "Set {} jog speed to {:.2} in/min.",
        axis.label(),
        speed_hundredths / 100.0
    ))
}

pub async fn jog_command_handler(
    State(state): State<AppState>,
    Path((axis, direction)): Path<(String, String)>,
    Form(form): Form<JogCommandForm>,
) -> JogFeedbackTemplate {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return JogFeedbackTemplate::err(format!("Unknown axis: {}", axis)),
    };

    let direction = match JogDirection::from_str(&direction) {
        Some(direction) => direction,
        None => return JogFeedbackTemplate::err(format!("Unknown direction: {}", direction)),
    };

    if form.active {
        if let Err(err) = state
            .clearcore_registers
            .write_coil(direction.opposite_coil(axis).address, false)
            .await
        {
            return JogFeedbackTemplate::err(err.to_string());
        }
    }

    if let Err(err) = state
        .clearcore_registers
        .write_coil(direction.target_coil(axis).address, form.active)
        .await
    {
        return JogFeedbackTemplate::err(err.to_string());
    }

    if form.active {
        JogFeedbackTemplate::ok(format!(
            "Jogging {} {}.",
            axis.label(),
            direction.label()
        ))
    } else {
        JogFeedbackTemplate::ok(String::new())
    }
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

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/gas-purge-modal.html")]
pub struct GasPurgeModalTemplate {
    post_url: String,
    feedback_target: String,
    min_seconds: String,
    max_seconds: String,
    step_seconds: String,
    prefill_seconds: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/gas-purge-feedback.html")]
pub struct GasPurgeFeedbackTemplate {
    result: Result<String, String>,
}

impl GasPurgeFeedbackTemplate {
    fn ok(message: String) -> Self {
        Self { result: Ok(message) }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

#[derive(Deserialize)]
pub struct GasPurgeForm {
    duration_seconds: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-modal.html")]
pub struct GoToPositionModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-feedback.html")]
pub struct GoToPositionFeedbackTemplate {
    result: Result<String, String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/relative-move-modal.html")]
pub struct RelativeMoveModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-feedback.html")]
pub struct RelativeMoveFeedbackTemplate {
    result: Result<String, String>,
}

impl GoToPositionFeedbackTemplate {
    fn ok(message: String) -> Self {
        Self { result: Ok(message) }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

impl RelativeMoveFeedbackTemplate {
    fn ok(message: String) -> Self {
        Self { result: Ok(message) }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

#[derive(Deserialize)]
pub struct GoToPositionForm {
    value: String,
}

#[derive(Deserialize)]
pub struct RelativeMoveForm {
    value: String,
}

#[derive(Clone, Copy)]
enum GoToAxis {
    X,
    Y,
    Z,
}

impl GoToAxis {
    fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "x" => Some(Self::X),
            "y" => Some(Self::Y),
            "z" => Some(Self::Z),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::X => "X Axis",
            Self::Y => "Y Axis",
            Self::Z => "Z Axis",
        }
    }

    fn slug(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
        }
    }

    fn hreg(&self) -> &'static RegisterAddress {
        match self {
            Self::X => &cc_regs::X_AXIS_GO_TO_POSITION.address,
            Self::Y => &cc_regs::Y_AXIS_GO_TO_POSITION.address,
            Self::Z => &cc_regs::Z_AXIS_GO_TO_POSITION.address,
        }
    }

    fn latch(&self) -> &'static RegisterAddress {
        match self {
            Self::X => &cc_regs::X_AXIS_GO_TO_POSITION_LATCH.address,
            Self::Y => &cc_regs::Y_AXIS_GO_TO_POSITION_LATCH.address,
            Self::Z => &cc_regs::Z_AXIS_GO_TO_POSITION_LATCH.address,
        }
    }
}

#[derive(Clone, Copy)]
enum RelativeMoveAxis {
    W,
}

impl RelativeMoveAxis {
    fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "w" => Some(Self::W),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::W => "W Axis",
        }
    }

    fn slug(&self) -> &'static str {
        match self {
            Self::W => "w",
        }
    }
}

pub async fn go_to_position_modal_handler(Path(axis): Path<String>) -> Html<String> {
    let html = match GoToAxis::from_str(&axis) {
        Some(axis) => GoToPositionModalTemplate {
            axis_label: axis.label().to_string(),
            post_url: format!("/clearcore-manual-control/go-to-position/{}", axis.slug()),
            feedback_target: format!("#go-to-position-feedback-{}", axis.slug()),
        }
        .render()
        .unwrap(),
        None => FeedbackResult::<String, String>::new_err(format!("Unknown axis: {}", axis))
            .render()
            .unwrap(),
    };
    Html(html)
}

pub async fn relative_move_modal_handler(Path(axis): Path<String>) -> Html<String> {
    let html = match RelativeMoveAxis::from_str(&axis) {
        Some(axis) => RelativeMoveModalTemplate {
            axis_label: axis.label().to_string(),
            post_url: format!("/clearcore-manual-control/relative-move/{}", axis.slug()),
            feedback_target: format!("#relative-move-feedback-{}", axis.slug()),
        }
        .render()
        .unwrap(),
        None => FeedbackResult::<String, String>::new_err(format!("Unknown axis: {}", axis))
            .render()
            .unwrap(),
    };
    Html(html)
}

pub async fn gas_purge_modal_handler() -> Html<String> {
    let html = GasPurgeModalTemplate {
        post_url: "/clearcore-manual-control/gas-purge".to_string(),
        feedback_target: "#gas-purge-feedback".to_string(),
        min_seconds: format!("{:.1}", GAS_PURGE_MIN_SECONDS),
        max_seconds: format!("{:.1}", GAS_PURGE_MAX_SECONDS),
        step_seconds: format!("{:.1}", GAS_PURGE_STEP_SECONDS),
        prefill_seconds: format!("{:.1}", GAS_PURGE_DEFAULT_SECONDS),
    }
    .render()
    .unwrap();
    Html(html)
}

pub async fn gas_purge_submit_handler(
    State(state): State<AppState>,
    Form(form): Form<GasPurgeForm>,
) -> GasPurgeFeedbackTemplate {
    let duration_seconds = match form.duration_seconds.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return GasPurgeFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    if !(GAS_PURGE_MIN_SECONDS..=GAS_PURGE_MAX_SECONDS).contains(&duration_seconds) {
        return GasPurgeFeedbackTemplate::err("Purge time must be between 0 and 30 seconds.".to_string());
    }

    match gas_purge(&state.clearcore_registers, duration_seconds).await {
        Ok(message) => GasPurgeFeedbackTemplate::ok(message),
        Err(err) => GasPurgeFeedbackTemplate::err(err),
    }
}

pub async fn go_to_position_submit_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<GoToPositionForm>,
) -> GoToPositionFeedbackTemplate {
    let axis = match GoToAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return GoToPositionFeedbackTemplate::err(format!("Unknown axis: {}", axis)),
    };

    let is_homed = match read_bool(&state.clearcore_registers, &cc_regs::IS_HOMED.address).await {
        Ok(value) => value,
        Err(err) => return GoToPositionFeedbackTemplate::err(err),
    };
    if !is_homed {
        return GoToPositionFeedbackTemplate::err("Clearcore is not homed".to_string());
    }

    let position_inches = match form.value.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return GoToPositionFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    if position_inches < 0.0 {
        return GoToPositionFeedbackTemplate::err("Position must be zero or greater.".to_string());
    }

    let position_hundredths = (position_inches * 100.0).round();
    if position_hundredths > u16::MAX as f64 {
        return GoToPositionFeedbackTemplate::err("Position exceeds register range.".to_string());
    }

    let position_raw = position_hundredths as u16;
    if let Err(err) = state
        .clearcore_registers
        .write_hreg(axis.hreg().address, position_raw)
        .await
    {
        return GoToPositionFeedbackTemplate::err(err.to_string());
    }
    if let Err(err) = state
        .clearcore_registers
        .write_coil(axis.latch().address, true)
        .await
    {
        return GoToPositionFeedbackTemplate::err(err.to_string());
    }

    GoToPositionFeedbackTemplate::ok(format!(
        "Commanded move to {:.2} in on {}.",
        position_hundredths / 100.0,
        axis.label()
    ))
}

pub async fn relative_move_submit_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<RelativeMoveForm>,
) -> RelativeMoveFeedbackTemplate {
    let axis = match RelativeMoveAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return RelativeMoveFeedbackTemplate::err(format!("Unknown axis: {}", axis)),
    };

    let delta_inches = match form.value.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return RelativeMoveFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    match relative_move_w_axis(&state.clearcore_registers, delta_inches).await {
        Ok(message) => RelativeMoveFeedbackTemplate::ok(message),
        Err(err) => RelativeMoveFeedbackTemplate::err(err),
    }
}

pub async fn gas_purge(
    _clearcore_registers: &CachedModbus,
    _duration_seconds: f64,
) -> Result<String, String> {
    todo!()
}

pub async fn relative_move_w_axis(
    _clearcore_registers: &CachedModbus,
    _delta_inches: f64,
) -> Result<String, String> {
    todo!()
}


pub async fn read_bool(registers: &CachedModbus, address: &RegisterAddress) -> Result<bool, String>{
    match registers.read(address).await {
        Some(ModbusValue::Bool(value)) => Ok(value),
        Some(_) => Err(format!("Register at {} is not a boolean", address)),
        None => Err(format!("Register at {} could not be read", address))
    }
}

pub async fn read_u16(registers: &CachedModbus, address: &RegisterAddress) -> Result<u16, String>{
    match registers.read(address).await {
        Some(ModbusValue::U16(value)) => Ok(value),
        Some(_) => Err(format!("Register at {} is not a u16", address)),
        None => Err(format!("Register at {} could not be read", address))
    }
}
