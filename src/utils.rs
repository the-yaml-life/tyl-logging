//! Utility functions for logging
//!
//! This module contains helper functions for formatting and ID generation.

use uuid::Uuid;
use crate::record::LogLevel;

/// Generate a new request ID for correlation
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Format a timestamp as a string
pub fn format_timestamp(timestamp: u64) -> String {
    // Simple timestamp formatting
    format!("{timestamp}")
}

/// Format a log level as a string
pub fn format_level(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "TRACE",
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    }
}