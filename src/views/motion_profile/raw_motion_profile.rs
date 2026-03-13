use serde::{Deserialize, Serialize};

use crate::error::HmPiError;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::MbDiffStub;
use crate::plc::plc_register_definitions;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RawMotionProfile {
    pub cycle_start_pos: u16,
    pub cycle_end_pos: u16,
    pub cycle_park_pos: u16,
    pub cycle_weld_speed: u16,
    pub cycle_reposition_speed_x: u16,
    pub cycle_reposition_speed_y: u16,
    pub cycle_reposition_speed_z: u16,
    pub cycle_wire_feed_speed: u16,
    pub cycle_avc_vref: u16,
    pub cycle_avc_correction_strength_multiplier: u16,
    pub cycle_avc_travel_speed_z: u16,
    pub cycle_axis_z_torch_up_offset: u16,
    pub cycle_z_static_offset: u16,
    pub cycle_touch_retract_reposition_distance: u16,
    pub cycle_touch_retract_probe_speed: u16,
    pub cycle_touch_retract_final_height: u16,
    pub cycle_use_avc: bool,
    pub cycle_use_touch_retract: bool,
}

impl Default for RawMotionProfile {
    fn default() -> Self {
        Self {
            cycle_start_pos: 0,
            cycle_end_pos: 0,
            cycle_park_pos: 0,
            cycle_weld_speed: 100,
            cycle_reposition_speed_x: 100,
            cycle_reposition_speed_y: 100,
            cycle_reposition_speed_z: 100,
            cycle_wire_feed_speed: 100,
            cycle_avc_vref: 1000,
            cycle_avc_correction_strength_multiplier: 200,
            cycle_avc_travel_speed_z: 100,
            cycle_axis_z_torch_up_offset: 100,
            cycle_z_static_offset: 200,
            cycle_touch_retract_reposition_distance: 100,
            cycle_touch_retract_probe_speed: 100,
            cycle_touch_retract_final_height: 25,
            cycle_use_avc: false,
            cycle_use_touch_retract: false,
        }
    }
}

impl RawMotionProfile {
    pub async fn capture_from_memory(clearcore_regs: &CachedModbus) -> Result<Self, HmPiError> {
        macro_rules! pull_hreg_from_mb {
            ($reg:ident) => {
                clearcore_regs
                    .read_hreg(plc_register_definitions::$reg.address.address)
                    .await
                    .ok_or(HmPiError::MissingExpectedRegister(
                        plc_register_definitions::$reg.address,
                        plc_register_definitions::$reg.name.to_string(),
                    ))?
            };
        }

        macro_rules! pull_coil_from_mb {
            ($reg:ident) => {
                clearcore_regs
                    .read_coil(plc_register_definitions::$reg.address.address)
                    .await
                    .ok_or(HmPiError::MissingExpectedRegister(
                        plc_register_definitions::$reg.address,
                        plc_register_definitions::$reg.name.to_string(),
                    ))?
            };
        }

        let cycle_start_pos = pull_hreg_from_mb!(CYCLE_START_POS);
        let cycle_end_pos = pull_hreg_from_mb!(CYCLE_END_POS);
        let cycle_park_pos = pull_hreg_from_mb!(CYCLE_PARK_POS);
        let cycle_weld_speed = pull_hreg_from_mb!(CYCLE_WELD_SPEED);
        let cycle_reposition_speed_x = pull_hreg_from_mb!(CYCLE_REPOSITION_SPEED_X);
        let cycle_reposition_speed_y = pull_hreg_from_mb!(CYCLE_REPOSITION_SPEED_Y);
        let cycle_reposition_speed_z = pull_hreg_from_mb!(CYCLE_REPOSITION_SPEED_Z);
        let cycle_wire_feed_speed = pull_hreg_from_mb!(CYCLE_WIRE_FEED_SPEED);
        let cycle_avc_vref = pull_hreg_from_mb!(CYCLE_AVC_VREF);
        let cycle_avc_correction_strength_multiplier = pull_hreg_from_mb!(CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER);
        let cycle_avc_travel_speed_z = pull_hreg_from_mb!(CYCLE_AVC_TRAVEL_SPEED_Z);
        let cycle_axis_z_torch_up_offset = pull_hreg_from_mb!(CYCLE_AXIS_Z_TORCH_UP_OFFSET);
        let cycle_z_static_offset = pull_hreg_from_mb!(CYCLE_Z_STATIC_OFFSET);
        let cycle_touch_retract_reposition_distance = pull_hreg_from_mb!(CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE);
        let cycle_touch_retract_probe_speed = pull_hreg_from_mb!(CYCLE_TOUCH_RETRACT_PROBE_SPEED);
        let cycle_touch_retract_final_height = pull_hreg_from_mb!(CYCLE_TOUCH_RETRACT_FINAL_HEIGHT);
        let cycle_use_avc = pull_coil_from_mb!(CYCLE_USE_AVC);
        let cycle_use_touch_retract = pull_coil_from_mb!(CYCLE_USE_TOUCH_RETRACT);

        Ok(Self {
            cycle_start_pos,
            cycle_end_pos,
            cycle_park_pos,
            cycle_weld_speed,
            cycle_reposition_speed_x,
            cycle_reposition_speed_y,
            cycle_reposition_speed_z,
            cycle_wire_feed_speed,
            cycle_avc_vref,
            cycle_avc_correction_strength_multiplier,
            cycle_avc_travel_speed_z,
            cycle_axis_z_torch_up_offset,
            cycle_z_static_offset,
            cycle_touch_retract_reposition_distance,
            cycle_touch_retract_probe_speed,
            cycle_touch_retract_final_height,
            cycle_use_avc,
            cycle_use_touch_retract,
        })
    }

