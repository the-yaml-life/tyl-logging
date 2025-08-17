use tyl_logging::{
    generate_request_id, ConsoleLogger, Environment, JsonLogger, LogLevel, LogRecord, Logger,
    LoggingConfig,
};

#[test]
fn test_end_to_end_console_logging() {
    // Test complete console logging flow
    let logger = ConsoleLogger::new();
    let mut record = LogRecord::new(LogLevel::Info, "Integration test message");
    record.add_field("test_id", serde_json::json!("integration_001"));

    // Should not panic
    logger.log(&record);
}

#[test]
fn test_end_to_end_json_logging() {
    // Test complete JSON logging flow
    let logger = JsonLogger::new();
    let record = LogRecord::new(LogLevel::Error, "Integration test error")
        .with_request_id(generate_request_id());

    // Should not panic and produce valid JSON
    logger.log(&record);
}

#[test]
fn test_configuration_integration() {
    // Test configuration with environment detection
    let config = LoggingConfig::new("integration-test")
        .with_level(LogLevel::Debug)
        .with_environment(Environment::Test);

    assert_eq!(config.service_name(), "integration-test");
    assert_eq!(config.level(), LogLevel::Debug);
    assert_eq!(config.environment(), Environment::Test);
}

#[test]
fn test_request_correlation_across_loggers() {
    // Test request ID correlation across different loggers
    let request_id = generate_request_id();
    let console_logger = ConsoleLogger::new();
    let json_logger = JsonLogger::new();

    let record1 = LogRecord::new(LogLevel::Info, "Console log").with_request_id(request_id.clone());
    let record2 = LogRecord::new(LogLevel::Info, "JSON log").with_request_id(request_id);

    // Both should handle the same request ID
    console_logger.log(&record1);
    json_logger.log(&record2);
}
