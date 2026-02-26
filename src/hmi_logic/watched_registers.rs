use axum::response::Sse;
use tokio::sync::broadcast;
use crate::hmi_logic::mb_watcher::WatchedRegister;
use crate::miller::miller_register_definitions;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::ModbusValue;
use crate::plc::plc_register_definitions;
use crate::{error_targeted, info_targeted, trace_targeted, warn_targeted};
use crate::sse::error_toast::ErrorToast;
use crate::sse::SseEvent;
use crate::sse::EstopStateUpdate;

pub fn touch_retract_passthrough(miller_regs: CachedModbus) -> WatchedRegister {
    WatchedRegister {
        address: &plc_register_definitions::TOUCH_RETRACT_REQUESTED.address,
        handler: Box::new(move |new_value| {
            info_targeted!(MODBUS, "Received touch retract requested: {:?}", new_value);
            let miller_regs = miller_regs.clone();

            // Spawn an async task to handle the write since the handler is sync
            tokio::spawn(async move {
                match new_value {
                    Some(ModbusValue::Bool(requested)) => {
                        match miller_regs.write_coil(
                            miller_register_definitions::TOUCH_SENSE_EN.address.address,
                            requested
                        ).await {
                            Ok(()) => {
                                trace_targeted!(MODBUS, "Touched retract requested: {}", requested);
                            }
                            Err(e) => {
                                warn_targeted!(MODBUS, "Error writing touch retract requested: {:?}", e);
                            }
                        }
                    }
                    Some(ModbusValue::U16(_a_word_reg)) => {
                        error_targeted!(MODBUS, "Received U16 value for touch retract requested");
                    }
                    None => {
                        warn_targeted!(MODBUS, "Received None value for touch retract requested");
                    }
                }
            });
        }),
    }
}


pub fn estop_emitter(sse_tx: broadcast::Sender<SseEvent>) -> WatchedRegister {
    WatchedRegister {
        address: &plc_register_definitions::IN_ESTOP.address,
        handler: Box::new(move |new_value| {
            info_targeted!(MODBUS, "Received Estop state update: {:?}", new_value);
            let in_estop = match new_value {
                Some(ModbusValue::Bool(in_estop)) => Some(in_estop),
                Some(ModbusValue::U16(_a_word_reg)) => None,
                None => None,
            };
            match sse_tx.send(EstopStateUpdate { in_estop }.into()){
                Ok(_) => {}
                Err(e) => {
                    warn_targeted!(MODBUS, "Failed to send EstopStateUpdate SSE event: {:?}", e);
                }
            }
            let message = match new_value {
                Some(ModbusValue::Bool(in_estop)) => {
                    Some((if in_estop {
                        "E-stop activated"
                    } else {
                        "E-stop de-activated"
                    }).to_string())
                }
                Some(ModbusValue::U16(_a_word_reg)) => {
                    error_targeted!(MODBUS, "Received U16 value for Estop state update");
                    Some("Critical error in e-stop system. \
                        This should never happen, contact Mitusa for support".to_string())
                }
                None => {
                    Some("E-stop state changed to unknown".to_string())
                }
            };
            if let Some(msg) = message {
                match sse_tx.send(ErrorToast{ msg }.into()){
                    Ok(_) => {}
                    Err(e) => {
                        warn_targeted!(MODBUS, "Failed to send EstopStateUpdate SSE event: {:?}", e);
                    }
                }
            }
        }),
    }
}
