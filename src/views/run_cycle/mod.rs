mod adjustments;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

use crate::{debug_targeted, error_targeted, info_targeted, warn_targeted, AppState};
use crate::error::HmPiError;
use crate::file_io::FixedDiskFile;
use crate::hx_trigger::HxTrigger;
use crate::miller::miller_register_definitions;
use crate::plc::plc_register_definitions;
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::views::miller_info::register_view::AnalogRegisterTemplate;
use crate::views::motion_profile::file_operations as motion_file_ops;
use crate::views::motion_profile::motion_profile::{MotionProfile, ProfileListEntry as MotionProfileListEntry};
use crate::views::motion_profile::raw_motion_profile::RawMotionProfile;
use crate::views::shared::{mb_read_bool_helper, mb_read_word_helper, StatusFeedbackTemplate};
use crate::views::shared::finger_status::finger_status_handler;
use crate::views::welder_profile::file_operations as weld_file_ops;
use crate::views::welder_profile::raw_weld_profile::RawWeldProfile;
use crate::views::welder_profile::weld_profile::{ProfileListEntry as WeldProfileListEntry, WeldProfile};
use crate::views::schedule_adjustments::allowed_adjustments::AllowedAdjustments;
use crate::views::{motion_profile, welder_profile};
use adjustments::{run_cycle_analog_registers, apply_adjustments_to_profiles, adjusted_cycle_start_pos, AdjustedStartPos};
use crate::views::run_cycle::adjustments::feedback_ok_with_triggers;

const PROFILE_VERIFY_TIMEOUT: Duration = Duration::from_secs(5);
const JOB_ACTIVE_TIMEOUT: Duration = Duration::from_secs(5);
const START_POS_VERIFY_TIMEOUT: Duration = Duration::from_secs(5);
const PRESET_VALUES_LOADED_EVENT: HxTrigger = HxTrigger {
    event: "run-cycle-presets-loaded",
    target: "#run-cycle-analog-registers",
};
const CYCLE_START_REGISTER_NAME: &str = plc_register_definitions::CYCLE_START_POS.name;

fn adjustment_input_id(name: &str) -> String {
    let mut output = String::with_capacity(name.len());
    let mut last_dash = false;
    for ch in name.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            output.push(lower);
            last_dash = false;
        } else if !last_dash {
            output.push('-');
            last_dash = true;
        }
    }
    while output.ends_with('-') {
        output.pop();
    }
    if output.is_empty() {
        "unknown".to_string()
    } else {
        output
    }
}

pub fn routes() -> Router<AppState> {
    let page = AppView::RunCycle;
    Router::new()
        .route(page.url(), get(show_run_cycle))
        .route(&page.url_with_path("/load-profiles"), post(load_profiles))
        .route(&page.url_with_path("/selected-profiles"), post(selected_profiles))
        .route(&page.url_with_path("/start"), post(start_cycle_real))
        .route(&page.url_with_path("/start-simulate"), post(start_cycle_simulate))
        .route(&page.url_with_path("/finger-status/{side}"), get(finger_status_handler))
        .route(&page.url_with_path("/go-to-start"), post(go_to_start))
        .route(&page.url_with_path("/status"), get(run_cycle_status))
        .route(&page.url_with_path("/analog-registers"), get(run_cycle_analog_registers))
        .route(&page.url_with_path("/register-info/{source}/{register_name}"), get(adjustments::run_cycle_register_info_modal))
}

