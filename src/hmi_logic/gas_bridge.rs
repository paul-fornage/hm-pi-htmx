use crate::miller::miller_register_definitions;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::ModbusState;
use crate::plc::plc_register_definitions;
use crate::{info_targeted, warn_targeted};

const GAS_BRIDGE_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);
const READ_ERROR_LOG_THROTTLE: std::time::Duration = std::time::Duration::from_secs(5);

pub async fn gas_bridge_task(
    clearcore_registers: CachedModbus,
    miller_registers: CachedModbus,
) -> ! {
    let mut interval = tokio::time::interval(GAS_BRIDGE_INTERVAL);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    let mut last_read_error_log: Option<std::time::Instant> = None;

    loop {
        interval.tick().await;

        let welder_gas_on = miller_registers
            .read_disc(miller_register_definitions::GAS_OUTPUT_ENABLED.address.address)
            .await
            .unwrap_or(false);

        let clearcore_requested_on_opt = clearcore_registers
            .read_coil(plc_register_definitions::ENABLE_GAS_OUTPUT.address.address)
            .await;
        let clearcore_requested_on = clearcore_requested_on_opt.unwrap_or(false);

        let clearcore_connected = matches!(
            clearcore_registers.get_connection_state().await,
            Ok(ModbusState::Connected)
        );

        if clearcore_requested_on_opt.is_none() && clearcore_connected {
            let should_log = match last_read_error_log {
                Some(last) => last.elapsed() >= READ_ERROR_LOG_THROTTLE,
                None => true,
            };
            if should_log {
                warn_targeted!(
                    MODBUS,
                    "Gas bridge read failed: clearcore requested gas state unavailable"
                );
                last_read_error_log = Some(std::time::Instant::now());
            }
        }

        if welder_gas_on == clearcore_requested_on {
            continue;
        }

        match clearcore_registers
            .write_coil(
                plc_register_definitions::ENABLE_GAS_OUTPUT.address.address,
                welder_gas_on,
            )
            .await
        {
            Ok(()) => {
                info_targeted!(
                    MODBUS,
                    "Gas bridge write: set clearcore requested gas to {} (was {}, welder gas output {})",
                    welder_gas_on,
                    clearcore_requested_on,
                    welder_gas_on
                );
            }
            Err(err) => {
                if clearcore_connected {
                    warn_targeted!(
                        MODBUS,
                        "Gas bridge write failed (clearcore requested gas {}): {:?}",
                        welder_gas_on,
                        err
                    );
                }
            }
        }
    }
}
