use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Form;
use serde::Deserialize;
use log::{error, info};

use crate::{debug_targeted, warn_targeted, AppState};
use super::file_operations;
use super::weld_profile::WeldProfile;
use super::raw_weld_profile::RawWeldProfile;
use super::file_system_templates::{
    SaveStatusTemplate, SaveAsModalTemplate, SaveAsProfileListTemplate,
    LoadModalTemplate, LoadPreviewTemplate, DeleteButtonTemplate, LoadProfileListTemplate,
};
use super::file_system_response::{FileSystemResponse, render_template};

pub async fn handle_save(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Save profile requested");

    let metadata = state.weld_profile_metadata.lock().await;
    let profile_name = match &metadata.name {
        Some(name) => name.clone(),
        None => {
            warn_targeted!(HTTP, "Cannot save profile: no name set");
            return render_template(SaveStatusTemplate {
                success: false,
                message: "No profile name set".to_string(),
            });
        }
    };

    let description = metadata.description.clone().unwrap_or_default();
    drop(metadata);

    let raw_profile = match RawWeldProfile::capture_from_memory(&state.miller_registers).await {
        Ok(profile) => profile,
        Err(e) => {
            error!("Failed to capture profile from memory: {}", e);
            return render_template(SaveStatusTemplate {
                success: false,
                message: "Failed to read from welder".to_string(),
            });
        }
    };

    let profile = WeldProfile::new(profile_name.clone(), description, raw_profile);

    match file_operations::save_profile(&profile).await {
        Ok(_) => {
            info!("Successfully saved profile: {}", profile_name);
            render_template(SaveStatusTemplate {
                success: true,
                message: "Saved".to_string(),
            })
        }
        Err(e) => {
            error!("Failed to save profile {}: {}", profile_name, e);
            render_template(SaveStatusTemplate {
                success: false,
                message: "Save failed".to_string(),
            })
        }
    }
}

pub async fn handle_save_as_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Save As modal requested");

    let profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error!("Failed to list profiles: {}", e);
            return FileSystemResponse::Error("Failed to load profile list".to_string());
        }
    };

    let metadata = state.weld_profile_metadata.lock().await;
    let current_name = metadata.name.clone();
    drop(metadata);

    render_template(SaveAsModalTemplate {
        current_name,
        profiles,
    })
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn handle_save_as_search(
    Query(query): Query<SearchQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Save As search requested: {}", query.q);

    let all_profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error!("Failed to list profiles: {}", e);
            return render_template(SaveAsProfileListTemplate {
                profiles: Vec::new(),
            });
        }
    };

    let search_lower = query.q.to_lowercase();
    let filtered: Vec<_> = all_profiles
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains(&search_lower))
        .collect();

    render_template(SaveAsProfileListTemplate {
        profiles: filtered,
    })
}

#[derive(Deserialize)]
pub struct SaveAsForm {
    name: String,
}

pub async fn handle_save_as_submit(
    axum::extract::State(state): axum::extract::State<AppState>,
    Form(form): Form<SaveAsForm>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Save As submit requested: {}", form.name);

    let name = form.name.trim().to_string();

    if name.is_empty() {
        warn_targeted!(HTTP, "Cannot save profile: empty name");
        return FileSystemResponse::Error("Name cannot be empty".to_string());
    }

    let metadata = state.weld_profile_metadata.lock().await;
    let description = metadata.description.clone().unwrap_or_default();
    drop(metadata);

    let raw_profile = match RawWeldProfile::capture_from_memory(&state.miller_registers).await {
        Ok(profile) => profile,
        Err(e) => {
            error!("Failed to capture profile from memory: {}", e);
            return FileSystemResponse::Error("Failed to read from welder".to_string());
        }
    };

    let profile = WeldProfile::new(name.clone(), description, raw_profile);

    match file_operations::save_profile(&profile).await {
        Ok(_) => {
            info!("Successfully saved profile as: {}", name);

            let mut metadata = state.weld_profile_metadata.lock().await;
            metadata.set_name(name.clone());
            drop(metadata);

            FileSystemResponse::SuccessStatus(format!("Saved as {}", name))
        }
        Err(e) => {
            error!("Failed to save profile {}: {}", name, e);
            FileSystemResponse::Error("Save failed".to_string())
        }
    }
}

pub async fn handle_load_modal() -> impl IntoResponse {
    debug_targeted!(HTTP, "Load modal requested");

    let profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error!("Failed to list profiles: {}", e);
            return FileSystemResponse::Error("Failed to load profile list".to_string());
        }
    };

    render_template(LoadModalTemplate { profiles })
}

#[derive(Deserialize)]
pub struct LoadPreviewQuery {
    name: String,
}

pub async fn handle_load_preview(
    Query(query): Query<LoadPreviewQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Load preview requested: {}", query.name);

    let profile = match file_operations::load_profile(&query.name).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to load profile {}: {}", query.name, e);
            return FileSystemResponse::Error("Failed to load profile".to_string());
        }
    };

    render_template(LoadPreviewTemplate {
        name: profile.name,
        description: profile.description,
    })
}

#[derive(Deserialize)]
pub struct LoadApplyQuery {
    name: String,
}

pub async fn handle_load_apply(
    axum::extract::State(state): axum::extract::State<AppState>,
    Query(query): Query<LoadApplyQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Load apply requested: {}", query.name);

    let profile = match file_operations::load_profile(&query.name).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to load profile {}: {}", query.name, e);
            return FileSystemResponse::Error("Failed to load profile from disk".to_string());
        }
    };

    if let Err(e) = profile.raw_profile.apply_to_memory(&state.miller_registers).await {
        error!("Failed to apply profile {} to memory: {}", query.name, e);
        return FileSystemResponse::Error(format!("Failed to write to welder: {}", e));
    }

    let profile_name = profile.name.clone();

    let mut metadata = state.weld_profile_metadata.lock().await;
    metadata.set_name(profile.name);
    metadata.set_description(profile.description);
    drop(metadata);

    info!("Successfully loaded profile: {}", profile_name);

    FileSystemResponse::SuccessStatus(format!("Loaded {}", profile_name))
}

#[derive(Deserialize)]
pub struct DeleteProfileQuery {
    name: String,
}

pub async fn handle_delete_profile_button(
    Query(query): Query<DeleteProfileQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Delete button clicked for: {}", query.name);

    render_template(DeleteButtonTemplate {
        profile_name: query.name,
        confirm_mode: true,
    })
}

pub async fn handle_delete_profile_confirm(
    Query(query): Query<DeleteProfileQuery>,
) -> impl IntoResponse {
    debug_targeted!(HTTP, "Delete confirmed for: {}", query.name);

    let result = file_operations::delete_profile(&query.name).await;

    match result {
        Ok(()) => info!("Successfully deleted profile: {}", query.name),
        Err(ref e) => error!("Failed to delete profile {}: {}", query.name, e),
    }

    let profiles = match file_operations::list_profiles().await {
        Ok(list) => list,
        Err(e) => {
            error!("Failed to reload profile list: {}", e);
            return FileSystemResponse::Error("Failed to reload profile list".to_string());
        }
    };

    render_template(LoadProfileListTemplate { profiles })
}