pub async fn show_run_cycle(
    State(state): State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Rendering run cycle view");

    let weld_profiles = match weld_file_ops::list_profiles().await {
        Ok(list) => list,
        Err(err) => {
            error_targeted!(HTTP, "Failed to list weld profiles: {}", err);
            Vec::new()
        }
    };
    let motion_profiles = match motion_file_ops::list_profiles().await {
        Ok(list) => list,
        Err(err) => {
            error_targeted!(HTTP, "Failed to list motion profiles: {}", err);
            Vec::new()
        }
    };

    let selected_weld = {
        let metadata = state.weld_profile_metadata.lock().await;
        metadata.name.clone()
    };
    let selected_motion = {
        let metadata = state.motion_profile_metadata.lock().await;
        metadata.name.clone()
    };

    let selected_weld = selected_weld.filter(|name| weld_profiles.iter().any(|p| p.name == *name));
    let selected_motion = selected_motion.filter(|name| motion_profiles.iter().any(|p| p.name == *name));

    let header = build_header_context(&state, AppView::RunCycle).await;
    RunCycleTemplate {
        header,
        weld_profiles,
        motion_profiles,
        selected_weld,
        selected_motion,
        cycle_start_register_name: CYCLE_START_REGISTER_NAME,
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "views/run-cycle.html")]
pub struct RunCycleTemplate {
    pub header: HeaderContext,
    pub weld_profiles: Vec<WeldProfileListEntry>,
    pub motion_profiles: Vec<MotionProfileListEntry>,
    pub selected_weld: Option<String>,
    pub selected_motion: Option<String>,
    pub cycle_start_register_name: &'static str,
}

impl RunCycleTemplate {
    pub fn has_selected_weld(&self) -> bool {
        self.selected_weld.is_some()
    }

    pub fn has_selected_motion(&self) -> bool {
        self.selected_motion.is_some()
    }

    pub fn is_selected_weld(&self, name: &str) -> bool {
        self.selected_weld.as_deref() == Some(name)
    }

    pub fn is_selected_motion(&self, name: &str) -> bool {
        self.selected_motion.as_deref() == Some(name)
    }
}

impl ViewTemplate for RunCycleTemplate {
    const APP_VIEW_VARIANT: AppView = AppView::RunCycle;
}

#[derive(Deserialize)]
pub struct RunCycleForm {
    weld_profile: Option<String>,
    motion_profile: Option<String>,
}

#[derive(Template)]
#[template(path = "components/shared/result-feedback.html")]
pub struct RunCycleFeedbackTemplate {
    pub result: Result<String, String>,
}

pub async fn selected_profiles(
    Form(form): Form<RunCycleForm>,
) -> impl IntoResponse {
    let weld_profiles = match weld_file_ops::list_profiles().await {
        Ok(list) => list,
        Err(err) => {
            error_targeted!(HTTP, "Failed to list weld profiles: {}", err);
            Vec::new()
        }
    };
    let motion_profiles = match motion_file_ops::list_profiles().await {
        Ok(list) => list,
        Err(err) => {
            error_targeted!(HTTP, "Failed to list motion profiles: {}", err);
            Vec::new()
        }
    };

    let weld_name = form
        .weld_profile
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string);
    let motion_name = form
        .motion_profile
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_string);

    let weld_description = weld_name
        .as_deref()
        .and_then(|name| weld_profiles.iter().find(|profile| profile.name == name))
        .map(|profile| profile.description.trim().to_string())
        .filter(|desc| !desc.is_empty());
    let motion_description = motion_name
        .as_deref()
        .and_then(|name| motion_profiles.iter().find(|profile| profile.name == name))
        .map(|profile| profile.description.trim().to_string())
        .filter(|desc| !desc.is_empty());

    RunCycleSelectedProfilesTemplate {
        weld_name,
        weld_description,
        motion_name,
        motion_description,
    }
}

pub async fn start_cycle_real(
    State(state): State<AppState>,
    Form(form): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    start_cycle(&state, &form, false).await
}

pub async fn start_cycle_simulate(
    State(state): State<AppState>,
    Form(form): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    start_cycle(&state, &form, true).await
}