    pub async fn modbus_diff(&self, clearcore_regs: &CachedModbus) -> Vec<MbDiffStub> {
        let mut diff = Vec::new();

        macro_rules! diff_hreg {
            ($field:ident, $reg:ident) => {
                match MbDiffStub::check_word(
                    clearcore_regs,
                    &plc_register_definitions::$reg,
                    self.$field,
                ).await {
                    Some(diff_stub) => diff.push(diff_stub),
                    None => {}
                }
            };
        }

        macro_rules! diff_coil {
            ($field:ident, $reg:ident) => {
                match MbDiffStub::check_bool(
                    clearcore_regs,
                    &plc_register_definitions::$reg,
                    self.$field,
                ).await {
                    Some(diff_stub) => diff.push(diff_stub),
                    None => {}
                }
            };
        }

        diff_hreg!(cycle_start_pos, CYCLE_START_POS);
        diff_hreg!(cycle_end_pos, CYCLE_END_POS);
        diff_hreg!(cycle_park_pos, CYCLE_PARK_POS);
        diff_hreg!(cycle_weld_speed, CYCLE_WELD_SPEED);
        diff_hreg!(cycle_reposition_speed_x, CYCLE_REPOSITION_SPEED_X);
        diff_hreg!(cycle_reposition_speed_y, CYCLE_REPOSITION_SPEED_Y);
        diff_hreg!(cycle_reposition_speed_z, CYCLE_REPOSITION_SPEED_Z);
        diff_hreg!(cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);
        diff_hreg!(cycle_avc_vref, CYCLE_AVC_VREF);
        diff_hreg!(cycle_avc_correction_strength_multiplier, CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER);
        diff_hreg!(cycle_avc_travel_speed_z, CYCLE_AVC_TRAVEL_SPEED_Z);
        diff_hreg!(cycle_axis_z_torch_up_offset, CYCLE_AXIS_Z_TORCH_UP_OFFSET);
        diff_hreg!(cycle_z_static_offset, CYCLE_Z_STATIC_OFFSET);
        diff_hreg!(cycle_touch_retract_reposition_distance, CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE);
        diff_hreg!(cycle_touch_retract_probe_speed, CYCLE_TOUCH_RETRACT_PROBE_SPEED);
        diff_hreg!(cycle_touch_retract_final_height, CYCLE_TOUCH_RETRACT_FINAL_HEIGHT);
        diff_coil!(cycle_use_avc, CYCLE_USE_AVC);
        diff_coil!(cycle_use_touch_retract, CYCLE_USE_TOUCH_RETRACT);

        diff
    }

