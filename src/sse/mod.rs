pub mod connection_status;
pub mod error_toast;
mod new_estop_state;

use std::convert::Infallible;
use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::{Event, KeepAlive};
use futures::{Stream, stream};
use tokio::sync::broadcast;
use crate::{trace_targeted, warn_targeted, AppState};
use crate::sse::connection_status::ConnectionStatus;
use crate::sse::error_toast::ErrorToast;
pub(crate) use crate::sse::new_estop_state::EstopStateUpdate;
use crate::udp_log_listener::ClearcoreLog;

#[derive(Clone, Debug)]
pub enum SseEvent {
    ErrorToast(ErrorToast),
    NewLog(ClearcoreLog),
    ConnectionStatus(ConnectionStatus),
    NewEstopState(EstopStateUpdate)
}

impl From<ErrorToast> for SseEvent { fn from(value: ErrorToast) -> Self { Self::ErrorToast(value) } }
impl From<ClearcoreLog> for SseEvent { fn from(value: ClearcoreLog) -> Self { Self::NewLog(value) } }
impl From<ConnectionStatus> for SseEvent { fn from(value: ConnectionStatus) -> Self { Self::ConnectionStatus(value) } }
impl From<EstopStateUpdate> for SseEvent { fn from(value: EstopStateUpdate) -> Self { Self::NewEstopState(value) } }

pub trait SseEventExt {
    const EVENT_TAG: &'static str;
    fn as_axum_event(&self) -> Event;
}

impl SseEvent {
    pub fn as_axum_event(&self) -> Event{
        match self {
            SseEvent::ErrorToast(err) => err.as_axum_event(),
            SseEvent::NewLog(log) => log.as_axum_event(),
            SseEvent::ConnectionStatus(evt) => evt.as_axum_event(),
            SseEvent::NewEstopState(estop) => estop.as_axum_event(),
        }
    }

    pub fn get_tag(&self) -> &'static str {
        match self {
            SseEvent::ErrorToast(_) => ErrorToast::EVENT_TAG,
            SseEvent::NewLog(_) => ClearcoreLog::EVENT_TAG,
            SseEvent::ConnectionStatus(_) => ConnectionStatus::EVENT_TAG,
            SseEvent::NewEstopState(_) => EstopStateUpdate::EVENT_TAG,
        }
    }
}




// SSE handler
pub async fn sse_handler(State(state): State<AppState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    log::info!("SSE handler started");
    let rx = state.sse_tx.subscribe();

    // Use unfold to create a stream from the broadcast receiver
    let stream = stream::unfold(rx, |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(evt) => {
                    // Return the event and the receiver for the next iteration
                    let html_sse_evt = evt.as_axum_event();
                    trace_targeted!(HTTP, "SSE event received in http context: {}", evt.get_tag());
                    return Some((Ok(html_sse_evt), rx));
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {
                    warn_targeted!(HTTP, "SSE channel lagged, skipping missed messages");
                    continue;
                }
                Err(broadcast::error::RecvError::Closed) => {
                    return None;
                }
            }
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}
