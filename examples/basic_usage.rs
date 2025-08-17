use tyl_logging::{generate_request_id, ConsoleLogger, JsonLogger, LogLevel, LogRecord, Logger};

fn main() {
    println!("=== TYL Logging Basic Usage ===\n");

    // Basic console logging
    console_logging_example();

    // Structured JSON logging
    json_logging_example();

    // Request correlation
    request_correlation_example();
}

fn console_logging_example() {
    println!("--- Console Logging ---");

    let logger = ConsoleLogger::new();

    // Basic logging
    logger.log(&LogRecord::new(LogLevel::Info, "Service started"));
    logger.log(&LogRecord::new(LogLevel::Warn, "This is a warning"));
    logger.log(&LogRecord::new(LogLevel::Error, "An error occurred"));

    // Logging with fields (fields won't show in console format)
    let mut record = LogRecord::new(LogLevel::Info, "User action completed");
    record.add_field("user_id", serde_json::json!("user123"));
    record.add_field("action", serde_json::json!("login"));
    logger.log(&record);

    println!();
}

fn json_logging_example() {
    println!("--- JSON Structured Logging ---");

    let logger = JsonLogger::new();

    // Basic JSON logging
    logger.log(&LogRecord::new(LogLevel::Info, "Service started"));

    // Structured logging with fields
    let mut record = LogRecord::new(LogLevel::Info, "User action");
    record.add_field("user_id", serde_json::json!("user123"));
    record.add_field("action", serde_json::json!("create_post"));
    record.add_field("post_id", serde_json::json!("post456"));
    record.add_field(
        "metadata",
        serde_json::json!({
            "ip": "192.168.1.100",
            "user_agent": "TYL Client 1.0"
        }),
    );
    logger.log(&record);

    // Error with context
    let mut error_record = LogRecord::new(LogLevel::Error, "Database operation failed");
    error_record.add_field("error_code", serde_json::json!(500));
    error_record.add_field("component", serde_json::json!("database"));
    error_record.add_field("table", serde_json::json!("users"));
    error_record.add_field("operation", serde_json::json!("INSERT"));
    logger.log(&error_record);

    println!();
}

fn request_correlation_example() {
    println!("--- Request Correlation ---");

    let logger = JsonLogger::new();

    // Generate request ID for correlation
    let request_id = generate_request_id();
    println!("Request ID: {}", request_id);

    // Log multiple related operations with same request ID
    let operations = [
        "Request received",
        "Validating input",
        "Querying database",
        "Processing results",
        "Sending response",
    ];

    for operation in operations {
        let record = LogRecord::new(LogLevel::Info, operation).with_request_id(request_id.clone());
        logger.log(&record);
    }

    // Error in the same request context
    let error_record =
        LogRecord::new(LogLevel::Error, "Validation failed").with_request_id(request_id);
    logger.log(&error_record);
}
