use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::IntoResponse;
use crate::AppState;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterAddress};
use crate::plc::plc_register_definitions as cc_regs;
use crate::views::{AppView, ViewTemplate};
use crate::views::shared::result_feedback::FeedbackResult;


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

pub async fn show_manual_control() -> impl IntoResponse {
    ManualControlTemplate {}
}

pub async fn home_all_axes_handler(State(state): State<AppState>) -> FeedbackResult<String, String> {
    home_all_axes(&state.clearcore_registers).await.into()
}

pub async fn home_all_axes(clearcore_registers: &CachedModbus) -> Result<String, String> {
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

    Ok("Homing requested".into())
}

pub async fn get_x_position_handler(State(state): State<AppState>) -> FeedbackResult<String, String> {
    get_x_position(&state.clearcore_registers).await.into()
}

pub async fn get_x_position(registers: &CachedModbus) -> Result<String, String> {
    let is_homed = read_bool(registers, &cc_regs::IS_HOMED.address).await?;
    if is_homed {
        let position_hundredths = read_u16(registers, &cc_regs::CURRENT_POSITION.address).await?;
        let position_inches = position_hundredths as f64 / 100.0;
        Ok(format!("{:.2} in", position_inches))
    } else {
        Err("Clearcore is not homed".into())
    }
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