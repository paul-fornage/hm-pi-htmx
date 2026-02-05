use askama::Template;
use askama_web::WebTemplate;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use tokio::time::{Duration, Instant};

use crate::{debug_targeted, error_targeted, info_targeted, warn_targeted, AppState};
use crate::error::HmPiError;
use crate::plc::plc_register_definitions;
use crate::views::{AppView, ViewTemplate};
use crate::views::motion_profile::file_operations as motion_file_ops;
use crate::views::motion_profile::motion_profile::{MotionProfile, ProfileListEntry as MotionProfileListEntry};
use crate::views::motion_profile::raw_motion_profile::RawMotionProfile;
use crate::views::shared::{mb_read_bool_helper, mb_read_word_helper, StatusFeedbackTemplate};
use crate::views::welder_profile::file_operations as weld_file_ops;
use crate::views::welder_profile::raw_weld_profile::RawWeldProfile;
use crate::views::welder_profile::weld_profile::{ProfileListEntry as WeldProfileListEntry, WeldProfile};

const PROFILE_VERIFY_TIMEOUT: Duration = Duration::from_secs(5);
const JOB_ACTIVE_TIMEOUT: Duration = Duration::from_secs(5);

pub fn routes() -> Router<AppState> {
    let page = AppView::RunCycle;
    Router::new()
        .route(page.url(), get(show_run_cycle))
        .route(&page.url_with_path("/load-profiles"), post(load_profiles))
        .route(&page.url_with_path("/selected-profiles"), post(selected_profiles))
        .route(&page.url_with_path("/start"), post(start_cycle_real))
        .route(&page.url_with_path("/start-simulate"), post(start_cycle_simulate))
        .route(&page.url_with_path("/go-to-start"), post(go_to_start))
        .route(&page.url_with_path("/status"), get(run_cycle_status))
        .route(&page.url_with_path("/status-feedback"), get(run_cycle_status_feedback))
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

    RunCycleTemplate {
        weld_profiles,
        motion_profiles,
        selected_weld,
        selected_motion,
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "views/run-cycle.html")]
pub struct RunCycleTemplate {
    pub weld_profiles: Vec<WeldProfileListEntry>,
    pub motion_profiles: Vec<MotionProfileListEntry>,
    pub selected_weld: Option<String>,
    pub selected_motion: Option<String>,
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
    Form(form): Form<RunCycleForm>,
) -> impl IntoResponse {
    start_cycle(&state, &form, false).await
}

pub async fn start_cycle_simulate(
    State(state): State<AppState>,
    Form(form): Form<RunCycleForm>,
) -> impl IntoResponse {
    start_cycle(&state, &form, true).await
}

async fn start_cycle(
    state: &AppState,
    form: &RunCycleForm,
    simulate_weld: bool,
) -> impl IntoResponse {
    debug_targeted!(
        HTTP,
        "Run cycle start requested: weld='{:?}', motion='{:?}', simulate={}",
        form.weld_profile,
        form.motion_profile,
        simulate_weld
    );

    let weld_name = match form.weld_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => name,
        None => return feedback_err("Select both profiles before starting".to_string()),
    };
    let motion_name = match form.motion_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
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

    let weld_profile = match weld_file_ops::load_profile(weld_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load weld profile: {}", err)),
    };
    let motion_profile = match motion_file_ops::load_profile(motion_name).await {
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
) -> impl IntoResponse {
    debug_targeted!(
        HTTP,
        "Load profiles requested: weld='{:?}', motion='{:?}'",
        form.weld_profile,
        form.motion_profile
    );

    let weld_name = match form.weld_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => name,
        None => return feedback_err("Select both profiles before loading".to_string()),
    };
    let motion_name = match form.motion_profile.as_deref().map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => name,
        None => return feedback_err("Select both profiles before loading".to_string()),
    };

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address).await,
        Some(true)
    ) {
        return feedback_err("Job already active".to_string());
    }

    let weld_profile = match weld_file_ops::load_profile(weld_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load weld profile: {}", err)),
    };
    let motion_profile = match motion_file_ops::load_profile(motion_name).await {
        Ok(profile) => profile,
        Err(err) => return feedback_err(format!("Failed to load motion profile: {}", err)),
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
        return feedback_err(format!("Failed to upload weld profile: {}", err));
    }

    if let Err(err) = RawMotionProfile::apply_diff(&state.clearcore_registers, motion_profile_diff)
        .await
    {
        return feedback_err(format!("Failed to upload motion profile: {}", err));
    }

    if let Err(err) = wait_for_profiles(&state, &weld_profile, &motion_profile).await {
        return feedback_err(err.to_string());
    }

    set_selected_profiles(&state, &weld_profile, &motion_profile).await;

    feedback_ok("Profiles loaded".to_string())
}

pub async fn run_cycle_status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let job_active = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::JOB_ACTIVE.address)
        .await
        .unwrap_or(false);

    let is_homed = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::IS_HOMED.address)
        .await;
    let at_start = mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::AT_START.address)
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

    let progress_raw = if job_active {
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
        progress_label,
        progress_width,
    }
}

pub async fn run_cycle_status_feedback(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mandrel_latch_closed = mb_read_bool_helper(
        &state.clearcore_registers,
        &plc_register_definitions::MANDREL_LATCH_CLOSED.address,
    )
    .await;

    StatusFeedbackTemplate {
        mandrel_latch_closed,
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "components/run-cycle/status.html")]
pub struct RunCycleStatusTemplate {
    pub job_active: bool,
    pub is_homed: Option<bool>,
    pub at_start: Option<bool>,
    pub arc_commanded: Option<bool>,
    pub arc_valid: Option<bool>,
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
) -> impl IntoResponse {
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

    if matches!(
        mb_read_bool_helper(&state.clearcore_registers, &plc_register_definitions::AT_START.address).await,
        Some(true)
    ) {
        return feedback_ok("Already at start position".to_string());
    }

    if let Err(err) = state
        .clearcore_registers
        .write_coil(plc_register_definitions::GO_TO_START_LATCH.address.address, true)
        .await
    {
        return feedback_err(format!("Failed to go to start: {}", err));
    }

    feedback_ok("Going to start position".to_string())
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
