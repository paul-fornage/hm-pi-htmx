use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{ModbusValue, RegisterAddress};


pub struct WatchedRegister {
    pub address: &'static RegisterAddress,
    pub handler: Box<dyn Fn(Option<ModbusValue>) + Send + Sync>,
}


pub async fn cc_mb_watcher_task(clearcore_registers: &CachedModbus, registers: Vec<WatchedRegister>) -> ! {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(10));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    
    let mut reg_buffer: Vec<Option<ModbusValue>> = vec![None; registers.len()];
    loop{
        interval.tick().await;
        for (i, reg) in registers.iter().enumerate() {
            let new_value = clearcore_registers.read(reg.address).await;
            if reg_buffer[i] != new_value {
                reg_buffer[i] = new_value.clone();
                (reg.handler)(new_value);
            }
        }
    }
}