use crate::plc::plc_register_definitions;
use crate::AppState;
use crate::{info_targeted, warn_targeted};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, State};
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;

#[derive(Template, WebTemplate)]
#[template(path = "components/estop.html")]
pub struct EstopControl {
    pub in_estop: Option<bool>,
    pub requesting_estop: bool,
}

#[derive(Deserialize)]
pub struct EstopRequestForm {
    requesting_estop: bool,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ui/estop", get(estop_component))
        .route("/ui/estop/request", post(set_estop_request))
}

async fn estop_component(State(state): State<AppState>) -> EstopControl {
    let (in_estop, requesting_estop) = read_estop_state(&state).await;
    build_estop_control(in_estop, requesting_estop)
}

async fn set_estop_request(
    State(state): State<AppState>,
    Form(form): Form<EstopRequestForm>,
) -> EstopControl {
    info_targeted!(
        HTTP,
        "POST /ui/estop/request - requesting_estop: {}",
        form.requesting_estop
    );

    let requested = form.requesting_estop;
    let write_ok = match state
        .clearcore_registers
        .write_coil(plc_register_definitions::FORCE_ESTOP.address.address, requested)
        .await
    {
        Ok(()) => {
            info_targeted!(MODBUS, "Set FORCE_ESTOP to {}", requested);
            true
        }
        Err(e) => {
            warn_targeted!(MODBUS, "Failed to set FORCE_ESTOP to {}: {:?}", requested, e);
            false
        }
    };

    let (in_estop, mut requesting_estop) = read_estop_state(&state).await;
    if write_ok {
        requesting_estop = requested;
    }
    build_estop_control(in_estop, requesting_estop)
}

async fn read_estop_state(state: &AppState) -> (Option<bool>, bool) {
    let in_estop =
        state
            .clearcore_registers
            .read_disc(plc_register_definitions::IN_ESTOP.address.address)
            .await;
    if in_estop.is_none() {
        warn_targeted!(MODBUS, "Failed to read IN_ESTOP state");
    }

    let requesting_estop = match state
        .clearcore_registers
        .read_coil(plc_register_definitions::FORCE_ESTOP.address.address)
        .await
    {
        Some(value) => value,
        None => {
            warn_targeted!(MODBUS, "Failed to read FORCE_ESTOP state; assuming false");
            false
        }
    };

    (in_estop, requesting_estop)
}

fn build_estop_control(in_estop: Option<bool>, requesting_estop: bool) -> EstopControl {
    EstopControl {
        in_estop,
        requesting_estop,
    }
}
