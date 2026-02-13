use std::collections::VecDeque;
use std::sync::LazyLock;
use axum::response::sse::Event;
use tokio::time::Instant;
use bytes::BytesMut;
use futures::StreamExt;
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, Mutex};
use tokio_util::codec::Decoder;
use tokio_util::udp::UdpFramed;
use crate::logging::LogTarget;
use crate::sse::{SseEvent, SseEventExt};
use crate::sse::error_toast::ErrorToast;

pub const MAX_LOG_ENTRIES: usize = 200;

pub static LOG_BUFFER: LazyLock<Mutex<VecDeque<ClearcoreLog>>> = LazyLock::new(|| {
    Mutex::new(VecDeque::with_capacity(MAX_LOG_ENTRIES))
});

// ANSI Color Tags
const ERROR_TAG: &[u8] = b"\x1b[31m[ERROR]\x1b[0m";
const WARN_TAG: &[u8] = b"\x1b[33m[WARN]\x1b[0m";
const INFO_TAG: &[u8] = b"\x1b[34m[INFO]\x1b[0m";
const DEBUG_TAG: &[u8] = b"\x1b[35m[DEBUG]\x1b[0m";
const TRACE_TAG: &[u8] = b"\x1b[37m[TRACE]\x1b[0m";

// Framing Constants
const SOH: u8 = 0x01; // Start of Heading
const STX: u8 = 0x02; // Start of Text
const ETX: u8 = 0x03; // End of Text

pub async fn start_listener(listen_port: u16, new_log_sender: broadcast::Sender<SseEvent>) {
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", listen_port)).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to bind UDP listener on port {}: {}", listen_port, e);
            return;
        }
    };

    log::info!("UDP Logging listener started on port {}", listen_port);

    let mut framed = UdpFramed::new(socket, ClearcoreLogDecoder);

    loop {
        match framed.next().await {
            Some(Ok((log_entry, _addr))) => {
                new_clearcore_log(log_entry.clone()).await;
                if log_entry.level == Some(log::Level::Error){
                    log::debug!("received clearcore error in UDP log listener: {{{}}} sending toast", &log_entry.message);
                    match new_log_sender.send(SseEvent::ErrorToast(ErrorToast{
                        msg: log_entry.message.clone()
                    })){
                        Ok(_) => {},
                        Err(broadcast::error::SendError(_)) => {
                            log::warn!("Failed to send error toast event to SSE client");
                        }
                    }
                }
                match new_log_sender.send(SseEvent::NewLog(log_entry)){
                    Ok(_) => {},
                    Err(broadcast::error::SendError(_)) => {
                        log::warn!("Failed to send new log event to SSE client");
                    }
                }
            }
            Some(Err(e)) => {
                log::warn!("Error decoding UDP log packet: {}", e);
            }
            None => break,
        }
    }
}


#[derive(Clone, Debug)]
pub struct ClearcoreLog {
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub level: Option<log::Level>,
    pub source_info: String,
    pub message: String,
}

impl SseEventExt for ClearcoreLog{
    fn as_axum_event(&self) -> Event {
        Event::default().event("clearcore-log").data(self.as_html_list_element())
    }
}

impl ClearcoreLog {

    pub fn log_level_as_list_element_class(log_level: Option<log::Level>) -> &'static str {
        match log_level {
            Some(log::Level::Error) => "clearcore-log-entry-error",
            Some(log::Level::Warn) => "clearcore-log-entry-warn",
            Some(log::Level::Info) => "clearcore-log-entry-info",
            Some(log::Level::Debug) => "clearcore-log-entry-debug",
            Some(log::Level::Trace) => "clearcore-log-entry-trace",
            None => "clearcore-log-entry-none",
        }
    }

    pub fn log_level_as_span_class(log_level: log::Level) -> &'static str {
        match log_level {
            log::Level::Error => "clearcore-log-level-error",
            log::Level::Warn => "clearcore-log-level-warn",
            log::Level::Info => "clearcore-log-level-info",
            log::Level::Debug => "clearcore-log-level-debug",
            log::Level::Trace => "clearcore-log-level-trace",
        }
    }
    pub fn log_level_as_span(log_level: log::Level) -> String {
        format!("<span class='{}'>[{log_level:<5}]</span>", ClearcoreLog::log_level_as_span_class(log_level))
    }
    pub fn as_html_list_element(&self) -> String {

        // TODO: Sanitize! I know cc won't send </span>, but still this is risky.

        let global_class = ClearcoreLog::log_level_as_list_element_class(self.level);
        let timestamp = self.timestamp;
        let level_span = self.level.map(|level| {
            ClearcoreLog::log_level_as_span(level)
        }).unwrap_or(String::new());
        let source_info = &self.source_info;
        let message: &str = &self.message;

        format!("\
            <li class='clearcore-log-entry {global_class}'>\
                <span class='clearcore-log-timestamp'>\
                    {timestamp} \
                </span>\
                <span class='clearcore-log-source'>\
                    {source_info} \
                </span> \
                {level_span} \
                <span class='clearcore-log-message'>\
                    {message}\
                </span>\
            </li>")
    }
}

