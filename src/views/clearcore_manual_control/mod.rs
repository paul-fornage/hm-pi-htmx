mod finger;
mod gas_purge;
mod go_to_position;
mod helpers;
mod jog;
mod manual_control;

use axum::routing::{get, post};
use axum::Router;

use crate::views::shared::finger_status::finger_status_handler;
use crate::views::AppView;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            AppView::ClearcoreManualControl.url(),
            get(manual_control::show_manual_control),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/home-axes"),
            post(manual_control::home_all_axes_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/homing-status"),
            get(manual_control::homing_status_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/finger-status/{side}"),
            get(finger_status_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/finger/{side}/{action}"),
            post(finger::finger_command_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/status-feedback"),
            get(manual_control::manual_control_status_feedback),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/x-position"),
            get(manual_control::get_x_position_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/y-position"),
            get(manual_control::get_y_position_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/z-position"),
            get(manual_control::get_z_position_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"),
            get(jog::jog_speed_modal_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/jog-speed/{axis}"),
            post(jog::jog_speed_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/jog-speed-display/{axis}"),
            get(jog::jog_speed_display_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/jog/{axis}/{direction}"),
            post(jog::jog_command_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/relative-move/{axis}"),
            get(go_to_position::relative_move_modal_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/relative-move/{axis}"),
            post(go_to_position::relative_move_submit_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/gas-purge"),
            get(gas_purge::gas_purge_modal_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/gas-purge"),
            post(gas_purge::gas_purge_submit_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/go-to-position/{axis}"),
            get(go_to_position::go_to_position_modal_handler),
        )
        .route(
            &AppView::ClearcoreManualControl.url_with_path("/go-to-position/{axis}"),
            post(go_to_position::go_to_position_submit_handler),
        )
}
