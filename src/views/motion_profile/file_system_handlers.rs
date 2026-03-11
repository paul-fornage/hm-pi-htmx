use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Form;
use serde::Deserialize;

use super::file_operations;
use super::file_system_templates::{
    LoadModalTemplate,
    LoadPreviewTemplate,
    LoadPreviewWindow,
    LoadProfileListTemplate,
    ProfileDeleteTemplate,
    ProfileFsOpResult,
    SaveAsModalTemplate,
    SaveAsProfileListTemplate,
};
use super::motion_profile::MotionProfile;
use super::raw_motion_profile::RawMotionProfile;
use super::BASE_URL;
use crate::{debug_targeted, error_targeted, info_targeted, warn_targeted, AppState};

pub async fn handle_save(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Save motion profile requested");

    let metadata = state.motion_profile_metadata.lock().await;
    let profile_name = match &metadata.name {
        Some(name) => name.clone(),
        None => {
            warn_targeted!(HTTP, "Cannot save motion profile: no name set");
            return ProfileFsOpResult::new_err_str("No profile name set".to_string());
        }
    };

    let description = metadata.description.clone().unwrap_or_default();
    drop(metadata);

    let raw_profile = match RawMotionProfile::capture_from_memory(&state.clearcore_registers).await {
        Ok(profile) => profile,
        Err(e) => {
            error_targeted!(HTTP, "Failed to capture profile from memory: {}", e);
            return ProfileFsOpResult::new_err_str("Failed to read from controller!".to_string());
        }
    };

    let profile = MotionProfile::new(profile_name.clone(), description, raw_profile);

    match file_operations::save_profile(&profile).await {
        Ok(_) => {
            info_targeted!(HTTP, "Successfully saved motion profile as: {}", profile_name);
            ProfileFsOpResult::new_ok_str(format!("Saved as {}", profile_name))
        }
        Err(e) => {
            error_targeted!(HTTP, "Failed to save motion profile {}: {}", profile_name, e);
            ProfileFsOpResult::new_err_str("Failed to save!".to_string())
        }
    }
}

pub async fn handle_save_as_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile save-as modal requested");

    let profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error_targeted!(HTTP, "Failed to list profiles: {}", e);
            return Err(ProfileFsOpResult{
                result: Err("Failed to load profiles!".to_string()),
                close_modal: false,
                reload_metadata: false,
                retarget: Some(ProfileFsOpResult::<String>::DEFAULT_TARGET)
            })
        }
    };

    let metadata = state.motion_profile_metadata.lock().await;
    let current_name = metadata.name.clone();
    drop(metadata);

    Ok(SaveAsModalTemplate {
        base_url: BASE_URL,
        current_name,
        profiles,
    })
}

#[derive(Deserialize)]
pub struct SearchQuery {
    name: String,
}

#[derive(Deserialize)]
pub struct LoadListQuery {
    search: Option<String>,
}

pub async fn handle_save_as_search(
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile save-as search requested: {}", query.name);

    let all_profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error_targeted!(HTTP, "Failed to get profiles: {}", e);
            return SaveAsProfileListTemplate {
                profiles: Vec::new(),
            };
        }
    };

    let search_lower = query.name.to_lowercase();
    let filtered: Vec<_> = all_profiles
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains(&search_lower))
        .collect();

    SaveAsProfileListTemplate {
        profiles: filtered,
    }
}

#[derive(Deserialize)]
pub struct SaveAsForm {
    name: String,
}

pub async fn handle_save_as_submit(
    axum::extract::State(state): axum::extract::State<AppState>,
    Form(form): Form<SaveAsForm>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile save-as submit requested: {}", form.name);

    let name = form.name.trim().to_string();

    if name.is_empty() {
        warn_targeted!(HTTP, "Cannot save motion profile: empty name");
        return ProfileFsOpResult {
            result: Err("Profile name cannot be empty".to_string()),
            close_modal: true,
            reload_metadata: false,
            retarget: None,
        }
    }

    let metadata = state.motion_profile_metadata.lock().await;
    let description = metadata.description.clone().unwrap_or_default();
    drop(metadata);

    let raw_profile = match RawMotionProfile::capture_from_memory(&state.clearcore_registers).await {
        Ok(profile) => profile,
        Err(e) => {
            error_targeted!(HTTP, "Failed to capture profile from memory: {}", e);
            return ProfileFsOpResult {
                result: Err("Failed to read current profile!".to_string()),
                close_modal: true,
                reload_metadata: false,
                retarget: None,
            }
        }
    };

    let profile = MotionProfile::new(name.clone(), description, raw_profile);

    match file_operations::save_profile(&profile).await {
        Ok(_) => {
            info_targeted!(HTTP, "Successfully saved motion profile as: {}", name);

            let mut metadata = state.motion_profile_metadata.lock().await;
            metadata.set_name(name.clone());
            drop(metadata);

            ProfileFsOpResult {
                result: Ok(format!("Saved as {}", name)),
                close_modal: true,
                reload_metadata: true,
                retarget: None,
            }
        }
        Err(e) => {
            error_targeted!(HTTP, "Failed to save motion profile {}: {}", name, e);
            ProfileFsOpResult {
                result: Ok(format!("Saved as {}", name)),
                close_modal: true,
                reload_metadata: true,
                retarget: None,
            }
        }
    }
}