async fn start_cycle(
    state: &AppState,
    form: &HashMap<String, String>,
    simulate_weld: bool,
) -> impl IntoResponse {
    let use_adjusted = use_adjusted_presets(form);
    debug_targeted!(
        HTTP,
        "Run cycle start requested: weld='{:?}', motion='{:?}', simulate={}, use_adjusted={}",
        form.get("weld_profile"),
        form.get("motion_profile"),
        simulate_weld,
        use_adjusted
    );

    let weld_name = match extract_profile_name(form, "weld_profile") {
        Some(name) => name,
        None => return feedback_err("Select both profiles before starting".to_string()),
    };
    let motion_name = match extract_profile_name(form, "motion_profile") {
        Some(name) => name,
        None => return feedback_err("Select both profiles before starting".to_string()),
    };

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address).await,
        Some(true)
    ) {
        return feedback_err("Job already active".to_string());
    }

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::IS_HOMED.address).await,
        Some(false)
    ) {
        return feedback_err("ClearCore not homed".to_string());
    }

    let weld_profile = match weld_file_ops::load_profile(&weld_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load weld profile: {}", err)),
    };
    let motion_profile = match motion_file_ops::load_profile(&motion_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load motion profile: {}", err)),
    };

    let (weld_matches, motion_matches) = match profiles_match(&state, &weld_profile, &motion_profile).await {
        Ok(result) => result,
        Err(err) => return feedback_err(format!("Failed to read back profiles: {}", err)),
    };

    if !weld_matches || !motion_matches {
        if !weld_matches {
            let diff_regs = weld_profile
                .raw_profile
                .modbus_diff(&state.miller_registers)
                .await;
            error_targeted!(MODBUS, "Weld profile mismatch: {:#?}", diff_regs);
        }
        if !motion_matches {
            let diff_regs = motion_profile
                .raw_profile
                .modbus_diff(&state.clearcore_registers)
                .await;
            error_targeted!(MODBUS, "Motion profile mismatch: {:#?}", diff_regs);
        }
        return feedback_err("Profiles are not loaded".to_string());
    }

    match mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::AT_START.address).await {
        Some(true) => {}
        Some(false) => return feedback_err("Machine is not at the start position".to_string()),
        None => return feedback_err("Unable to read start position".to_string()),
    }

    if use_adjusted {
        let adjustments = extract_adjustments(form);
        if adjustments.is_empty() {
            return feedback_err("Adjusted presets requested, but no values were provided.".to_string());
        }

        let allowed_adjustments = match AllowedAdjustments::load().await {
            Ok(mut form) => {
                form.conform_to_schema();
                form
            }
            Err(err) => {
                warn_targeted!(HTTP, "Failed to load allowed adjustments: {}", err);
                return feedback_err("Adjusted presets are unavailable because adjustment limits failed to load.".to_string());
            }
        };

        let adjusted_profiles = match apply_adjustments_to_profiles(
            &weld_profile,
            &motion_profile,
            &allowed_adjustments,
            &adjustments,
        ) {
            Ok(profiles) => profiles,
            Err(err) => {
                warn_targeted!(HTTP, "Adjusted preset validation failed: {}", err);
                return feedback_err(err);
            }
        };

        if adjusted_profiles.adjusted_registers.is_empty() {
            info_targeted!(HTTP, "Adjusted presets enabled with no value changes");
        } else {
            info_targeted!(
                HTTP,
                "Applying {} adjusted preset values",
                adjusted_profiles.adjusted_registers.len()
            );
        }

        let weld_diff = adjusted_profiles
            .weld_profile
            .raw_profile
            .modbus_diff(&state.miller_registers)
            .await;
        let motion_diff = adjusted_profiles
            .motion_profile
            .raw_profile
            .modbus_diff(&state.clearcore_registers)
            .await;

        if let Err(err) = RawWeldProfile::apply_diff(&state.miller_registers, weld_diff).await {
            error_targeted!(MODBUS, "Failed to apply adjusted weld profile: {}", err);
            return feedback_err(format!("Failed to apply adjusted weld presets: {}", err));
        }

        if let Err(err) = RawMotionProfile::apply_diff(&state.clearcore_registers, motion_diff).await {
            error_targeted!(MODBUS, "Failed to apply adjusted motion profile: {}", err);
            return feedback_err(format!("Failed to apply adjusted motion presets: {}", err));
        }

        if let Err(err) = wait_for_profiles(
            &state,
            &adjusted_profiles.weld_profile,
            &adjusted_profiles.motion_profile,
        )
        .await
        {
            let weld_diff = adjusted_profiles
                .weld_profile
                .raw_profile
                .modbus_diff(&state.miller_registers)
                .await;
            let motion_diff = adjusted_profiles
                .motion_profile
                .raw_profile
                .modbus_diff(&state.clearcore_registers)
                .await;
            if !weld_diff.is_empty() {
                error_targeted!(MODBUS, "Adjusted weld profile mismatch: {:#?}", weld_diff);
            }
            if !motion_diff.is_empty() {
                error_targeted!(MODBUS, "Adjusted motion profile mismatch: {:#?}", motion_diff);
            }
            warn_targeted!(HTTP, "Adjusted presets failed readback: {}", err);
            return feedback_err(
                "Adjusted presets failed to apply. Verify controller limits and try again."
                    .to_string(),
            );
        }
    }

    if let Err(err) = state
        .clearcore_registers
        .write_coil(
            plc_register_definitions::WELDER_SIMULATE_MODE.address.address,
            simulate_weld,
        )
        .await
    {
        return feedback_err(format!("Failed to set simulate mode: {}", err));
    }

    if let Err(err) = state
        .clearcore_registers
        .write_coil(plc_register_definitions::START_CYCLE_LATCH.address.address, true)
        .await
    {
        return feedback_err(format!("Failed to start cycle: {}", err));
    }

    if let Err(err) = wait_for_job_active(&state).await {
        return feedback_err(err);
    }

    feedback_ok("Cycle started".to_string())
}

