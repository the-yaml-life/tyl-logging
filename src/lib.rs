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

// Module declarations
pub mod config;
pub mod record;
pub mod loggers;
pub mod utils;

// TYL Framework imports
use tyl_errors::TylResult;

/// Result type for logging operations using unified TYL error handling
pub type LoggingResult<T> = TylResult<T>;

// Re-exports for public API
pub use config::{LoggingConfig, Environment};
pub use record::{LogRecord, LogLevel};
pub use loggers::{Logger, ConsoleLogger, JsonLogger};
pub use utils::generate_request_id;

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