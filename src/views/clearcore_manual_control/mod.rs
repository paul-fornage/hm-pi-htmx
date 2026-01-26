use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::Router;
use crate::AppState;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterAddress};
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::{AppView, ViewTemplate};
use crate::views::shared::result_feedback::FeedbackResult;
use axum::extract::{Form, Path};
use serde::Deserialize;


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
pub struct ManualControlTemplate {}
impl ViewTemplate for ManualControlTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::ClearcoreManualControl;
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(AppView::ClearcoreManualControl.url(), get(show_manual_control))
        .route(&AppView::ClearcoreManualControl.url_with_path("/home-axes"), post(home_all_axes_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/homing-status"), get(homing_status_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/x-position"), get(get_x_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/y-position"), get(get_y_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/z-position"), get(get_z_position_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"), get(jog_speed_modal_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"), post(jog_speed_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog-speed-display/{axis}"), get(jog_speed_display_handler))
        .route(&AppView::ClearcoreManualControl.url_with_path("/jog/{axis}/{direction}"), post(jog_command_handler))
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

pub async fn show_manual_control() -> impl IntoResponse {
    ManualControlTemplate {}
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

enum AxisPosition {
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
    let is_homed = read_bool(registers, is_homed_register).await.inspect_err(|err| log::error!("Error reading {} homed status: {}", axis_label, err))?;
    if is_homed {
        let register = match axis {
            AxisPosition::X => &cc_regs::X_AXIS_POSITION.address,
            AxisPosition::Y => &cc_regs::Y_AXIS_POSITION.address,
            AxisPosition::Z => &cc_regs::Z_AXIS_POSITION.address,
        };
        let position_hundredths = read_u16(registers, register).await.inspect_err(|err| log::error!("Error reading {} position: {}", axis_label, err))?;
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

impl GoToPositionFeedbackTemplate {
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
