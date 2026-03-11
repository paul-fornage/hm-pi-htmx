use crate::views::run_cycle::weld_file_ops;
use crate::views::run_cycle::motion_file_ops;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use crate::hx_trigger::HxTrigger;
use crate::views::miller_info::register_details_modal::RegisterModalTemplate;
use crate::views::miller_info::register_view::AnalogRegisterTemplate;
use crate::views::motion_profile::motion_profile::MotionProfile;
use crate::views::motion_profile::raw_motion_profile::RawMotionProfile;
use crate::views::run_cycle::{profiles_match, RunCycleFeedbackTemplate};
use crate::views::shared::WriteErrorModalTemplate;
use crate::views::{motion_profile, welder_profile};
use crate::{error_targeted, warn_targeted, AppState};
use crate::file_io::FixedDiskFile;
use crate::miller::miller_register_definitions;
use crate::plc::plc_register_definitions;
use crate::views::welder_profile::weld_profile::WeldProfile;
use crate::views::welder_profile::raw_weld_profile::RawWeldProfile;
use crate::views::schedule_adjustments::allowed_adjustments::AllowedAdjustments;
use crate::views::schedule_adjustments::allowed_adjustments::AdjustmentRowDisplay;

pub async fn run_cycle_analog_registers(
    State(state): State<AppState>,
) -> RunCycleAnalogRegistersTemplate {
    let (selected_weld, selected_motion) = {
        let weld_metadata = state.weld_profile_metadata.lock().await;
        let motion_metadata = state.motion_profile_metadata.lock().await;
        (weld_metadata.name.clone(), motion_metadata.name.clone())
    };

    let (Some(weld_name), Some(motion_name)) = (selected_weld, selected_motion) else {
        return RunCycleAnalogRegistersTemplate {
            rows: Vec::new(),
            missing_preview: None,
            error_message: Some("Adjustments are unavailable until you load both profiles.".to_string()),
        };
    };

    let weld_profile = match weld_file_ops::load_profile(&weld_name).await {
        Ok(profile) => profile,
        Err(err) => {
            error_targeted!(HTTP, "Failed to load selected weld profile '{weld_name}': {err}");
            return RunCycleAnalogRegistersTemplate {
                rows: Vec::new(),
                missing_preview: None,
                error_message: Some(format!("Failed to load weld profile '{weld_name}'.")),
            };
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
            };
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
            };
        }
    };

    if !weld_matches || !motion_matches {
        warn_targeted!(HTTP,
            "Run cycle presets not loaded yet (weld_loaded={weld_matches}, motion_loaded={motion_matches})"
        );
        return RunCycleAnalogRegistersTemplate {
            rows: Vec::new(),
            missing_preview: None,
            error_message: Some("Adjustments are unavailable because the selected profiles are not loaded yet.".to_string()),
        };
    }

    let allowed_adjustments = match AllowedAdjustments::load().await {
        Ok(mut form) => {
            form.conform_to_schema();
            Some(form)
        }
        Err(err) => {
            warn_targeted!(HTTP, "Failed to load allowed adjustments: {}", err);
            None
        }
    };

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
        let adjustment = allowed_adjustments
            .as_ref()
            .and_then(|form| form.get(info.meta.name))
            .and_then(|range| compute_adjustment_range(info, raw_value, range));
        rows.push(RunCycleAnalogRegisterRow {
            source: "ClearCore",
            register: AnalogRegisterTemplate {
                raw_value,
                register_info: info,
            },
            adjustment,
        });
    }

    for info in welder_profile::WELD_PROFILE_ANALOG_REGISTERS.iter() {
        let raw_value = weld_profile_analog_value(&weld_profile.raw_profile, info);
        if raw_value.is_none() {
            missing_names.push(info.meta.name);
        }
        let adjustment = allowed_adjustments
            .as_ref()
            .and_then(|form| form.get(info.meta.name))
            .and_then(|range| compute_adjustment_range(info, raw_value, range));
        rows.push(RunCycleAnalogRegisterRow {
            source: "Welder",
            register: AnalogRegisterTemplate {
                raw_value,
                register_info: info,
            },
            adjustment,
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
}

pub struct RunCycleAnalogRegisterRow {
    pub source: &'static str,
    pub register: AnalogRegisterTemplate,
    pub adjustment: Option<RunCycleAdjustmentRange>,
}

impl RunCycleAnalogRegisterRow {
    pub fn name(&self) -> &'static str {
        self.register.register_info.meta.name
    }

    pub fn unit(&self) -> &'static str {
        self.register.register_info.unit_string()
    }

    pub fn source_slug(&self) -> &'static str {
        match self.source {
            "ClearCore" => "clearcore",
            "Welder" => "welder",
            _ => "unknown",
        }
    }

    pub fn formatted_value(&self) -> String {
        self.register.formatted_value()
    }

    pub fn preset_input_value(&self) -> Option<String> {
        self.register
            .raw_value
            .map(|raw| self.register.register_info.formatted_value(Some(raw)))
    }

    pub fn has_value(&self) -> bool {
        self.register.has_value()
    }

    pub fn range_label(&self) -> String {
        match &self.adjustment {
            Some(range) => {
                let min_label = self.register.register_info.formatted_value(Some(range.min_raw));
                let max_label = self.register.register_info.formatted_value(Some(range.max_raw));
                if range.min_raw == range.max_raw {
                    min_label
                } else {
                    format!("{min_label}..{max_label}")
                }
            }
            None => "Not adjustable".to_string(),
        }
    }

    pub fn adjustment_min_value(&self) -> Option<String> {
        self.adjustment
            .map(|range| self.register.register_info.formatted_value(Some(range.min_raw)))
    }

    pub fn adjustment_max_value(&self) -> Option<String> {
        self.adjustment
            .map(|range| self.register.register_info.formatted_value(Some(range.max_raw)))
    }

    pub fn full_range_label(&self) -> String {
        let info = self.register.register_info;
        let min_label = info.formatted_value(Some(info.min_value));
        let max_label = info.formatted_value(Some(info.max_value));
        if info.min_value == info.max_value {
            min_label
        } else {
            format!("{min_label}..{max_label}")
        }
    }

    pub fn is_adjustable(&self) -> bool {
        match &self.adjustment {
            Some(range) => range.is_adjustable(),
            None => false,
        }
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/run-cycle/analog-registers.html")]
pub struct RunCycleAnalogRegistersTemplate {
    pub rows: Vec<RunCycleAnalogRegisterRow>,
    pub missing_preview: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Deserialize)]
