use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::miller::miller_memory::MillerMemory;
use crate::miller::miller_register_definitions;

/// Raw welding profile containing all register values as they are stored in modbus memory.
/// All values are u16 or bool - no interpretation or conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawWeldProfile {
    // Boolean registers
    pub use_dc_output: bool,
    pub use_ep_polarity: bool,
    pub boost_en: bool,
    pub droop_en: bool,
    pub use_low_ocv: bool,
    pub pulser_en: bool,
    pub use_low_ac_commutation_amp: bool,
    pub ac_independant_en: bool,

    // Enum registers (stored as raw u16)
    pub tungsten_preset: u16,
    pub arc_start_polarity_phase: u16,
    pub ac_en_wave_shape: u16,
    pub ac_ep_wave_shape: u16,

    // Analog registers (stored as raw u16)
    pub preset_min_amperage: u16,
    pub arc_start_amperage: u16,
    pub arc_start_time: u16,
    pub arc_start_slope_time: u16,
    pub arc_start_ac_time: u16,
    pub hot_start_time: u16,
    pub ac_en_amperage: u16,
    pub ac_ep_amperage: u16,
    pub ac_balance: u16,
    pub ac_frequency: u16,
    pub weld_amperage: u16,
    pub pulser_pps: u16,
    pub pulser_peak_time: u16,
    pub preflow_time: u16,
    pub initial_amperage: u16,
    pub initial_time: u16,
    pub initial_slope_time: u16,
    pub main_time: u16,
    pub final_slope_time: u16,
    pub final_amperage: u16,
    pub final_time: u16,
    pub hot_wire_voltage: u16,

    // Special case register
    pub postflow_time: u16,
}

impl RawWeldProfile {
    /// Captures the current welding profile from the Miller memory.
    /// Returns an error if any register cannot be read.
    pub async fn capture_from_memory(miller_regs: &MillerMemory) -> Result<Self, String> {

        macro_rules! pull_coil_from_mb {
                ($reg:ident) => {
                    miller_regs
                        .read_coil(miller_register_definitions::$reg.address.address)
                        .await
                        .ok_or(concat!("Failed to read ", stringify!($reg)))?
                };
            }

        macro_rules! pull_hreg_from_mb {
                ($reg:ident) => {
                    miller_regs
                        .read_hreg(miller_register_definitions::$reg.address.address)
                        .await
                        .ok_or(concat!("Failed to read ", stringify!($reg)))?
                };
            }

        // Read all boolean registers
        let use_dc_output = pull_coil_from_mb!(USE_DC_OUTPUT);
        let use_ep_polarity = pull_coil_from_mb!(USE_EP_POLARITY);
        let boost_en = pull_coil_from_mb!(BOOST_EN);
        let droop_en = pull_coil_from_mb!(DROOP_EN);
        let use_low_ocv = pull_coil_from_mb!(USE_LOW_OCV);
        let pulser_en = pull_coil_from_mb!(PULSER_EN);
        let use_low_ac_commutation_amp = pull_coil_from_mb!(USE_LOW_AC_COMMUTATION_AMP);
        let ac_independant_en = pull_coil_from_mb!(AC_INDEPENDANT_EN);

        // Read enum registers
        let tungsten_preset = pull_hreg_from_mb!(TUNGSTEN_PRESET);
        let arc_start_polarity_phase = pull_hreg_from_mb!(ARC_START_POLARITY_PHASE);
        let ac_en_wave_shape = pull_hreg_from_mb!(AC_EN_WAVE_SHAPE);
        let ac_ep_wave_shape = pull_hreg_from_mb!(AC_EP_WAVE_SHAPE);

        // Read analog registers
        let preset_min_amperage = pull_hreg_from_mb!(PRESET_MIN_AMPERAGE);
        let arc_start_amperage = pull_hreg_from_mb!(ARC_START_AMPERAGE);
        let arc_start_time = pull_hreg_from_mb!(ARC_START_TIME);
        let arc_start_slope_time = pull_hreg_from_mb!(ARC_START_SLOPE_TIME);
        let arc_start_ac_time = pull_hreg_from_mb!(ARC_START_AC_TIME);
        let hot_start_time = pull_hreg_from_mb!(HOT_START_TIME);
        let ac_en_amperage = pull_hreg_from_mb!(AC_EN_AMPERAGE);
        let ac_ep_amperage = pull_hreg_from_mb!(AC_EP_AMPERAGE);
        let ac_balance = pull_hreg_from_mb!(AC_BALANCE);
        let ac_frequency = pull_hreg_from_mb!(AC_FREQUENCY);
        let weld_amperage = pull_hreg_from_mb!(WELD_AMPERAGE);
        let pulser_pps = pull_hreg_from_mb!(PULSER_PPS);
        let pulser_peak_time = pull_hreg_from_mb!(PULSER_PEAK_TIME);
        let preflow_time = pull_hreg_from_mb!(PREFLOW_TIME);
        let initial_amperage = pull_hreg_from_mb!(INITIAL_AMPERAGE);
        let initial_time = pull_hreg_from_mb!(INITIAL_TIME);
        let initial_slope_time = pull_hreg_from_mb!(INITIAL_SLOPE_TIME);
        let main_time = pull_hreg_from_mb!(MAIN_TIME);
        let final_slope_time = pull_hreg_from_mb!(FINAL_SLOPE_TIME);
        let final_amperage = pull_hreg_from_mb!(FINAL_AMPERAGE);
        let final_time = pull_hreg_from_mb!(FINAL_TIME);
        let hot_wire_voltage = pull_hreg_from_mb!(HOT_WIRE_VOLTAGE);

        // Read postflow time
        let postflow_time = pull_hreg_from_mb!(POSTFLOW_TIME);

        Ok(RawWeldProfile {
            use_dc_output,
            use_ep_polarity,
            boost_en,
            droop_en,
            use_low_ocv,
            pulser_en,
            use_low_ac_commutation_amp,
            ac_independant_en,
            tungsten_preset,
            arc_start_polarity_phase,
            ac_en_wave_shape,
            ac_ep_wave_shape,
            preset_min_amperage,
            arc_start_amperage,
            arc_start_time,
            arc_start_slope_time,
            arc_start_ac_time,
            hot_start_time,
            ac_en_amperage,
            ac_ep_amperage,
            ac_balance,
            ac_frequency,
            weld_amperage,
            pulser_pps,
            pulser_peak_time,
            preflow_time,
            initial_amperage,
            initial_time,
            initial_slope_time,
            main_time,
            final_slope_time,
            final_amperage,
            final_time,
            hot_wire_voltage,
            postflow_time,
        })
    }

