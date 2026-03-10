use crate::views::run_cycle::weld_file_ops;
use crate::views::run_cycle::motion_file_ops;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse};
use crate::hx_trigger::HxTrigger;
use crate::views::miller_info::register_view::AnalogRegisterTemplate;
use crate::views::motion_profile::raw_motion_profile::RawMotionProfile;
use crate::views::run_cycle::{profiles_match, RunCycleFeedbackTemplate};
use crate::views::{motion_profile, welder_profile};
use crate::{debug_targeted, error_targeted, warn_targeted, AppState};
use crate::miller::miller_register_definitions;
use crate::plc::plc_register_definitions;
use crate::views::welder_profile::raw_weld_profile::RawWeldProfile;

pub async fn run_cycle_analog_registers(
    State(state): State<AppState>,
) -> axum::response::Response {
    let (selected_weld, selected_motion) = {
        let weld_metadata = state.weld_profile_metadata.lock().await;
        let motion_metadata = state.motion_profile_metadata.lock().await;
        (weld_metadata.name.clone(), motion_metadata.name.clone())
    };

    let Some(weld_name) = selected_weld else {
        return Html(String::new()).into_response();
    };
    let Some(motion_name) = selected_motion else {
        return Html(String::new()).into_response();
    };

    let weld_profile = match weld_file_ops::load_profile(&weld_name).await {
        Ok(profile) => profile,
        Err(err) => {
            error_targeted!(HTTP, "Failed to load selected weld profile '{weld_name}': {err}");
            return RunCycleAnalogRegistersTemplate {
                rows: Vec::new(),
                missing_preview: None,
                error_message: Some(format!("Failed to load weld profile '{weld_name}'.")),
            }.into_response();
        }
    };

    let motion_profile = match motion_file_ops::load_profile(&motion_name).await {
        Ok(profile) => profile,
        Err(err) => {
            error_targeted!(HTTP, "Failed to load selected motion profile '{motion_name}': {err}");
            return RunCycleAnalogRegistersTemplate {
                rows: Vec::new(),
                missing_preview: None,
                error_message: Some(format!("Failed to load motion profile '{motion_name}'.")),
            }.into_response();
        }
    };

    let (weld_matches, motion_matches) = match profiles_match(&state, &weld_profile, &motion_profile).await {
        Ok(result) => result,
        Err(err) => {
            error_targeted!(MODBUS, "Failed to verify loaded profiles: {err}");
            return RunCycleAnalogRegistersTemplate {
                rows: Vec::new(),
                missing_preview: None,
                error_message: Some("Unable to verify loaded profiles.".to_string()),
            }.into_response();
        }
    };

    if !weld_matches || !motion_matches {
        debug_targeted!(MODBUS, "Run cycle presets not loaded yet (weld_loaded={weld_matches}, motion_loaded={motion_matches})");
        return Html(String::new()).into_response();
    }

    let mut rows = Vec::with_capacity(
        motion_profile::MOTION_PROFILE_ANALOG_REGISTERS.len()
            + welder_profile::WELD_PROFILE_ANALOG_REGISTERS.len(),
    );
    let mut missing_names: Vec<&'static str> = Vec::new();

    for info in motion_profile::MOTION_PROFILE_ANALOG_REGISTERS.iter() {
        let raw_value = motion_profile_analog_value(&motion_profile.raw_profile, info);
        if raw_value.is_none() {
            missing_names.push(info.meta.name);
        }
        rows.push(RunCycleAnalogRegisterRow {
            source: "ClearCore",
            register: AnalogRegisterTemplate {
                raw_value,
                register_info: info,
            },
        });
    }

    for info in welder_profile::WELD_PROFILE_ANALOG_REGISTERS.iter() {
        let raw_value = weld_profile_analog_value(&weld_profile.raw_profile, info);
        if raw_value.is_none() {
            missing_names.push(info.meta.name);
        }
        rows.push(RunCycleAnalogRegisterRow {
            source: "Welder",
            register: AnalogRegisterTemplate {
                raw_value,
                register_info: info,
            },
        });
    }

    let missing_preview = if missing_names.is_empty() {
        None
    } else {
        let preview = missing_names
            .iter()
            .take(8)
            .copied()
            .collect::<Vec<_>>()
            .join(", ");
        let suffix = if missing_names.len() > 8 { ", ..." } else { "" };
        Some(format!(
            "Missing preset values for {} registers: {}{}",
            missing_names.len(),
            preview,
            suffix
        ))
    };

    if let Some(preview) = &missing_preview {
        warn_targeted!(HTTP, "Run cycle analog preset mapping incomplete: {}", preview);
    }

    RunCycleAnalogRegistersTemplate {
        rows,
        missing_preview,
        error_message: None,
    }
        .into_response()
}

pub struct RunCycleAnalogRegisterRow {
    pub source: &'static str,
    pub register: AnalogRegisterTemplate,
}

