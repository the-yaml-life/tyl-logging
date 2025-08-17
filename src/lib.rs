//! # TYL Logging
//!
//! A simple structured logging library for the TYL framework following hexagonal architecture.
//!
//! This crate provides a clean interface for logging with multiple output formats,
//! allowing easy extension without modifying core functionality.
//!
//! ## Features
//!
//! - **Hexagonal Architecture**: Clean ports and adapters pattern
//! - **Multiple Formats**: Console and structured JSON logging
//! - **Environment Detection**: Automatic dev/prod configuration
//! - **Request Correlation**: UUID-based request tracking
//! - **TDD Approach**: Test-driven development from day one
//!
//! ## Quick Start
//!
//! ```rust
//! use tyl_logging::{Logger, ConsoleLogger, LogLevel, LogRecord};
//!
//! let logger = ConsoleLogger::new();
//! let record = LogRecord::new(LogLevel::Info, "Hello, world!");
//! logger.log(&record);
//! ```
//!
//! ## Structured Logging
//!
//! ```rust
//! use tyl_logging::{Logger, JsonLogger, LogLevel, LogRecord};
//! use serde_json::json;
//!
//! let logger = JsonLogger::new();
//! let mut record = LogRecord::new(LogLevel::Info, "User login");
//! record.add_field("user_id", json!("user123"));
//! record.add_field("ip", json!("192.168.1.1"));
//! logger.log(&record);
//! ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_record_creation_should_work() {
        // Given: log level and message
        // When: creating log record
        let record = LogRecord::new(LogLevel::Info, "Test message");

        // Then: should create valid record
        assert_eq!(record.level(), LogLevel::Info);
        assert_eq!(record.message(), "Test message");
        assert!(record.timestamp() > 0);
    }

    #[test]
    fn test_log_record_with_fields_should_store_metadata() {
        // Given: log record
        // When: adding fields
        let mut record = LogRecord::new(LogLevel::Error, "Database error");
        record.add_field("error_code", serde_json::json!(500));
        record.add_field("component", serde_json::json!("database"));

        // Then: should store fields
        assert_eq!(record.fields().len(), 2);
        assert_eq!(record.fields()["error_code"], serde_json::json!(500));
        assert_eq!(record.fields()["component"], serde_json::json!("database"));
    }

    #[test]
    fn test_console_logger_should_output_readable_format() {
        // Given: console logger and log record
        // When: logging a message
        let logger = ConsoleLogger::new();
        let record = LogRecord::new(LogLevel::Warn, "Test warning");

        // Then: should not panic (we can't easily test stdout)
        logger.log(&record);
    }

    #[test]
    fn test_json_logger_should_output_structured_format() {
        // Given: JSON logger and log record with fields
        // When: logging structured data
        let logger = JsonLogger::new();
        let mut record = LogRecord::new(LogLevel::Info, "User action");
        record.add_field("user_id", serde_json::json!("user123"));
        record.add_field("action", serde_json::json!("login"));

        // Then: should not panic (we can't easily test stdout)
        logger.log(&record);
    }

    #[test]
    fn test_logging_config_should_detect_environment() {
        // Given: no environment override
        // When: creating config
        let config = LoggingConfig::new("test-service");

        // Then: should have valid defaults
        assert_eq!(config.service_name(), "test-service");
        assert!(matches!(
            config.environment(),
            Environment::Development | Environment::Production | Environment::Test
        ));
    }

    #[test]
    fn test_logging_config_builder_pattern_should_work() {
        // Given: config builder
        // When: using builder pattern
        let config = LoggingConfig::new("my-service")
            .with_level(LogLevel::Debug)
            .with_environment(Environment::Production);

        // Then: should apply configuration
        assert_eq!(config.service_name(), "my-service");
        assert_eq!(config.level(), LogLevel::Debug);
        assert_eq!(config.environment(), Environment::Production);
    }

    #[test]
    fn test_request_id_generation_should_be_unique() {
        // Given: request ID generation
        // When: generating multiple IDs
        let id1 = generate_request_id();
        let id2 = generate_request_id();

        // Then: should be unique and valid UUIDs
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 36); // UUID v4 length
        assert_eq!(id2.len(), 36);
    }

    #[test]
    fn test_log_level_ordering_should_work() {
        // Given: different log levels
        // When: comparing levels
        // Then: should follow expected ordering
        assert!(LogLevel::Error > LogLevel::Warn);
        assert!(LogLevel::Warn > LogLevel::Info);
        assert!(LogLevel::Info > LogLevel::Debug);
        assert!(LogLevel::Debug > LogLevel::Trace);
    }

    #[test]
    fn test_logger_trait_allows_custom_implementations() {
        // Given: custom logger implementation
        struct TestLogger {
            pub messages: std::cell::RefCell<Vec<String>>,
        }

        impl Logger for TestLogger {
            fn log(&self, record: &LogRecord) {
                self.messages
                    .borrow_mut()
                    .push(record.message().to_string());
            }
        }

        // When: using custom logger
        let logger = TestLogger {
            messages: std::cell::RefCell::new(Vec::new()),
        };
        let record = LogRecord::new(LogLevel::Info, "Test message");
        logger.log(&record);

        // Then: should capture messages
        assert_eq!(logger.messages.borrow().len(), 1);
        assert_eq!(logger.messages.borrow()[0], "Test message");
    }
}

// Implementation starts here - all tests will fail initially (TDD red phase)
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Result type for logging operations
pub type LoggingResult<T> = Result<T, LoggingError>;

/// Errors that can occur during logging operations  
#[derive(Debug, thiserror::Error)]
pub enum LoggingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Configuration error: {message}")]
    Configuration { message: String },
}

/// Port - Main logging interface that all loggers must implement
pub trait Logger {
    /// Log a record to the output destination
    fn log(&self, record: &LogRecord);
}

/// Log severity levels in order of importance
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

/// Runtime environment for the service
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
    Test,
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

/// Configuration for logging setup
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    service_name: String,
    level: LogLevel,
    environment: Environment,
}

// Temporary implementations that will fail tests (TDD red phase)
impl LogRecord {
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

    pub fn level(&self) -> LogLevel {
        self.level
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    pub fn add_field(&mut self, key: impl Into<String>, value: Value) {
        self.fields.insert(key.into(), value);
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

/// Adapter - Simple console logger for development
pub struct ConsoleLogger;

impl ConsoleLogger {
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

/// Adapter - JSON structured logger for production
pub struct JsonLogger;

impl JsonLogger {
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
            "request_id": record.request_id
        });
        println!("{json_record}");
    }
}

impl LoggingConfig {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            level: LogLevel::Info,
            environment: Environment::from_env(),
        }
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }
    pub fn level(&self) -> LogLevel {
        self.level
    }
    pub fn environment(&self) -> Environment {
        self.environment.clone()
    }
}

impl Environment {
    pub fn from_env() -> Self {
        match std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" | "prod" => Environment::Production,
            "test" | "testing" => Environment::Test,
            _ => Environment::Development,
        }
    }
}

/// Generate a new request ID for correlation
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

fn format_timestamp(timestamp: u64) -> String {
    // Simple timestamp formatting
    format!("{timestamp}")
}

fn format_level(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Trace => "TRACE",
        LogLevel::Debug => "DEBUG",
        LogLevel::Info => "INFO",
        LogLevel::Warn => "WARN",
        LogLevel::Error => "ERROR",
    }
}