    /// Applies this welding profile to the Miller memory.
    /// Returns an error on first write failure.
    pub async fn apply_to_memory(&self, miller_regs: &MillerMemory) -> Result<(), String> {

        macro_rules! write_coil_to_mb {
            ($val:expr, $reg:ident) => {
                miller_regs
                    .write_coil(miller_register_definitions::$reg.address.address, $val)
                    .await
                    .map_err(|e| {
                        format!(concat!("Failed to save ", stringify!($reg), ": {:?}"), e)
                    })?
            };
        }

        macro_rules! write_hreg_to_mb {
            ($val:expr, $reg:ident) => {
                miller_regs
                    .write_hreg(miller_register_definitions::$reg.address.address, $val)
                    .await
                    .map_err(|e| {
                        format!(concat!("Failed to save ", stringify!($reg), ": {:?}"), e)
                    })?
            };
        }

        // Write enum registers
        write_hreg_to_mb!(self.tungsten_preset, TUNGSTEN_PRESET);
        write_hreg_to_mb!(self.arc_start_polarity_phase, ARC_START_POLARITY_PHASE);
        write_hreg_to_mb!(self.ac_en_wave_shape, AC_EN_WAVE_SHAPE);
        write_hreg_to_mb!(self.ac_ep_wave_shape, AC_EP_WAVE_SHAPE);

        // Write analog registers
        write_hreg_to_mb!(self.preset_min_amperage, PRESET_MIN_AMPERAGE);
        write_hreg_to_mb!(self.arc_start_amperage, ARC_START_AMPERAGE);
        write_hreg_to_mb!(self.arc_start_time, ARC_START_TIME);
        write_hreg_to_mb!(self.arc_start_slope_time, ARC_START_SLOPE_TIME);
        write_hreg_to_mb!(self.arc_start_ac_time, ARC_START_AC_TIME);
        write_hreg_to_mb!(self.hot_start_time, HOT_START_TIME);
        write_hreg_to_mb!(self.ac_en_amperage, AC_EN_AMPERAGE);
        write_hreg_to_mb!(self.ac_ep_amperage, AC_EP_AMPERAGE);
        write_hreg_to_mb!(self.ac_balance, AC_BALANCE);
        write_hreg_to_mb!(self.ac_frequency, AC_FREQUENCY);
        write_hreg_to_mb!(self.weld_amperage, WELD_AMPERAGE);
        write_hreg_to_mb!(self.pulser_pps, PULSER_PPS);
        write_hreg_to_mb!(self.pulser_peak_time, PULSER_PEAK_TIME);
        write_hreg_to_mb!(self.preflow_time, PREFLOW_TIME);
        write_hreg_to_mb!(self.initial_amperage, INITIAL_AMPERAGE);
        write_hreg_to_mb!(self.initial_time, INITIAL_TIME);
        write_hreg_to_mb!(self.initial_slope_time, INITIAL_SLOPE_TIME);
        write_hreg_to_mb!(self.main_time, MAIN_TIME);
        write_hreg_to_mb!(self.final_slope_time, FINAL_SLOPE_TIME);
        write_hreg_to_mb!(self.final_amperage, FINAL_AMPERAGE);
        write_hreg_to_mb!(self.final_time, FINAL_TIME);
        write_hreg_to_mb!(self.hot_wire_voltage, HOT_WIRE_VOLTAGE);

        // Write postflow time
        write_hreg_to_mb!(self.postflow_time, POSTFLOW_TIME);

        // Write all boolean registers
        write_coil_to_mb!(self.use_dc_output, USE_DC_OUTPUT);
        write_coil_to_mb!(self.use_ep_polarity, USE_EP_POLARITY);
        write_coil_to_mb!(self.boost_en, BOOST_EN);
        write_coil_to_mb!(self.droop_en, DROOP_EN);
        write_coil_to_mb!(self.use_low_ocv, USE_LOW_OCV);
        write_coil_to_mb!(self.pulser_en, PULSER_EN);
        write_coil_to_mb!(self.use_low_ac_commutation_amp, USE_LOW_AC_COMMUTATION_AMP);
        write_coil_to_mb!(self.ac_independant_en, AC_INDEPENDANT_EN);

        Ok(())
    }
}