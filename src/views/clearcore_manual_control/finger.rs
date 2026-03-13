use crate::modbus::RegisterAddress;
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::shared::finger_status::FingerSide;
use crate::views::shared::result_feedback::FeedbackResult;
use crate::AppState;
use axum::extract::{Path, State};

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

pub(super) async fn finger_command_handler(
    State(state): State<AppState>,
    Path((side, action)): Path<(String, String)>,
) -> FeedbackResult<String, String> {
    let side = match FingerSide::from_str(&side) {
        Some(side) => side,
        None => return FeedbackResult::new_err(format!("Unknown finger side: {side}")),
    };

    let command = match FingerCommand::from_str(&action) {
        Some(command) => command,
        None => return FeedbackResult::new_err(format!("Unknown finger command: {action}")),
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