pub struct RunCycleRegisterInfoPath {
    source: String,
    register_name: String,
}

pub async fn run_cycle_register_info_modal(
    State(state): State<AppState>,
    Path(path): Path<RunCycleRegisterInfoPath>,
) -> impl IntoResponse {
    let source = path.source.trim().to_ascii_lowercase();
    let register_name = path.register_name.trim().to_string();

    let register_rows = run_cycle_analog_registers(State(state)).await;
    if let Some(error_message) = &register_rows.error_message {
        warn_targeted!(HTTP, "Run cycle register info unavailable: {}", error_message);
        return info_modal_error(error_message.clone());
    }

    let row = register_rows.rows.iter().find(|row| {
        row.name() == register_name && row.source_slug() == source
    });

    let Some(row) = row else {
        warn_targeted!(
            HTTP,
            "Run cycle register info not found: source='{}', name='{}'",
            source,
            register_name
        );
        return info_modal_error(format!("Register '{register_name}' not found."));
    };

    if !row.has_value() {
        warn_targeted!(
            HTTP,
            "Run cycle register info missing preset value for '{}'",
            register_name
        );
        return info_modal_error(format!("Preset value missing for '{register_name}'."));
    }

    let value = row.formatted_value();
    let preset_value = if row.unit().is_empty() {
        value
    } else {
        format!("{} {}", value, row.unit())
    };

    let template = RegisterModalTemplate {
        meta: row.register.register_info.meta,
        preset_value: Some(preset_value),
    };

    Html(template.render().unwrap())
}

fn info_modal_error(message: String) -> Html<String> {
    let template = WriteErrorModalTemplate {
        title: "Info Unavailable".to_string(),
        message,
    };
    Html(template.render().unwrap())
}

#[derive(Clone, Copy)]
pub struct RunCycleAdjustmentRange {
    pub min_semantic: f32,
    pub max_semantic: f32,
    pub min_raw: u16,
    pub max_raw: u16,
    pub clamped_to_bounds: bool,
}

impl RunCycleAdjustmentRange {
    pub fn is_adjustable(&self) -> bool {
        self.min_semantic != self.max_semantic
    }
}

pub struct RunCycleAdjustedProfiles {
    pub weld_profile: WeldProfile,
    pub motion_profile: MotionProfile,
    pub adjusted_registers: Vec<String>,
}

