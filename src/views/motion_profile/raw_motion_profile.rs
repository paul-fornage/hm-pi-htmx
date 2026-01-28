use serde::{Deserialize, Serialize};

use crate::error::HmPiError;
use crate::modbus::cached_modbus::CachedModbus;
use crate::modbus::{MbDiffStub, ModbusValue, RegisterMetadata};
use crate::plc::plc_register_definitions;
use crate::warn_targeted;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawMotionProfile {
    pub cycle_start_pos: u16,
    pub cycle_end_pos: u16,
    pub cycle_park_pos: u16,
    pub cycle_weld_speed: u16,
    pub cycle_reposition_speed: u16,
    pub cycle_wire_feed_speed: u16,
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

        let cycle_start_pos = pull_hreg_from_mb!(CYCLE_START_POS);
        let cycle_end_pos = pull_hreg_from_mb!(CYCLE_END_POS);
        let cycle_park_pos = pull_hreg_from_mb!(CYCLE_PARK_POS);
        let cycle_weld_speed = pull_hreg_from_mb!(CYCLE_WELD_SPEED);
        let cycle_reposition_speed = pull_hreg_from_mb!(CYCLE_REPOSITION_SPEED);
        let cycle_wire_feed_speed = pull_hreg_from_mb!(CYCLE_WIRE_FEED_SPEED);

        Ok(Self {
            cycle_start_pos,
            cycle_end_pos,
            cycle_park_pos,
            cycle_weld_speed,
            cycle_reposition_speed,
            cycle_wire_feed_speed,
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

        diff_hreg!(cycle_start_pos, CYCLE_START_POS);
        diff_hreg!(cycle_end_pos, CYCLE_END_POS);
        diff_hreg!(cycle_park_pos, CYCLE_PARK_POS);
        diff_hreg!(cycle_weld_speed, CYCLE_WELD_SPEED);
        diff_hreg!(cycle_reposition_speed, CYCLE_REPOSITION_SPEED);
        diff_hreg!(cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);

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

        write_hreg_to_mb!(self.cycle_start_pos, CYCLE_START_POS);
        write_hreg_to_mb!(self.cycle_end_pos, CYCLE_END_POS);
        write_hreg_to_mb!(self.cycle_park_pos, CYCLE_PARK_POS);
        write_hreg_to_mb!(self.cycle_weld_speed, CYCLE_WELD_SPEED);
        write_hreg_to_mb!(self.cycle_reposition_speed, CYCLE_REPOSITION_SPEED);
        write_hreg_to_mb!(self.cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);

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

        write_hreg_to_mb!(self.cycle_start_pos, CYCLE_START_POS);
        write_hreg_to_mb!(self.cycle_end_pos, CYCLE_END_POS);
        write_hreg_to_mb!(self.cycle_park_pos, CYCLE_PARK_POS);
        write_hreg_to_mb!(self.cycle_weld_speed, CYCLE_WELD_SPEED);
        write_hreg_to_mb!(self.cycle_reposition_speed, CYCLE_REPOSITION_SPEED);
        write_hreg_to_mb!(self.cycle_wire_feed_speed, CYCLE_WIRE_FEED_SPEED);

        Ok(())
    }
}