impl RunCycleAnalogRegisterRow {
    pub fn name(&self) -> &'static str {
        self.register.register_info.meta.name
    }

    pub fn unit(&self) -> &'static str {
        self.register.register_info.unit_string()
    }

    pub fn formatted_value(&self) -> String {
        self.register.formatted_value()
    }

    pub fn has_value(&self) -> bool {
        self.register.has_value()
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/run-cycle/analog-registers.html")]
pub struct RunCycleAnalogRegistersTemplate {
    pub rows: Vec<RunCycleAnalogRegisterRow>,
    pub missing_preview: Option<String>,
    pub error_message: Option<String>,
}



pub fn feedback_ok_with_triggers(message: String, triggers: &[HxTrigger]) -> axum::response::Response {
    let mut headers = HeaderMap::new();
    apply_triggers(&mut headers, triggers);
    let html = RunCycleFeedbackTemplate {
        result: Ok(message),
    }
        .render()
        .unwrap();
    (headers, Html(html)).into_response()
}

fn apply_triggers(headers: &mut HeaderMap, triggers: &[HxTrigger]) {
    // TODO: consolidate with usb_transfer::handlers::apply_triggers if we want a shared helper.
    if triggers.is_empty() {
        return;
    }

    let json_map = HxTrigger::list_to_json(triggers);
    if let Ok(json_string) = serde_json::to_string(&json_map) {
        if let Ok(header_value) = HeaderValue::from_str(&json_string) {
            headers.insert("HX-Trigger", header_value);
        }
    }
}

fn motion_profile_analog_value(
    profile: &RawMotionProfile,
    info: &'static crate::views::shared::analog_register::AnalogRegisterInfo,
) -> Option<u16> {
    let meta = info.meta;
    if std::ptr::eq(meta, &plc_register_definitions::CYCLE_START_POS) {
        Some(profile.cycle_start_pos)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_END_POS) {
        Some(profile.cycle_end_pos)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_PARK_POS) {
        Some(profile.cycle_park_pos)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_WELD_SPEED) {
        Some(profile.cycle_weld_speed)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_X) {
        Some(profile.cycle_reposition_speed_x)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_Y) {
        Some(profile.cycle_reposition_speed_y)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_Z) {
        Some(profile.cycle_reposition_speed_z)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_WIRE_FEED_SPEED) {
        Some(profile.cycle_wire_feed_speed)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_AVC_VREF) {
        Some(profile.cycle_avc_vref)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_Z_STATIC_OFFSET) {
        Some(profile.cycle_z_static_offset)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_AXIS_Z_TORCH_UP_OFFSET) {
        Some(profile.cycle_axis_z_torch_up_offset)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE) {
        Some(profile.cycle_touch_retract_reposition_distance)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_PROBE_SPEED) {
        Some(profile.cycle_touch_retract_probe_speed)
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_FINAL_HEIGHT) {
        Some(profile.cycle_touch_retract_final_height)
    } else {
        None
    }
}

fn weld_profile_analog_value(
    profile: &RawWeldProfile,
    info: &'static crate::views::shared::analog_register::AnalogRegisterInfo,
) -> Option<u16> {
    let meta = info.meta;
    if std::ptr::eq(meta, &miller_register_definitions::PRESET_MIN_AMPERAGE) {
        Some(profile.preset_min_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_AMPERAGE) {
        Some(profile.arc_start_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_TIME) {
        Some(profile.arc_start_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_SLOPE_TIME) {
        Some(profile.arc_start_slope_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_AC_TIME) {
        Some(profile.arc_start_ac_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::HOT_START_TIME) {
        Some(profile.hot_start_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_EN_AMPERAGE) {
        Some(profile.ac_en_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_EP_AMPERAGE) {
        Some(profile.ac_ep_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_BALANCE) {
        Some(profile.ac_balance)
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_FREQUENCY) {
        Some(profile.ac_frequency)
    } else if std::ptr::eq(meta, &miller_register_definitions::WELD_AMPERAGE) {
        Some(profile.weld_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::PULSER_PPS) {
        Some(profile.pulser_pps)
    } else if std::ptr::eq(meta, &miller_register_definitions::PULSER_PEAK_TIME) {
        Some(profile.pulser_peak_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::PREFLOW_TIME) {
        Some(profile.preflow_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_AMPERAGE) {
        Some(profile.initial_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_TIME) {
        Some(profile.initial_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_SLOPE_TIME) {
        Some(profile.initial_slope_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::MAIN_TIME) {
        Some(profile.main_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_SLOPE_TIME) {
        Some(profile.final_slope_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_AMPERAGE) {
        Some(profile.final_amperage)
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_TIME) {
        Some(profile.final_time)
    } else if std::ptr::eq(meta, &miller_register_definitions::HOT_WIRE_VOLTAGE) {
        Some(profile.hot_wire_voltage)
    } else {
        None
    }
}