pub async fn load_profiles(
    State(state): State<AppState>,
    Form(form): Form<RunCycleForm>,
) -> axum::response::Response {
    debug_targeted!(
        HTTP,
        "Load profiles requested: weld='{:?}', motion='{:?}'",
        form.weld_profile,
        form.motion_profile
    );

    let weld_name = match form.weld_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => name,
        None => return feedback_err("Select both profiles before loading".to_string()).into_response(),
    };
    let motion_name = match form.motion_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => name,
        None => return feedback_err("Select both profiles before loading".to_string()).into_response(),
    };

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address).await,
        Some(true)
    ) {
        return feedback_err("Job already active".to_string()).into_response();
    }

    let weld_profile = match weld_file_ops::load_profile(weld_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load weld profile: {}", err)).into_response(),
    };
    let motion_profile = match motion_file_ops::load_profile(motion_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load motion profile: {}", err)).into_response(),
    };

    let weld_profile_diff = weld_profile.raw_profile.modbus_diff(&state.miller_registers).await;
    let motion_profile_diff = motion_profile.raw_profile.modbus_diff(&state.clearcore_registers).await;

    for reg in weld_profile_diff.iter() {
        info_targeted!(MODBUS, "Weld profile register diff: {:?}", reg);
    }
    for reg in motion_profile_diff.iter() {
        info_targeted!(MODBUS, "Motion profile register diff: {:?}", reg);
    }

    if let Err(err) = RawWeldProfile::apply_diff(&state.miller_registers, weld_profile_diff)
        .await
    {
        return feedback_err(format!("Failed to upload weld profile: {}", err)).into_response();
    }

    if let Err(err) = RawMotionProfile::apply_diff(&state.clearcore_registers, motion_profile_diff)
        .await
    {
        return feedback_err(format!("Failed to upload motion profile: {}", err)).into_response();
    }

    if let Err(err) = wait_for_profiles(&state, &weld_profile, &motion_profile).await {
        return feedback_err(err.to_string()).into_response();
    }

    set_selected_profiles(&state, &weld_profile, &motion_profile).await;

    feedback_ok_with_triggers(
        "Profiles loaded".to_string(),
        &[PRESET_VALUES_LOADED_EVENT],
    )
}

