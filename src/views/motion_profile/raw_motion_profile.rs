use serde::{Deserialize, Serialize};
use crate::modbus::cached_modbus::CachedModbus;
use crate::plc::plc_register_definitions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMotionProfile {
    pub weld_enable: bool,
    pub uses_y_axis: bool,
    pub uses_z_axis: bool,
    pub uses_w_axis: bool,
    pub cycle_start_pos: u16,
    pub cycle_end_pos: u16,
    pub cycle_park_pos: u16,
    pub cycle_weld_speed: u16,
    pub cycle_reposition_speed: u16,
    pub cycle_wire_feed_speed: u16,
    pub axis_z_homing_offset: u16,
    pub axis_z_homing_speed: u16,
}

impl RawMotionProfile {
    pub async fn capture_from_memory(clearcore_regs: &CachedModbus) -> Result<Self, String> {
        macro_rules! pull_coil_from_mb {
            ($reg:ident) => {
                clearcore_regs
                    .read_coil(plc_register_definitions::$reg.address.address)
                    .await
                    .ok_or(concat!("Failed to read ", stringify!($reg)))?
            };
        }

        macro_rules! pull_hreg_from_mb {
            ($reg:ident) => {
                clearcore_regs
                    .read_hreg(plc_register_definitions::$reg.address.address)
                    .await
                    .ok_or(concat!("Failed to read ", stringify!($reg)))?
            };
        }

        let weld_enable = pull_coil_from_mb!(WELD_ENABLE);
        let uses_y_axis = pull_coil_from_mb!(USES_Y_AXIS);
        let uses_z_axis = pull_coil_from_mb!(USES_Z_AXIS);
        let uses_w_axis = pull_coil_from_mb!(USES_W_AXIS);

        let cycle_start_pos = pull_hreg_from_mb!(CYCLE_START_POS);
        let cycle_end_pos = pull_hreg_from_mb!(CYCLE_END_POS);
        let cycle_park_pos = pull_hreg_from_mb!(CYCLE_PARK_POS);
        let cycle_weld_speed = pull_hreg_from_mb!(CYCLE_WELD_SPEED);
        let cycle_reposition_speed = pull_hreg_from_mb!(CYCLE_REPOSITION_SPEED);
        let cycle_wire_feed_speed = pull_hreg_from_mb!(CYCLE_WIRE_FEED_SPEED);
        let axis_z_homing_offset = pull_hreg_from_mb!(AXIS_Z_HOMING_OFFSET);
        let axis_z_homing_speed = pull_hreg_from_mb!(AXIS_Z_HOMING_SPEED);

        Ok(RawMotionProfile {
            weld_enable,
            uses_y_axis,
            uses_z_axis,
            uses_w_axis,
            cycle_start_pos,
            cycle_end_pos,
            cycle_park_pos,
            cycle_weld_speed,
            cycle_reposition_speed,
            cycle_wire_feed_speed,
            axis_z_homing_offset,
            axis_z_homing_speed,
        })
    }

    pub async fn apply_to_memory(&self, clearcore_regs: &CachedModbus) -> Result<(), String> {
        macro_rules! write_coil_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .write_coil(plc_register_definitions::$reg.address.address, $val)
                    .await
                    .map_err(|e| {
                        format!(concat!("Failed to save ", stringify!($reg), ": {:?}"), e)
                    })?
            };
        }

        macro_rules! write_hreg_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .write_hreg(plc_register_definitions::$reg.address.address, $val)
                    .await
                    .map_err(|e| {
                        format!(concat!("Failed to save ", stringify!($reg), ": {:?}"), e)
                    })?
            };
        }

        write_hreg_to_mb!(self.cycle_start_pos, CYCLE_START_POS);
        write_hreg_to_mb!(self.cycle_end_pos, CYCLE_END_POS);
        write_hreg_to_mb!(self.cycle_park_pos, CYCLE_PARK_POS);
        write_hreg_to_mb!(self.cycle_weld_speed, CYCLE_WELD_SPEED);
        write_hreg_to_mb!(self.cycle_reposition_speed, CYCLE_REPOSITION_SPEED);
        write_hreg_to_mb!(self.cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);
        write_hreg_to_mb!(self.axis_z_homing_offset, AXIS_Z_HOMING_OFFSET);
        write_hreg_to_mb!(self.axis_z_homing_speed, AXIS_Z_HOMING_SPEED);

        write_coil_to_mb!(self.weld_enable, WELD_ENABLE);
        write_coil_to_mb!(self.uses_y_axis, USES_Y_AXIS);
        write_coil_to_mb!(self.uses_z_axis, USES_Z_AXIS);
        write_coil_to_mb!(self.uses_w_axis, USES_W_AXIS);

        Ok(())
    }
}
