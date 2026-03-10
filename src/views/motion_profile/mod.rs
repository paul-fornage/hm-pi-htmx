pub mod profile_metadata;
pub mod raw_motion_profile;
pub mod motion_profile;
pub mod file_operations;
pub mod file_system_handlers;
pub mod file_system_templates;

use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get, post};
use axum::{Form, Router};
use serde::Deserialize;

use crate::AppState;
use crate::plc::plc_register_definitions;
use crate::views::{AppView, HeaderContext, ViewTemplate, build_header_context};
use crate::views::shared::{
    AnalogEditModalTemplate,
    BooleanEditModalTemplate,
    EditableAnalogRegister,
    EditableBooleanRegister,
    WriteErrorModalTemplate,
    mb_read_bool_helper,
    mb_read_word_helper,
};
use crate::views::shared::analog_register::AnalogRegisterInfo;
use crate::views::shared::boolean_register::BooleanRegisterInfo;
use crate::views::welder_profile::description_edit_modal::DescriptionEditModalTemplate;
use crate::views::welder_profile::ProfileMetadataDisplayTemplate;
use crate::views::motion_profile::file_system_handlers::{
    handle_delete_profile_confirm,
    handle_get_profile_list,
    handle_load_apply,
    handle_load_modal,
    handle_load_preview,
    handle_save,
    handle_save_as_modal,
    handle_save_as_search,
    handle_save_as_submit,
};

pub const BASE_URL: &str = "/motion-profile";

pub fn routes() -> Router<AppState> {
    let page = AppView::MotionProfile;
    Router::new()
        .route(page.url(), get(show_motion_profile))
        .route(&page.url_with_path("/grid"), get(show_motion_profile_grid))
        .route(&page.url_with_path("/metadata"), get(show_profile_metadata))
        .route(&page.url_with_path("/edit/{register_name}"), get(show_edit_modal))
        .route(&page.url_with_path("/write/{register_name}"), post(submit_register_write))
        .route(&page.url_with_path("/edit-description"), get(show_description_edit_modal))
        .route(&page.url_with_path("/update-description"), post(update_description))
        .route(&page.url_with_path("/fs/save"), get(handle_save))
        .route(&page.url_with_path("/fs/save_as"), get(handle_save_as_modal))
        .route(&page.url_with_path("/fs/save_as/search"), post(handle_save_as_search))
        .route(&page.url_with_path("/fs/load/list"), get(handle_get_profile_list))
        .route(&page.url_with_path("/fs/save_as/submit"), post(handle_save_as_submit))
        .route(&page.url_with_path("/fs/load"), get(handle_load_modal))
        .route(&page.url_with_path("/fs/load/preview"), get(handle_load_preview))
        .route(&page.url_with_path("/fs/load/apply"), post(handle_load_apply))
        .route(&page.url_with_path("/fs/load/delete_confirm"), delete(handle_delete_profile_confirm))
}

const MOTION_PROFILE_BOOLEAN_REGISTERS: [BooleanRegisterInfo; 2] = [
    BooleanRegisterInfo::new_default(&plc_register_definitions::CYCLE_USE_AVC),
    BooleanRegisterInfo::new_default(&plc_register_definitions::CYCLE_USE_TOUCH_RETRACT),
];

pub(crate) const MOTION_PROFILE_ANALOG_REGISTERS: [AnalogRegisterInfo; 14] = [
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_START_POS, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_END_POS, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_PARK_POS, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_WELD_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_REPOSITION_SPEED_X, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_REPOSITION_SPEED_Y, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_REPOSITION_SPEED_Z, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_WIRE_FEED_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_AVC_VREF, "V", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_Z_STATIC_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_AXIS_Z_TORCH_UP_OFFSET, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_TOUCH_RETRACT_REPOSITION_DISTANCE, "in", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_TOUCH_RETRACT_PROBE_SPEED, "in/min", 2, 0),
    AnalogRegisterInfo::new(&plc_register_definitions::CYCLE_TOUCH_RETRACT_FINAL_HEIGHT, "in", 2, 0),

];

pub async fn show_motion_profile(State(state): State<AppState>) -> impl IntoResponse {
    let header = build_header_context(&state, AppView::MotionProfile).await;
    MotionProfileTemplate { header }
}

pub async fn show_motion_profile_grid(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let mut boolean_registers = Vec::new();
    for info in MOTION_PROFILE_BOOLEAN_REGISTERS.iter() {
        let value = mb_read_bool_helper(&state.clearcore_registers, &info.meta.address).await;
        boolean_registers.push(EditableBooleanRegister {
            register_info: info,
            value,
            base_url: BASE_URL,
        });
    }

    let mut analog_registers = Vec::new();
    for info in MOTION_PROFILE_ANALOG_REGISTERS.iter() {
        let value = mb_read_word_helper(&state.clearcore_registers, &info.meta.address).await;
        analog_registers.push(EditableAnalogRegister {
            register_info: info,
            value,
            base_url: BASE_URL,
        });
    }

    MotionProfileGridTemplate {
        boolean_registers,
        analog_registers,
    }
}