    pub async fn apply_to_memory(&self, clearcore_regs: &CachedModbus) -> Result<(), HmPiError> {
        macro_rules! write_hreg_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .write_hreg(plc_register_definitions::$reg.address.address, $val)
                    .await?
            };
        }

        macro_rules! write_coil_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .write_coil(plc_register_definitions::$reg.address.address, $val)
                    .await?
            };
        }

        write_hreg_to_mb!(self.cycle_start_pos, CYCLE_START_POS);
        write_hreg_to_mb!(self.cycle_end_pos, CYCLE_END_POS);
        write_hreg_to_mb!(self.cycle_park_pos, CYCLE_PARK_POS);
        write_hreg_to_mb!(self.cycle_weld_speed, CYCLE_WELD_SPEED);
        write_hreg_to_mb!(self.cycle_reposition_speed_x, CYCLE_REPOSITION_SPEED_X);
        write_hreg_to_mb!(self.cycle_reposition_speed_y, CYCLE_REPOSITION_SPEED_Y);
        write_hreg_to_mb!(self.cycle_reposition_speed_z, CYCLE_REPOSITION_SPEED_Z);
        write_hreg_to_mb!(self.cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);
        write_hreg_to_mb!(self.cycle_avc_vref, CYCLE_AVC_VREF);
        write_hreg_to_mb!(self.cycle_avc_correction_strength_multiplier, CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER);
        write_hreg_to_mb!(self.cycle_avc_travel_speed_z, CYCLE_AVC_TRAVEL_SPEED_Z);
        write_hreg_to_mb!(self.cycle_axis_z_torch_up_offset, CYCLE_AXIS_Z_TORCH_UP_OFFSET);
        write_hreg_to_mb!(self.cycle_z_static_offset, CYCLE_Z_STATIC_OFFSET);
        write_hreg_to_mb!(self.cycle_touch_retract_reposition_distance, CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE);
        write_hreg_to_mb!(self.cycle_touch_retract_probe_speed, CYCLE_TOUCH_RETRACT_PROBE_SPEED);
        write_hreg_to_mb!(self.cycle_touch_retract_final_height, CYCLE_TOUCH_RETRACT_FINAL_HEIGHT);
        write_coil_to_mb!(self.cycle_use_avc, CYCLE_USE_AVC);
        write_coil_to_mb!(self.cycle_use_touch_retract, CYCLE_USE_TOUCH_RETRACT);

        Ok(())
    }

    pub async fn apply_diff(
        clearcore_regs: &CachedModbus,
        diffs: Vec<MbDiffStub>,
    ) -> Result<(), HmPiError> {
        for diff in diffs {
            diff.apply(clearcore_regs).await?;
        }
        Ok(())
    }

    pub async fn apply_to_memory_diff(&self, clearcore_regs: &CachedModbus) -> Result<(), HmPiError> {
        macro_rules! write_hreg_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .diff_write_hreg(plc_register_definitions::$reg.address.address, $val)
                    .await?
            };
        }

        macro_rules! write_coil_to_mb {
            ($val:expr, $reg:ident) => {
                clearcore_regs
                    .diff_write_coil(plc_register_definitions::$reg.address.address, $val)
                    .await?
            };
        }

        write_hreg_to_mb!(self.cycle_start_pos, CYCLE_START_POS);
        write_hreg_to_mb!(self.cycle_end_pos, CYCLE_END_POS);
        write_hreg_to_mb!(self.cycle_park_pos, CYCLE_PARK_POS);
        write_hreg_to_mb!(self.cycle_weld_speed, CYCLE_WELD_SPEED);
        write_hreg_to_mb!(self.cycle_reposition_speed_x, CYCLE_REPOSITION_SPEED_X);
        write_hreg_to_mb!(self.cycle_reposition_speed_y, CYCLE_REPOSITION_SPEED_Y);
        write_hreg_to_mb!(self.cycle_reposition_speed_z, CYCLE_REPOSITION_SPEED_Z);
        write_hreg_to_mb!(self.cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);
        write_hreg_to_mb!(self.cycle_avc_vref, CYCLE_AVC_VREF);
        write_hreg_to_mb!(self.cycle_avc_correction_strength_multiplier, CYCLE_AVC_CORRECTION_STRENGTH_MULTIPLIER);
        write_hreg_to_mb!(self.cycle_avc_travel_speed_z, CYCLE_AVC_TRAVEL_SPEED_Z);
        write_hreg_to_mb!(self.cycle_axis_z_torch_up_offset, CYCLE_AXIS_Z_TORCH_UP_OFFSET);
        write_hreg_to_mb!(self.cycle_z_static_offset, CYCLE_Z_STATIC_OFFSET);
        write_hreg_to_mb!(self.cycle_touch_retract_reposition_distance, CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE);
        write_hreg_to_mb!(self.cycle_touch_retract_probe_speed, CYCLE_TOUCH_RETRACT_PROBE_SPEED);
        write_hreg_to_mb!(self.cycle_touch_retract_final_height, CYCLE_TOUCH_RETRACT_FINAL_HEIGHT);
        write_coil_to_mb!(self.cycle_use_avc, CYCLE_USE_AVC);
        write_coil_to_mb!(self.cycle_use_touch_retract, CYCLE_USE_TOUCH_RETRACT);

        Ok(())
    }
}