pub fn apply_adjustments_to_profiles(
    weld_profile: &WeldProfile,
    motion_profile_data: &MotionProfile,
    allowed_adjustments: &AllowedAdjustments,
    adjustments: &HashMap<String, String>,
) -> Result<RunCycleAdjustedProfiles, String> {
    let mut adjusted_weld = weld_profile.clone();
    let mut adjusted_motion = motion_profile_data.clone();
    let mut seen_adjustments: HashSet<String> = HashSet::new();
    let mut adjusted_registers: Vec<String> = Vec::new();

    for info in motion_profile::MOTION_PROFILE_ANALOG_REGISTERS.iter() {
        let name = info.meta.name;
        let Some(range_row) = allowed_adjustments.get(name) else {
            if adjustments.contains_key(name) {
                return Err(format!("Register '{name}' is not adjustable."));
            }
            continue;
        };

        let base_raw = motion_profile_analog_value(&motion_profile_data.raw_profile, info)
            .ok_or_else(|| format!("Missing preset value for '{name}'."))?;
        let range = compute_adjustment_range(info, Some(base_raw), range_row)
            .ok_or_else(|| format!("Missing preset value for '{name}'."))?;

        if !range.is_adjustable() {
            if adjustments.contains_key(name) {
                return Err(format!("Register '{name}' is fixed and cannot be adjusted."));
            }
            continue;
        }

        let input = adjustments
            .get(name)
            .ok_or_else(|| format!("Missing adjustment for '{name}'."))?;
        let semantic_value = parse_adjustment_value(input, name)?;
        let adjusted_raw = info.convert_to_raw(semantic_value);

        if adjusted_raw < range.min_raw || adjusted_raw > range.max_raw {
            let min_label = info.formatted_value(Some(range.min_raw));
            let max_label = info.formatted_value(Some(range.max_raw));
            return Err(format!(
                "Adjustment for '{name}' must be between {min_label} and {max_label}."
            ));
        }

        if !set_motion_profile_analog_value(&mut adjusted_motion.raw_profile, info, adjusted_raw) {
            return Err(format!("Failed to apply adjustment for '{name}'."));
        }

        seen_adjustments.insert(name.to_string());
        if adjusted_raw != base_raw {
            adjusted_registers.push(name.to_string());
        }
    }

    for info in welder_profile::WELD_PROFILE_ANALOG_REGISTERS.iter() {
        let name = info.meta.name;
        let Some(range_row) = allowed_adjustments.get(name) else {
            if adjustments.contains_key(name) {
                return Err(format!("Register '{name}' is not adjustable."));
            }
            continue;
        };

        let base_raw = weld_profile_analog_value(&weld_profile.raw_profile, info)
            .ok_or_else(|| format!("Missing preset value for '{name}'."))?;
        let range = compute_adjustment_range(info, Some(base_raw), range_row)
            .ok_or_else(|| format!("Missing preset value for '{name}'."))?;

        if !range.is_adjustable() {
            if adjustments.contains_key(name) {
                return Err(format!("Register '{name}' is fixed and cannot be adjusted."));
            }
            continue;
        }

        let input = adjustments
            .get(name)
            .ok_or_else(|| format!("Missing adjustment for '{name}'."))?;
        let semantic_value = parse_adjustment_value(input, name)?;
        let adjusted_raw = info.convert_to_raw(semantic_value);

        if adjusted_raw < range.min_raw || adjusted_raw > range.max_raw {
            let min_label = info.formatted_value(Some(range.min_raw));
            let max_label = info.formatted_value(Some(range.max_raw));
            return Err(format!(
                "Adjustment for '{name}' must be between {min_label} and {max_label}."
            ));
        }

        if !set_weld_profile_analog_value(&mut adjusted_weld.raw_profile, info, adjusted_raw) {
            return Err(format!("Failed to apply adjustment for '{name}'."));
        }

        seen_adjustments.insert(name.to_string());
        if adjusted_raw != base_raw {
            adjusted_registers.push(name.to_string());
        }
    }

    for name in adjustments.keys() {
        if !seen_adjustments.contains(name) {
            return Err(format!("Register '{name}' is not adjustable."));
        }
    }

    Ok(RunCycleAdjustedProfiles {
        weld_profile: adjusted_weld,
        motion_profile: adjusted_motion,
        adjusted_registers,
    })
}

fn parse_adjustment_value(value: &str, name: &str) -> Result<f32, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("Adjusted value for '{name}' is empty."));
    }
    let parsed: f32 = trimmed
        .parse()
        .map_err(|_| format!("Adjusted value for '{name}' is not a number."))?;
    if !parsed.is_finite() {
        return Err(format!("Adjusted value for '{name}' is not a valid number."));
    }
    Ok(parsed)
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