pub async fn show_profile_metadata(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let metadata_lock = state.motion_profile_metadata.lock().await;
    ProfileMetadataDisplayTemplate {
        base_url: BASE_URL,
        name: metadata_lock.name.clone(),
        description: metadata_lock.description.clone(),
    }
}

pub async fn show_description_edit_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    let metadata_lock = state.motion_profile_metadata.lock().await;
    let current_description = metadata_lock.description.clone();

    DescriptionEditModalTemplate {
        base_url: BASE_URL,
        current_description,
    }
}

pub async fn update_description(
    axum::extract::State(state): axum::extract::State<AppState>,
    Form(form): Form<DescriptionUpdateForm>,
) -> impl IntoResponse {
    let mut metadata_lock = state.motion_profile_metadata.lock().await;

    if form.description.trim().is_empty() {
        metadata_lock.description = None;
    } else {
        metadata_lock.set_description(form.description.trim().to_string());
    }

    let response = ProfileMetadataDisplayTemplate {
        base_url: BASE_URL,
        name: metadata_lock.name.clone(),
        description: metadata_lock.description.clone(),
    };

    response
}

#[derive(Deserialize)]
pub struct DescriptionUpdateForm {
    description: String,
}

pub async fn show_edit_modal(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(register_name): Path<String>,
) -> impl IntoResponse {
    if let Some(info) = find_boolean_register(&register_name) {
        let current_value = mb_read_bool_helper(&state.clearcore_registers, &info.meta.address).await;
        let template = BooleanEditModalTemplate {
            register_info: info,
            current_value,
            register_name,
            base_url: BASE_URL,
        };
        return Html(template.render().unwrap());
    }

    if let Some(info) = find_analog_register(&register_name) {
        let current_value = mb_read_word_helper(&state.clearcore_registers, &info.meta.address).await;
        let template = AnalogEditModalTemplate {
            register_info: info,
            current_value,
            register_name,
            base_url: BASE_URL,
        };
        return Html(template.render().unwrap());
    }

    Html("<div>Error: Register not found</div>".to_string())
}

#[derive(Deserialize)]
pub struct RawWriteForm {
    value: String,
}

pub async fn submit_register_write(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(register_name): Path<String>,
    Form(form): Form<RawWriteForm>,
) -> impl IntoResponse {
    let render_error = |msg: &str| -> Html<String> {
        let t = WriteErrorModalTemplate {
            title: "Write Failed".to_string(),
            message: msg.to_string(),
        };
        Html(t.render().unwrap())
    };

    if let Some(info) = find_boolean_register(&register_name) {
        let value = form.value == "true";
        match state.clearcore_registers.write_coil(info.meta.address.address, value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    if let Some(info) = find_analog_register(&register_name) {
        let val_f32 = match form.value.parse::<f32>() {
            Ok(v) => v,
            Err(_) => {
                return render_error("Invalid number format provided.").into_response();
            }
        };

        if let Err(msg) = info.validate_semantic_value(val_f32) {
            return render_error(&msg).into_response();
        }

        let raw_value = info.convert_to_raw(val_f32);
        match state.clearcore_registers.write_hreg(info.meta.address.address, raw_value).await {
            Ok(_) => return Html("".to_string()).into_response(),
            Err(e) => return render_error(&e.to_string()).into_response(),
        }
    }

    Html("<div>Error: Register not found</div>".to_string()).into_response()
}

fn find_boolean_register(name: &str) -> Option<&'static BooleanRegisterInfo> {
    MOTION_PROFILE_BOOLEAN_REGISTERS
        .iter()
        .find(|info| info.meta.name == name)
}

fn find_analog_register(name: &str) -> Option<&'static AnalogRegisterInfo> {
    MOTION_PROFILE_ANALOG_REGISTERS
        .iter()
        .find(|info| info.meta.name == name)
}

#[derive(Template, WebTemplate)]
#[template(path = "views/motion-profile.html")]
pub struct MotionProfileTemplate {
    pub header: HeaderContext,
}
impl ViewTemplate for MotionProfileTemplate { const APP_VIEW_VARIANT: AppView = AppView::MotionProfile; }

#[derive(Template, WebTemplate)]
#[template(path = "components/motion-profile/motion-profile-grid.html")]
pub struct MotionProfileGridTemplate {
    pub boolean_registers: Vec<EditableBooleanRegister>,
    pub analog_registers: Vec<EditableAnalogRegister>,
}