struct ClearcoreLogDecoder;

/*
Example Log Line (New Format):
<SOH> <OPTIONAL_ANSI_TAG> <SOURCE_INFO> <STX> <MESSAGE> <ETX>
0000   c0 47 0e fc 1e 8d 24 15 10 b0 59 7f 08 00 45 00   .G....$...Y...E.
0010   00 70 12 98 00 00 ff 11 25 71 c0 a8 01 0e c0 a8   .p......%q......
0020   01 15 53 ac a4 55 00 5c e7 bc 01 1b 5b 33 34 6d   ..S..U.\....[34m
0030   5b 49 4e 46 4f 5d 1b 5b 30 6d 20 6d 61 69 6e 2e   [INFO].[0m main.
0040   63 70 70 02 20 4c 2e 34 35 33 20 73 74 61 74 65   cpp. L.453 state
0050   5f 6d 61 63 68 69 6e 65 5f 69 74 65 72 3a 20 57   _machine_iter: W
0060   61 69 74 69 6e 67 20 66 6f 72 20 63 6f 6e 66 69   aiting for confi
0070   67 20 74 6f 20 62 65 20 72 65 61 64 79 03         g to be ready.

*/

impl Decoder for ClearcoreLogDecoder {
    type Item = ClearcoreLog;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None);
        }

        let buf = &src[..];

        // 1. Validate Start of Heading (SOH)
        if buf.first() != Some(&SOH) {
            // Not a valid packet start, consume and ignore
            src.clear();
            return Ok(None);
        }

        let mut cursor = 1; // Skip SOH
        let remainder = &buf[cursor..];

        // 2. Identify Log Level and consume tag if present
        let (level, after_tag_bytes) = if remainder.starts_with(ERROR_TAG) {
            (Some(log::Level::Error), &remainder[ERROR_TAG.len()..])
        } else if remainder.starts_with(WARN_TAG) {
            (Some(log::Level::Warn), &remainder[WARN_TAG.len()..])
        } else if remainder.starts_with(INFO_TAG) {
            (Some(log::Level::Info), &remainder[INFO_TAG.len()..])
        } else if remainder.starts_with(DEBUG_TAG) {
            (Some(log::Level::Debug), &remainder[DEBUG_TAG.len()..])
        } else if remainder.starts_with(TRACE_TAG) {
            (Some(log::Level::Trace), &remainder[TRACE_TAG.len()..])
        } else {
            (None, remainder)
        };

        // 3. Find Start of Text (STX) to separate Source Info from Message
        let stx_pos = match after_tag_bytes.iter().position(|&b| b == STX) {
            Some(pos) => pos,
            None => {
                // Malformed: Missing STX
                src.clear();
                return Ok(None);
            }
        };

        let source_bytes = &after_tag_bytes[..stx_pos];
        let message_start = stx_pos + 1;
        let after_stx_bytes = &after_tag_bytes[message_start..];

        // 4. Find End of Text (ETX) to terminate Message
        let etx_pos = match after_stx_bytes.iter().position(|&b| b == ETX) {
            Some(pos) => pos,
            None => {
                // Malformed: Missing ETX
                src.clear();
                return Ok(None);
            }
        };

        let message_bytes = &after_stx_bytes[..etx_pos];

        // 5. Convert to Strings
        let source_info = String::from_utf8_lossy(source_bytes).trim().to_string();
        let message = String::from_utf8_lossy(message_bytes).trim().to_string();

        src.clear();

        Ok(Some(ClearcoreLog {
            timestamp: chrono::Local::now(),
            level,
            source_info,
            message,
        }))
    }
}

pub async fn new_clearcore_log(log: ClearcoreLog) {
    log::log!(target: LogTarget::Clearcore.into(),
        log.level.unwrap_or(log::Level::Info), "[{}] {}", log.source_info, log.message);

    let mut buffer = LOG_BUFFER.lock().await;
    if buffer.len() >= MAX_LOG_ENTRIES {
        buffer.pop_front();
    }
    buffer.push_back(log);
}