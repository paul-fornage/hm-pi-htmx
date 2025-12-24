



// State:
//  0 Initial Amperage
//  1 Initial Slope Time
//  2 Main Amperage
//  3 Final Slope Time
//  4 Final Amperage
//  5 Preflow
//  6 Standby
//  7 Output Shorted
//  8 Release Trigger
//  9 Output Disabled
//  13 Error
//  14 Power Down
//  15 Power Up
struct WeldState{

}



pub struct MillerStats {
    pub ps_ui_disable: bool,
    pub rmt_trigger_disable: bool,
    pub gas_en: bool,
    pub cooler_en: bool,
    pub cooler_tig_en: bool,
    pub cooler_error_en: bool,
    pub touch_sense_en: bool,
    pub rms_en: bool,
    pub cooler_detected: bool,
    pub cooler_load_detected: bool,
    pub foot_control_detected: bool,
    pub rmt_trigger_enabled: bool,
    pub contactor_output_enabled: bool,
    pub gas_output_enabled: bool,
    pub is_valid_arc: bool,
    pub arc_length_ctl_lockout: bool,
    pub touch_sense_detect: bool,
    pub is_ce_model: bool,
    pub is_str_model: bool,
    pub is_dx_model: bool,
    pub rms_hw_present: bool,
    pub low_live_input: bool,
    pub hot_start_supported: bool,
    pub ac_independant_supported: bool,
    pub is_mig_volt_sense_model: bool,
    pub is_syncrowave_model: bool,
    pub non_cooler_supply_detect: bool,

    pub max_amps: u16, // 4032
    pub min_dc_amps: u16, // 4033
    pub min_ac_amps: u16, // 4034
    pub commanded_output_amperage: u16, // 4200
    pub output_amperage: u16, // 4201
    pub output_voltage: u16, // 4202
    pub output_current_dc_pulse_peak: u16, // 4203
    pub output_voltage_dc_pulse_peak: u16, // 4204
    pub output_current_dc_pulse_back: u16, // 4205
    pub output_voltage_dc_pulse_back: u16, // 4206
    pub fan_output: u16, // 4300
    pub temperature_1: u16, // 4301
    pub temperature_2: u16, // 4302
    pub temperature_3: u16, // 4303
    pub temperature_4: u16, // 4304
    pub temperature_5: u16, // 4305
    pub temperature_6: u16, // 4306
    pub temperature_7: u16, // 4307
    pub primary_line_current: u16, // 4400
    pub primary_line_voltage: u16, // 4401
    pub primary_line_voltage_peak: u16, // 4402
    pub primary_bus_voltage: u16, // 4403
    pub cooler_output_voltage: u16, // 4404
    pub cooler_output_current: u16, // 4405
    pub cooler_bus_voltage: u16, // 4406
    pub primary_2_line_voltage_peak: u16, // 4407
    pub primary_2_bus_voltage_peak: u16, // 4408

    pub weld_state: u16, // 4100
    pub weld_process: u16, // 6201

    pub error_reg_1: u16,
    pub error_reg_2: u16,
    pub error_reg_3: u16,

    pub app_software_version_pcb_1: u16,
    pub app_software_version_pcb_2: u16,
    pub app_software_version_pcb_3: u16,
    pub app_software_version_pcb_4: u16,
    pub app_software_version_pcb_5: u16,
    pub app_software_version_pcb_6: u16,
    pub app_software_version_pcb_7: u16,
}
