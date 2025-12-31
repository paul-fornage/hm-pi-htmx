pub mod arc_start;
mod sub_types;

use arc_start::ArcStartProfile;
use sub_types::*;

pub struct MillerWeldProfile {
    pub use_dc_output: bool,
    pub use_ep_polarity: bool,
    pub boost_en: bool,
    pub droop_en: bool,
    pub use_low_ocv: bool,
    pub pulser_en: bool,
    pub use_low_ac_commutation_amp: bool,
    pub ac_independent_en: bool,
    pub arc_start_profile: ArcStartProfile,
    pub ac_en_wave_shape: WaveShape,
    pub ac_ep_wave_shape: WaveShape,
    pub hot_start_time: HotStartProfile,
    pub ac_en_amperage: AcAmperageProfile,
    pub ac_ep_amperage: AcAmperageProfile,
    pub ac_balance: AcBalance,
    pub ac_frequency: AcFrequency,
    pub weld_amperage: AcAmperageProfile,
    pub pulser_pps: PulseFrequency,
    pub pulser_peak_time: PulserPeakTime,
    pub preflow_time: WavePhaseDuration<250>,
    pub initial_amperage: AcAmperageProfile,
    pub initial_time: WavePhaseDuration<250>,
    pub initial_slope_time: WavePhaseDuration<500>,
    pub main_time: WavePhaseDuration<9990>,
    pub final_slope_time: WavePhaseDuration<500>,
    pub final_amperage: AcAmperageProfile,
    pub final_time: WavePhaseDuration<250>,
    pub postflow_time: PostFlowTime,
    pub hot_wire_voltage: HotWireVoltage,
}




