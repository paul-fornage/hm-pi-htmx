use crate::modbus::cached_modbus::CachedModbus;
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::shared::result_feedback::FeedbackResult;
use crate::AppState;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, Path, State};
use axum::response::Html;
use serde::Deserialize;

use super::helpers::read_bool;

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-modal.html")]
pub(super) struct GoToPositionModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-feedback.html")]
pub(super) struct GoToPositionFeedbackTemplate {
    result: Result<String, String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/relative-move-modal.html")]
pub(super) struct RelativeMoveModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/go-to-position-feedback.html")]
pub(super) struct RelativeMoveFeedbackTemplate {
    result: Result<String, String>,
}

impl From<Result<String, String>> for RelativeMoveFeedbackTemplate {
    fn from(value: Result<String, String>) -> Self {
        Self { result: value }
    }
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
    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

#[derive(Deserialize)]
pub(super) struct GoToPositionForm {
    value: String,
}

#[derive(Deserialize)]
pub(super) struct RelativeMoveForm {
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

    fn hreg(&self) -> &'static crate::modbus::RegisterAddress {
        match self {
            Self::X => &cc_regs::X_AXIS_GO_TO_POSITION.address,
            Self::Y => &cc_regs::Y_AXIS_GO_TO_POSITION.address,
            Self::Z => &cc_regs::Z_AXIS_GO_TO_POSITION.address,
        }
    }

    fn latch(&self) -> &'static crate::modbus::RegisterAddress {
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

pub(super) async fn go_to_position_modal_handler(Path(axis): Path<String>) -> Html<String> {
    let html = match GoToAxis::from_str(&axis) {
        Some(axis) => GoToPositionModalTemplate {
            axis_label: axis.label().to_string(),
            post_url: format!("/clearcore-manual-control/go-to-position/{}", axis.slug()),
            feedback_target: format!("#go-to-position-feedback-{}", axis.slug()),
        }
        .render()
        .unwrap(),
        None => FeedbackResult::<String, String>::new_err(format!("Unknown axis: {axis}"))
            .render()
            .unwrap(),
    };
    Html(html)
}

pub(super) async fn relative_move_modal_handler(Path(axis): Path<String>) -> Html<String> {
    let html = match RelativeMoveAxis::from_str(&axis) {
        Some(axis) => RelativeMoveModalTemplate {
            axis_label: axis.label().to_string(),
            post_url: format!("/clearcore-manual-control/relative-move/{}", axis.slug()),
            feedback_target: format!("#relative-move-feedback-{}", axis.slug()),
        }
        .render()
        .unwrap(),
        None => FeedbackResult::<String, String>::new_err(format!("Unknown axis: {axis}"))
            .render()
            .unwrap(),
    };
    Html(html)
}

pub(super) async fn go_to_position_submit_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<GoToPositionForm>,
) -> GoToPositionFeedbackTemplate {
    let axis = match GoToAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return GoToPositionFeedbackTemplate::err(format!("Unknown axis: {axis}")),
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

pub(super) async fn relative_move_submit_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<RelativeMoveForm>,
) -> RelativeMoveFeedbackTemplate {
    let axis = match RelativeMoveAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return RelativeMoveFeedbackTemplate::err(format!("Unknown axis: {axis}")),
    };

    let delta_inches = match form.value.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return RelativeMoveFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    match axis {
        RelativeMoveAxis::W => {
            RelativeMoveFeedbackTemplate::from(
                relative_move_w_axis(&state.clearcore_registers, delta_inches).await,
            )
        }
    }
}

async fn relative_move_w_axis(
    clearcore_registers: &CachedModbus,
    delta_inches: f64,
) -> Result<String, String> {
    const MAX_RELATIVE_MOVE_W_INCHES: f64 = 12.0;
    if delta_inches < -MAX_RELATIVE_MOVE_W_INCHES || delta_inches > MAX_RELATIVE_MOVE_W_INCHES {
        return Err(format!(
            "Delta must be between -{MAX_RELATIVE_MOVE_W_INCHES} and {MAX_RELATIVE_MOVE_W_INCHES} inches."
        ));
    }

    let move_hundredths: i16 = (delta_inches * 100.0).round() as i16;
    let move_shifted: u16 = (move_hundredths - i16::MIN) as u16;

    clearcore_registers
        .write_hreg(
            cc_regs::W_AXIS_RELATIVE_GO_TO_POTION.address.address,
            move_shifted,
        )
        .await
        .map_err(|err| format!("Could not write W jog distance to modbus: {err}"))?;
    clearcore_registers
        .write_coil(
            cc_regs::W_AXIS_GO_TO_RELATIVE_POSITION_LATCH.address.address,
            true,
        )
        .await
        .map_err(|err| format!("Could not write W jog enable to modbus: {err}"))?;
    Ok(format!("Relative move of {delta_inches} inches requested"))
}
