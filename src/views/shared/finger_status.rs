use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use crate::{warn_targeted, AppState};
use crate::views::shared::mb_read_bool_helper;
use crate::plc::plc_register_definitions as cc_regs;

#[derive(Template, WebTemplate)]
#[template(path = "components/shared/finger-status.html")]
pub struct FingerStatusTemplate {
    pub state: Option<bool>,
}


#[derive(Clone, Copy)]
pub enum FingerSide {
    Left,
    Right,
}

impl FingerSide {
    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }
}

pub async fn finger_status_handler(
    State(state): State<AppState>,
    Path(path): Path<String>
) -> FingerStatusTemplate {

    let side = FingerSide::from_str(&path);
    match side {
        None => {
            warn_targeted!(HTTP, "Invalid finger side provided: {}", path);
            FingerStatusTemplate{
                state: None
            }
        }
        Some(FingerSide::Left) => {
            FingerStatusTemplate{
                state: mb_read_bool_helper(
                    &state.clearcore_registers,
                    &cc_regs::LF_COMMANDED_DOWN.address,
                ).await,
            }
        }
        Some(FingerSide::Right) => {
            FingerStatusTemplate{
                state: mb_read_bool_helper(
                    &state.clearcore_registers,
                    &cc_regs::RF_COMMANDED_DOWN.address,
                ).await,
            }
        }
    }
}