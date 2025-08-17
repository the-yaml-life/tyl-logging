//! Log record types and utilities
//!
//! This module contains the core logging data structures including
//! log levels, records, and related helper functions.

use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log severity levels in order of importance
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

/// A structured log record containing all log information
#[derive(Debug, Clone)]
pub struct LogRecord {
    level: LogLevel,
    message: String,
    timestamp: u64,
    fields: HashMap<String, Value>,
    request_id: Option<String>,
}

impl LogRecord {
    /// Create a new log record with the given level and message
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            fields: HashMap::new(),
            request_id: None,
        }
    }

    /// Get the log level
    pub fn level(&self) -> LogLevel {
        self.level
    }
    
    /// Get the log message
    pub fn message(&self) -> &str {
        &self.message
    }
    
    /// Get the timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    
    /// Get the additional fields
    pub fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    /// Add a field to the log record
    pub fn add_field(&mut self, key: impl Into<String>, value: Value) {
        self.fields.insert(key.into(), value);
    }

    /// Add a request ID to the log record
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Get the request ID if present
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}