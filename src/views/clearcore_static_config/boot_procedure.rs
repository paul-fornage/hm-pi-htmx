use crate::error::HmPiError;
use crate::modbus::cached_modbus::CachedModbus;
use crate::views::clearcore_static_config::config_data::ClearcoreConfig;
use crate::{error_targeted, info_targeted, warn_targeted};
use crate::plc::plc_register_definitions::CONFIG_READY;

impl ClearcoreConfig {
    pub async fn on_boot(clearcore_registers: &CachedModbus) -> Result<bool, HmPiError> {
        let clearcore_config = ClearcoreConfig::load_config().await?;
        match clearcore_config {
            Some(config) => {
                info_targeted!(FS, "Clearcore config found, loading...");

                let cfg_ready = clearcore_registers.read_coil(CONFIG_READY.address.address)
                    .await.ok_or(HmPiError::ReadUnpopulatedRegister(CONFIG_READY.address))?;

                if cfg_ready {
                    warn_targeted!(FS, "Clearcore config already loaded");
                    let diff = config.modbus_diff(clearcore_registers).await?;
                    if !diff.is_empty() {
                        warn_targeted!(FS, "Clearcore config differs from disk. Loading config failed");
                        return Ok(false);
                    }

                    Ok(true)
                } else {
                    info_targeted!(FS, "Clearcore config not loaded yet, loading...");

                    config.write_to_modbus(&clearcore_registers).await?;

                    clearcore_registers.write_coil(CONFIG_READY.address.address, true).await?;
                    info_targeted!(FS, "Clearcore config loaded successfully");
                    Ok(true)
                }
            }
            None => {
                warn_targeted!(FS, "Clearcore config not found, first time boot?");
                Ok(false)
            }
        }
    }
    
    
}