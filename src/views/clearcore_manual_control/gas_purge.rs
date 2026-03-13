use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crate::miller::miller_register_definitions as miller_regs;
use crate::modbus::cached_modbus::CachedModbus;
use crate::sse::error_toast::ErrorToast;
use crate::{error_targeted, AppState};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, State};
use axum::response::Html;
use serde::Deserialize;

use super::helpers::read_bool;

const GAS_PURGE_MIN_SECONDS: f64 = 0.0;
const GAS_PURGE_MAX_SECONDS: f64 = 30.0;
const GAS_PURGE_DEFAULT_SECONDS: f64 = 5.0;
const GAS_PURGE_STEP_SECONDS: f64 = 0.1;
const GAS_PURGE_WRITE_INTERVAL: Duration = Duration::from_millis(500);
const GAS_PURGE_READBACK_TIMEOUT: Duration = Duration::from_millis(500);
const GAS_PURGE_READBACK_POLL: Duration = Duration::from_millis(50);

static GAS_PURGE_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/gas-purge-modal.html")]
pub(super) struct GasPurgeModalTemplate {
    post_url: String,
    feedback_target: String,
    min_seconds: String,
    max_seconds: String,
    step_seconds: String,
    prefill_seconds: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "components/clearcore-manual-control/gas-purge-feedback.html")]
pub(super) struct GasPurgeFeedbackTemplate {
    result: Result<String, String>,
}

impl GasPurgeFeedbackTemplate {
    fn running(duration_seconds: f64) -> Self {
        Self {
            result: Ok(format!("{duration_seconds:.1}")),
        }
    }

    fn err(message: String) -> Self {
        Self { result: Err(message) }
    }
}

#[derive(Deserialize)]
pub(super) struct GasPurgeForm {
    duration_seconds: String,
}

pub(super) async fn gas_purge_modal_handler() -> Html<String> {
    let html = GasPurgeModalTemplate {
        post_url: "/clearcore-manual-control/gas-purge".to_string(),
        feedback_target: "#gas-purge-feedback".to_string(),
        min_seconds: format!("{GAS_PURGE_MIN_SECONDS:.1}"),
        max_seconds: format!("{GAS_PURGE_MAX_SECONDS:.1}"),
        step_seconds: format!("{GAS_PURGE_STEP_SECONDS:.1}"),
        prefill_seconds: format!("{GAS_PURGE_DEFAULT_SECONDS:.1}"),
    }
    .render()
    .unwrap();
    Html(html)
}

pub(super) async fn gas_purge_submit_handler(
    State(state): State<AppState>,
    Form(form): Form<GasPurgeForm>,
) -> GasPurgeFeedbackTemplate {
    let duration_seconds = match form.duration_seconds.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return GasPurgeFeedbackTemplate::err("Invalid number format.".to_string()),
    };

    if !(GAS_PURGE_MIN_SECONDS..=GAS_PURGE_MAX_SECONDS).contains(&duration_seconds) {
        return GasPurgeFeedbackTemplate::err(
            "Purge time must be between 0 and 30 seconds.".to_string(),
        );
    }

    if GAS_PURGE_ACTIVE.swap(true, Ordering::SeqCst) {
        return GasPurgeFeedbackTemplate::err("Gas purge already running.".to_string());
    }

    let miller_registers = state.miller_registers.clone();
    let sse_tx = state.sse_tx.clone();
    tokio::spawn(async move {
        struct GasPurgeGuard;
        impl Drop for GasPurgeGuard {
            fn drop(&mut self) {
                GAS_PURGE_ACTIVE.store(false, Ordering::SeqCst);
            }
        }

        let _guard = GasPurgeGuard;
        if let Err(err) = gas_purge(&miller_registers, duration_seconds).await {
            if sse_tx
                .send(
                    ErrorToast {
                        msg: format!("Gas purge failed: {err}"),
                    }
                    .into(),
                )
                .is_err()
            {
                error_targeted!(HTTP, "Failed to send gas purge error toast");
            }
        }
    });

    GasPurgeFeedbackTemplate::running(duration_seconds)
}

async fn gas_purge(
    miller_registers: &CachedModbus,
    duration_seconds: f64,
) -> Result<(), String> {
    let duration = Duration::from_secs_f64(duration_seconds);
    let mut wrote_true = false;

    let result = 'purge: loop {
        match miller_registers
            .write_coil(miller_regs::GAS_REQUEST.address.address, true)
            .await
        {
            Ok(()) => wrote_true = true,
            Err(err) => {
                break Err(format!("Failed to write GAS_REQUEST true: {err}"));
            }
        }

        let start = Instant::now();

        if duration < Duration::from_secs(1) {
            tokio::time::sleep(duration).await;
            break Ok(());
        }

        let readback_deadline = start + GAS_PURGE_READBACK_TIMEOUT;
        loop {
            match read_bool(miller_registers, &miller_regs::GAS_OUTPUT_ENABLED.address).await {
                Ok(true) => break,
                Ok(false) => {}
                Err(err) => break 'purge Err(err),
            }

            if Instant::now() >= readback_deadline {
                break 'purge Err("Timed out waiting for GAS_OUTPUT_ENABLED".to_string());
            }

            tokio::time::sleep(GAS_PURGE_READBACK_POLL).await;
        }

        let mut next_write = start + GAS_PURGE_WRITE_INTERVAL;

        loop {
            if start.elapsed() >= duration {
                break 'purge Ok(());
            }

            match read_bool(miller_registers, &miller_regs::GAS_OUTPUT_ENABLED.address).await {
                Ok(true) => {}
                Ok(false) => {
                    break 'purge Err("GAS_OUTPUT_ENABLED went false during purge".to_string());
                }
                Err(err) => break 'purge Err(err),
            }

            let now = Instant::now();
            if now >= next_write {
                if let Err(err) = miller_registers
                    .write_coil(miller_regs::GAS_REQUEST.address.address, true)
                    .await
                {
                    break 'purge Err(format!("Failed to refresh GAS_REQUEST: {err}"));
                }
                next_write += GAS_PURGE_WRITE_INTERVAL;
            }

            tokio::time::sleep(GAS_PURGE_READBACK_POLL).await;
        }
    };

    if wrote_true {
        if let Err(err) = miller_registers
            .write_coil(miller_regs::GAS_REQUEST.address.address, false)
            .await
        {
            return Err(match result {
                Ok(()) => format!("Failed to write GAS_REQUEST false: {err}"),
                Err(existing) => {
                    format!("{existing}. Failed to write GAS_REQUEST false: {err}")
                }
            });
        }
    }

    result
}
