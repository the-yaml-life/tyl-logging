//! Logger implementations
//!
//! This module contains different logger implementations that can output
//! log records in various formats.

use crate::record::LogRecord;

/// Port - Main logging interface that all loggers must implement
pub trait Logger {
    /// Log a record to the output destination
    fn log(&self, record: &LogRecord);
}

// Re-export logger implementations
pub mod console;
pub mod json;

pub use console::ConsoleLogger;
pub use json::JsonLogger;