pub async fn run_cycle_status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let job_active = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address)
        .await;
    let is_homed = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::IS_HOMED.address)
        .await;
    let at_start = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::AT_START.address)
        .await;

    let left_fingers_clamped = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::LF_COMMANDED_DOWN.address)
        .await;
    let right_fingers_clamped = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::RF_COMMANDED_DOWN.address)
        .await;

    let mandrel_latch_closed = mb_read_bool_helper(
        &state.clearcore_registers,
        &plc_register_definitions::MANDREL_LATCH_CLOSED.address,
    )
        .await;


    let arc_commanded = mb_read_bool_helper(
        &state.clearcore_registers,
        &plc_register_definitions::WELDER_ARC_COMMANDED.address,
    )
    .await;
    let arc_valid = mb_read_bool_helper(
        &state.clearcore_registers,
        &plc_register_definitions::WELDER_ARC_VALID.address,
    )
    .await;

    let progress_raw = if matches!(job_active, Some(true)) {
        mb_read_word_helper(&state.clearcore_registers, &plc_register_definitions::CYCLE_PROGRESS.address)
            .await
            .unwrap_or(0)
    } else {
        0
    };

    let progress_percent = (progress_raw.min(10000) as u32 / 100) as u16;
    let progress_label = format!("{}%", progress_percent);
    let progress_width = progress_label.clone();

    RunCycleStatusTemplate {
        job_active,
        is_homed,
        at_start,
        arc_commanded,
        arc_valid,
        mandrel_latch_closed,
        left_fingers_clamped,
        right_fingers_clamped,
        progress_label,
        progress_width,
    }
}

// pub async fn run_cycle_status_feedback(
//     State(state): State<AppState>,
// ) -> impl IntoResponse {
//     let mandrel_latch_closed = mb_read_bool_helper(
//         &state.clearcore_registers,
//         &plc_register_definitions::MANDREL_LATCH_CLOSED.address,
//     )
//     .await;
//
//     StatusFeedbackTemplate {
//         mandrel_latch_closed,
//     }
// }



