use axum::response::sse::Event;
use serde::Serialize;

use crate::sse::SseEventExt;

#[derive(Clone, Debug, Serialize)]
pub struct EstopStateUpdate {
    pub in_estop: Option<bool>,
}


impl SseEventExt for EstopStateUpdate {

    const EVENT_TAG: &'static str = "estop-state-update";
    fn as_axum_event(&self) -> Event {
        let payload = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        Event::default()
            .event(Self::EVENT_TAG)
            .data(payload)
    }
}
