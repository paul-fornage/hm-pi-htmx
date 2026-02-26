use axum::response::sse::Event;
use serde::Serialize;

use crate::modbus::ModbusState;
use crate::sse::SseEventExt;

#[derive(Clone, Debug, Serialize)]
pub struct EstopStateUpdate {}


impl SseEventExt for EstopStateUpdate {

    const EVENT_TAG: &'static str = "estop-state-update";
    fn as_axum_event(&self) -> Event {
        Event::default().event(Self::EVENT_TAG)
    }
}
