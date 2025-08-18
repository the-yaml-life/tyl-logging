//! JSON logger implementation
//!
//! Provides structured JSON logging for production environments.

use super::Logger;
use crate::record::LogRecord;
use crate::utils::format_level;

/// Adapter - JSON structured logger for production
pub struct JsonLogger;

impl JsonLogger {
    /// Create a new JSON logger
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for JsonLogger {
    fn log(&self, record: &LogRecord) {
        let json_record = serde_json::json!({
            "timestamp": record.timestamp(),
            "level": format_level(record.level()),
            "message": record.message(),
            "fields": record.fields(),
            "request_id": record.request_id()
        });
        println!("{json_record}");
    }
}
