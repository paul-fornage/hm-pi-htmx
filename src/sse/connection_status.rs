use axum::response::sse::Event;
use serde::Serialize;

use crate::modbus::ModbusState;
use crate::sse::SseEventExt;

#[derive(Clone, Debug, Serialize)]
pub struct ConnectionStatus {
    pub connection: &'static str,
    pub state: &'static str,
}

impl ConnectionStatus {
    pub fn new(connection: &'static str, state: ModbusState) -> Self {
        Self {
            connection,
            state: state.to_str(),
        }
    }
}

impl SseEventExt for ConnectionStatus {
    const EVENT_TAG: &'static str = "connection-status";
    fn as_axum_event(&self) -> Event {
        let data = serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string());
        Event::default().event(Self::EVENT_TAG).data(data)
    }
}
