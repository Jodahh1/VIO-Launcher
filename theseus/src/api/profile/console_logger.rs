//! Встроенный консольный логгер - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub timestamp: i64,
    pub level: LogLevel,
    pub logger: String,
    pub message: String,
    pub raw: String,
}

#[derive(Debug, Clone)]
pub struct ConsoleLogger {
    pub is_running: bool,
    pub message_count: usize,
    pub last_message: Option<i64>,
}

pub fn init_log_channel() -> tokio::sync::broadcast::Sender<LogMessage> {
    let (tx, _rx) = tokio::sync::broadcast::channel(1000);
    tx
}

pub fn subscribe_to_logs() -> tokio::sync::broadcast::Receiver<LogMessage> {
    let (tx, rx) = tokio::sync::broadcast::channel(1000);
    rx
}

pub fn emit_log_message(_message: LogMessage) {}

pub async fn start_log_monitor(_profile_path: &ProfilePathId) -> crate::Result<ConsoleLogger> {
    Ok(ConsoleLogger {
        is_running: false,
        message_count: 0,
        last_message: None,
    })
}

pub async fn get_recent_logs(_profile_path: &ProfilePathId, _limit: usize) -> crate::Result<Vec<LogMessage>> {
    Ok(Vec::new())
}

pub async fn clear_logs(_profile_path: &ProfilePathId) -> crate::Result<usize> {
    Ok(0)
}

pub async fn scan_for_errors(_profile_path: &ProfilePathId) -> crate::Result<Vec<LogMessage>> {
    Ok(Vec::new())
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use uuid::Uuid;

    pub async fn emit_logs_cleared(_profile_uuid: &Uuid, _count: usize) -> crate::Result<()> {
        emit_event("logs:cleared", json!({})).await
    }

    pub async fn emit_error_detected(_profile_uuid: &Uuid, _error_count: usize) -> crate::Result<()> {
        emit_event("logs:error_detected", json!({})).await
    }
}