fn compute_adjustment_range(
    info: &'static crate::views::shared::analog_register::AnalogRegisterInfo,
    raw_value: Option<u16>,
    range: &AdjustmentRowDisplay,
) -> Option<RunCycleAdjustmentRange> {
    let raw_value = raw_value?;
    let base_semantic = info.convert_from_raw(raw_value);
    let percent_less = range.percent_less() as f32 / 100.0;
    let percent_over = range.percent_over() as f32 / 100.0;

    let mut min_semantic = base_semantic * (1.0 - percent_less);
    let mut max_semantic = base_semantic * (1.0 + percent_over);
    if min_semantic > max_semantic {
        std::mem::swap(&mut min_semantic, &mut max_semantic);
    }

    let raw_min = info.min_value;
    let raw_max = info.max_value;
    let semantic_min_bound = info.convert_from_raw(raw_min);
    let semantic_max_bound = info.convert_from_raw(raw_max);

    let mut clamped = false;
    if min_semantic < semantic_min_bound {
        min_semantic = semantic_min_bound;
        clamped = true;
    }
    if max_semantic > semantic_max_bound {
        max_semantic = semantic_max_bound;
        clamped = true;
    }

    let min_raw = info.convert_to_raw(min_semantic);
    let max_raw = info.convert_to_raw(max_semantic);
    if min_raw > raw_max || max_raw > raw_max || min_raw < raw_min || max_raw < raw_min {
        warn_targeted!(
            HTTP,
            "Adjustment range exceeded raw bounds for {} (raw={}, min_raw={}, max_raw={}, bounds={}..{})",
            info.meta.name,
            raw_value,
            min_raw,
            max_raw,
            raw_min,
            raw_max
        );
        min_semantic = semantic_min_bound;
        max_semantic = semantic_max_bound;
        clamped = true;
    }

    Some(RunCycleAdjustmentRange {
        min_semantic,
        max_semantic,
        min_raw: info.convert_to_raw(min_semantic),
        max_raw: info.convert_to_raw(max_semantic),
        clamped_to_bounds: clamped,
    })
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

fn set_motion_profile_analog_value(
    profile: &mut RawMotionProfile,
    info: &'static crate::views::shared::analog_register::AnalogRegisterInfo,
    raw_value: u16,
) -> bool {
    // TODO: consolidate with motion_profile_analog_value mapping to avoid duplication.
    let meta = info.meta;
    if std::ptr::eq(meta, &plc_register_definitions::CYCLE_START_POS) {
        profile.cycle_start_pos = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_END_POS) {
        profile.cycle_end_pos = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_PARK_POS) {
        profile.cycle_park_pos = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_WELD_SPEED) {
        profile.cycle_weld_speed = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_X) {
        profile.cycle_reposition_speed_x = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_Y) {
        profile.cycle_reposition_speed_y = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_REPOSITION_SPEED_Z) {
        profile.cycle_reposition_speed_z = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_WIRE_FEED_SPEED) {
        profile.cycle_wire_feed_speed = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_AVC_VREF) {
        profile.cycle_avc_vref = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_Z_STATIC_OFFSET) {
        profile.cycle_z_static_offset = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_AXIS_Z_TORCH_UP_OFFSET) {
        profile.cycle_axis_z_torch_up_offset = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE) {
        profile.cycle_touch_retract_reposition_distance = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_PROBE_SPEED) {
        profile.cycle_touch_retract_probe_speed = raw_value;
    } else if std::ptr::eq(meta, &plc_register_definitions::CYCLE_TOUCH_RETRACT_FINAL_HEIGHT) {
        profile.cycle_touch_retract_final_height = raw_value;
    } else {
        return false;
    }
    true
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

fn set_weld_profile_analog_value(
    profile: &mut RawWeldProfile,
    info: &'static crate::views::shared::analog_register::AnalogRegisterInfo,
    raw_value: u16,
) -> bool {
    // TODO: consolidate with weld_profile_analog_value mapping to avoid duplication.
    let meta = info.meta;
    if std::ptr::eq(meta, &miller_register_definitions::PRESET_MIN_AMPERAGE) {
        profile.preset_min_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_AMPERAGE) {
        profile.arc_start_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_TIME) {
        profile.arc_start_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_SLOPE_TIME) {
        profile.arc_start_slope_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::ARC_START_AC_TIME) {
        profile.arc_start_ac_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::HOT_START_TIME) {
        profile.hot_start_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_EN_AMPERAGE) {
        profile.ac_en_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_EP_AMPERAGE) {
        profile.ac_ep_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_BALANCE) {
        profile.ac_balance = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::AC_FREQUENCY) {
        profile.ac_frequency = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::WELD_AMPERAGE) {
        profile.weld_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::PULSER_PPS) {
        profile.pulser_pps = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::PULSER_PEAK_TIME) {
        profile.pulser_peak_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::PREFLOW_TIME) {
        profile.preflow_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_AMPERAGE) {
        profile.initial_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_TIME) {
        profile.initial_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::INITIAL_SLOPE_TIME) {
        profile.initial_slope_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::MAIN_TIME) {
        profile.main_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_SLOPE_TIME) {
        profile.final_slope_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_AMPERAGE) {
        profile.final_amperage = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::FINAL_TIME) {
        profile.final_time = raw_value;
    } else if std::ptr::eq(meta, &miller_register_definitions::HOT_WIRE_VOLTAGE) {
        profile.hot_wire_voltage = raw_value;
    } else {
        return false;
    }
    true
}