pub async fn handle_load_modal() -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile load modal requested");
    LoadModalTemplate { base_url: BASE_URL }
}

#[derive(Deserialize)]
pub struct LoadPreviewQuery {
    name: String,
}

pub async fn handle_load_preview(
    Query(query): Query<LoadPreviewQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile load preview requested: {}", query.name);

    match file_operations::load_profile(&query.name).await {
        Ok(p) => {
            LoadPreviewTemplate {
                base_url: BASE_URL,
                result: Ok(LoadPreviewWindow {
                    name: p.name.clone(),
                    description: p.description.clone(),
                }),
            }
        },
        Err(e) => {
            error_targeted!(HTTP, "Failed to load profile {}: {}", query.name, e);
            LoadPreviewTemplate {
                base_url: BASE_URL,
                result: Err("Failed to load profile".to_string()),
            }

        }
    }
}

#[derive(Deserialize)]
pub struct LoadApplyQuery {
    name: String,
}

pub async fn handle_load_apply(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<LoadApplyQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Motion profile load apply requested: {}", query.name);

    let profile = match file_operations::load_profile(&query.name).await {
        Ok(p) => p,
        Err(e) => {
            error_targeted!(HTTP, "Failed to load profile {}: {}", query.name, e);
            return ProfileFsOpResult {
                result: Err("Failed to load profile from disk!".to_string()),
                close_modal: true,
                reload_metadata: true,
                retarget: None,
            };
        }
    };

    if let Err(e) = profile.raw_profile.apply_to_memory(&state.clearcore_registers).await {
        error_targeted!(HTTP, "Failed to apply profile {} to memory: {}", query.name, e);
        return ProfileFsOpResult {
            result: Err("Failed to apply profile to controller!".to_string()),
            close_modal: true,
            reload_metadata: true,
            retarget: None,
        };
    }

    let profile_name = profile.name.clone();

    let mut metadata = state.motion_profile_metadata.lock().await;
    metadata.set_name(profile.name);
    metadata.set_description(profile.description);
    drop(metadata);

    info_targeted!(HTTP, "Successfully loaded motion profile: {}", profile_name);

    ProfileFsOpResult {
        result: Ok(format!("Loaded {}", profile_name)),
        close_modal: true,
        reload_metadata: true,
        retarget: None,
    }
}

pub async fn handle_get_profile_list(
    Query(query): Query<LoadListQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Reloading motion profile list with search: {:?}", query.search);

    let result = match file_operations::list_profiles().await {
        Ok(all_profiles) => {
            let profiles = match &query.search {
                Some(search_term) if !search_term.trim().is_empty() => {
                    let search_lower = search_term.to_lowercase();
                    all_profiles
                        .into_iter()
                        .filter(|p| {
                            p.name.to_lowercase().contains(&search_lower) ||
                            p.description.to_lowercase().contains(&search_lower)
                        })
                        .collect()
                }
                _ => all_profiles,
            };
            Ok(profiles)
        }
        Err(e) => {
            error_targeted!(HTTP, "Failed to list profiles: {}", e);
            Err(format!("Failed to load profiles: {}", e))
        }
    };

    LoadProfileListTemplate { base_url: BASE_URL, result }
}

#[derive(Deserialize)]
pub struct DeleteProfileQuery {
    name: String,
}

pub async fn handle_delete_profile_confirm(
    Query(query): Query<DeleteProfileQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Delete confirmed for motion profile: {}", query.name);

    let result = file_operations::delete_profile(&query.name).await;

    match result {
        Ok(()) => {
            info_targeted!(HTTP, "Successfully deleted motion profile: {}", query.name);
            ProfileDeleteTemplate {
                name: query.name,
                result: Ok(()),
            }
        }
        Err(e) => {
            error_targeted!(HTTP, "Failed to delete motion profile {}: {}", query.name, e);
            ProfileDeleteTemplate {
                name: query.name,
                result: Err(e.to_string()),
            }
        }
    }
}
