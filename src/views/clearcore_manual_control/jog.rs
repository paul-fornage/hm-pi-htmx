use crate::modbus::RegisterAddress;
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::shared::result_feedback::FeedbackResult;
use crate::{warn_targeted, AppState};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, Path, State};
use axum::response::Html;
use serde::Deserialize;

use super::helpers::read_u16;

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

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-display.html")]
pub(super) struct JogSpeedDisplayTemplate {
    axis_label: String,
    axis_slug: String,
    speed_display: String,
    error: Option<String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-modal.html")]
pub(super) struct JogSpeedModalTemplate {
    axis_label: String,
    post_url: String,
    feedback_target: String,
    current_speed: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-feedback.html")]
pub(super) struct JogFeedbackTemplate {
    result: Result<String, String>,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/jog-speed-feedback.html")]
pub(super) struct JogSpeedFeedbackTemplate {
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
pub(super) struct JogSpeedForm {
    value: String,
}

#[derive(Deserialize)]
pub(super) struct JogCommandForm {
    active: bool,
}

pub(super) async fn jog_speed_display_handler(
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
                speed_display: format!("{speed_in_min:.2}"),
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

pub(super) async fn jog_speed_modal_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
) -> Html<String> {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => {
            let html =
                FeedbackResult::<String, String>::new_err(format!("Unknown axis: {axis}"))
                    .render()
                    .unwrap();
            return Html(html);
        }
    };

    let current_speed = match read_u16(&state.clearcore_registers, axis.speed_hreg()).await {
        Ok(speed_raw) => format!("{:.2}", speed_raw as f64 / 100.0),
        Err(err) => {
            warn_targeted!(
                MODBUS,
                "Failed to read {} jog speed: {err}",
                axis.label()
            );
            "0.00".to_string()
        }
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

pub(super) async fn jog_speed_handler(
    State(state): State<AppState>,
    Path(axis): Path<String>,
    Form(form): Form<JogSpeedForm>,
) -> JogSpeedFeedbackTemplate {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return JogSpeedFeedbackTemplate::err(format!("Unknown axis: {axis}")),
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

pub(super) async fn jog_command_handler(
    State(state): State<AppState>,
    Path((axis, direction)): Path<(String, String)>,
    Form(form): Form<JogCommandForm>,
) -> JogFeedbackTemplate {
    let axis = match JogAxis::from_str(&axis) {
        Some(axis) => axis,
        None => return JogFeedbackTemplate::err(format!("Unknown axis: {axis}")),
    };

    let direction = match JogDirection::from_str(&direction) {
        Some(direction) => direction,
        None => return JogFeedbackTemplate::err(format!("Unknown direction: {direction}")),
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