#[derive(Template, WebTemplate)]
#[template(path = "components/run-cycle/status.html")]
pub struct RunCycleStatusTemplate {
    pub job_active: Option<bool>,
    pub is_homed: Option<bool>,
    pub at_start: Option<bool>,
    pub arc_commanded: Option<bool>,
    pub arc_valid: Option<bool>,
    pub mandrel_latch_closed: Option<bool>,
    pub left_fingers_clamped: Option<bool>,
    pub right_fingers_clamped: Option<bool>,
    pub progress_label: String,
    pub progress_width: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/run-cycle/selected-profiles.html")]
pub struct RunCycleSelectedProfilesTemplate {
    pub weld_name: Option<String>,
    pub weld_description: Option<String>,
    pub motion_name: Option<String>,
    pub motion_description: Option<String>,
}


pub async fn go_to_start(
    State(state): State<AppState>,
    Form(form): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let use_adjusted = use_adjusted_presets(&form);
    debug_targeted!(HTTP, "Go to start requested (use_adjusted={})", use_adjusted);

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address).await,
        Some(true)
    ) {
        return feedback_err("Job already active".to_string());
    }

    match mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::IS_HOMED.address).await {
        Some(true) => {}
        Some(false) => return feedback_err("Machine is not homed".to_string()),
        None => return feedback_err("Unable to read homed state".to_string()),
    }

    let mut adjusted_note: Option<String> = None;
    if use_adjusted {
        let adjustments = extract_adjustments(&form);
        let raw_input = adjustments
            .get(CYCLE_START_REGISTER_NAME)
            .map(|value| value.trim().to_string());
        let adjusted_input = match raw_input.as_deref() {
            Some(value) if !value.is_empty() && !value.eq_ignore_ascii_case("none") => Some(value.to_string()),
            _ => None,
        };

        if let Some(input) = adjusted_input {
            let allowed_adjustments = match AllowedAdjustments::load().await {
                Ok(mut form) => {
                    form.conform_to_schema();
                    form
                }
                Err(err) => {
                    warn_targeted!(HTTP, "Failed to load allowed adjustments: {}", err);
                    return feedback_err("Adjusted start position unavailable because adjustment limits failed to load.".to_string());
                }
            };

            let (selected_weld, selected_motion) = {
                let weld_metadata = state.weld_profile_metadata.lock().await;
                let motion_metadata = state.motion_profile_metadata.lock().await;
                (weld_metadata.name.clone(), motion_metadata.name.clone())
            };
            let (Some(weld_name), Some(motion_name)) = (selected_weld, selected_motion) else {
                return feedback_err("Adjusted start position unavailable until both profiles are loaded.".to_string());
            };

            let weld_profile = match weld_file_ops::load_profile(&weld_name).await {
                Ok(profile) => profile,
                Err(err) => {
                    error_targeted!(HTTP, "Failed to load selected weld profile '{weld_name}': {err}");
                    return feedback_err(format!("Failed to load weld profile '{weld_name}'."));
                }
            };
            let motion_profile = match motion_file_ops::load_profile(&motion_name).await {
                Ok(profile) => profile,
                Err(err) => {
                    error_targeted!(HTTP, "Failed to load selected motion profile '{motion_name}': {err}");
                    return feedback_err(format!("Failed to load motion profile '{motion_name}'."));
                }
            };

            let (weld_matches, motion_matches) = match profiles_match(&state, &weld_profile, &motion_profile).await {
                Ok(result) => result,
                Err(err) => return feedback_err(format!("Failed to read back profiles: {}", err)),
            };
            if !weld_matches || !motion_matches {
                return feedback_err("Profiles are not loaded. Reload before using adjustments.".to_string());
            }

            let AdjustedStartPos { base_raw, adjusted_raw } =
                match adjusted_cycle_start_pos(&motion_profile, &allowed_adjustments, &input) {
                    Ok(result) => result,
                    Err(err) => return feedback_err(err),
                };

            if adjusted_raw == base_raw {
                adjusted_note = Some("adjusted start position matches preset".to_string());
                info_targeted!(HTTP, "Adjusted start position matches preset");
            } else {
                if let Err(err) = state
                    .clearcore_registers
                    .write_hreg(plc_register_definitions::CYCLE_START_POS.address.address, adjusted_raw)
                    .await
                {
                    return feedback_err(format!("Failed to apply adjusted start position: {}", err));
                }
                if let Err(err) = wait_for_start_pos_readback(&state, adjusted_raw).await {
                    warn_targeted!(HTTP, "Adjusted start position readback failed: {}", err);
                    return feedback_err(err);
                }
                adjusted_note = Some("adjusted start position applied".to_string());
            }
        } else {
            adjusted_note = Some("adjusted start position unavailable".to_string());
            info_targeted!(HTTP, "Adjusted start position requested but no adjustable value provided");
        }
    }

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::AT_START.address).await,
        Some(true)
    ) {
        let message = match adjusted_note {
            Some(note) => format!("Already at start position ({note})"),
            None => "Already at start position".to_string(),
        };
        return feedback_ok(message);
    }

    if let Err(err) = state
        .clearcore_registers
        .write_coil(plc_register_definitions::GO_TO_START_LATCH.address.address, true)
        .await
    {
        return feedback_err(format!("Failed to go to start: {}", err));
    }

    let message = match adjusted_note {
        Some(note) => format!("Going to start position ({note})"),
        None => "Going to start position".to_string(),
    };
    feedback_ok(message)
}

