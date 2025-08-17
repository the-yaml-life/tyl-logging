//! Console logger implementation
//!
//! Provides a simple console logger for development and debugging.

use super::Logger;
use crate::record::LogRecord;
use crate::utils::{format_timestamp, format_level};

/// Adapter - Simple console logger for development
pub struct ConsoleLogger;

impl ConsoleLogger {
    /// Create a new console logger
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, record: &LogRecord) {
        println!(
            "[{}] {}: {}",
            format_timestamp(record.timestamp()),
            format_level(record.level()),
            record.message()
        );
    }
}