async fn wait_for_start_pos_readback(state: &AppState, expected: u16) -> Result<(), String> {
    let started = Instant::now();
    let mut last_read: Option<u16> = None;

    loop {
        if let Some(value) = mb_read_word_helper(
            &state.clearcore_registers,
            &plc_register_definitions::CYCLE_START_POS.address,
        )
        .await
        {
            last_read = Some(value);
            if value == expected {
                return Ok(());
            }
        }

        if started.elapsed() >= START_POS_VERIFY_TIMEOUT {
            let last_label = last_read
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unreadable".to_string());
            return Err(format!(
                "Timed out waiting for start position readback (expected {expected}, got {last_label})."
            ));
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn profiles_match(
    state: &AppState,
    weld_profile: &WeldProfile,
    motion_profile: &MotionProfile,
) -> Result<(bool, bool), HmPiError> {
    let weld_current = RawWeldProfile::capture_from_memory(&state.miller_registers).await?;
    let motion_current = RawMotionProfile::capture_from_memory(&state.clearcore_registers).await?;
    Ok((
        weld_current == weld_profile.raw_profile,
        motion_current == motion_profile.raw_profile,
    ))
}

async fn wait_for_profiles(
    state: &AppState,
    weld_profile: &WeldProfile,
    motion_profile: &MotionProfile,
) -> Result<(), HmPiError> {
    let started = Instant::now();
    let mut last_error: Option<HmPiError> = None;

    loop {
        match profiles_match(state, weld_profile, motion_profile).await {
            Ok((true, true)) => return Ok(()),
            Ok((weld_matches, motion_matches)) => {
                debug_targeted!(MODBUS,
                    "Weld profile match: {weld_matches}, \
                    Motion profile match: {motion_matches}")
            },
            Err(err) => last_error = Some(err),
        }

        if started.elapsed() >= PROFILE_VERIFY_TIMEOUT {

            let different_regs = weld_profile
                .raw_profile
                .modbus_diff(&state.miller_registers)
                .await;
            if !different_regs.is_empty() {
                error_targeted!(MODBUS, "Weld profile mismatch: {:#?}", different_regs);
            }

            return match last_error {
                Some(err) => Err(err),
                None => Err(HmPiError::ModbusFailToReadBackWrites()),
            };
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn wait_for_job_active(state: &AppState) -> Result<(), String> {
    let started = Instant::now();
    loop {
        if matches!(
            mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address).await,
            Some(true)
        ) {
            return Ok(());
        }

        if started.elapsed() >= JOB_ACTIVE_TIMEOUT {
            return Err("Timed out waiting for JOB_ACTIVE".to_string());
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn set_selected_profiles(
    state: &AppState,
    weld_profile: &WeldProfile,
    motion_profile: &MotionProfile,
) {
    {
        let mut metadata = state.weld_profile_metadata.lock().await;
        metadata.name = Some(weld_profile.name.clone());
        metadata.description = if weld_profile.description.trim().is_empty() {
            None
        } else {
            Some(weld_profile.description.clone())
        };
    }
    {
        let mut metadata = state.motion_profile_metadata.lock().await;
        metadata.name = Some(motion_profile.name.clone());
        metadata.description = if motion_profile.description.trim().is_empty() {
            None
        } else {
            Some(motion_profile.description.clone())
        };
    }
}

fn feedback_ok(message: String) -> Html<String> {
    Html(
        RunCycleFeedbackTemplate {
            result: Ok(message),
        }
        .render()
        .unwrap(),
    )
}

fn feedback_err(message: String) -> Html<String> {
    warn_targeted!(HTTP, "Run cycle failed: {}", message);
    Html(
        RunCycleFeedbackTemplate {
            result: Err(message),
        }
        .render()
        .unwrap(),
    )
}

fn extract_profile_name(form: &HashMap<String, String>, key: &str) -> Option<String> {
    form.get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn use_adjusted_presets(form: &HashMap<String, String>) -> bool {
    match form.get("use_adjusted_presets") {
        Some(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "true" | "on" | "1" | "yes")
        }
        None => false,
    }
}

fn extract_adjustments(form: &HashMap<String, String>) -> HashMap<String, String> {
    let mut adjustments = HashMap::new();
    for (key, value) in form.iter() {
        if let Some(name) = parse_adjustment_key(key) {
            adjustments.insert(name, value.trim().to_string());
        }
    }
    adjustments
}

fn parse_adjustment_key(key: &str) -> Option<String> {
    const PREFIX: &str = "adjustments[";
    if key.starts_with(PREFIX) && key.ends_with(']') {
        let name = &key[PREFIX.len()..key.len() - 1];
        if name.trim().is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    } else {
        None
    }
}
