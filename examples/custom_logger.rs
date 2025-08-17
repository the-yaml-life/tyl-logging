use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tyl_logging::{LogLevel, LogRecord, Logger};

/// Example of a custom logger that stores logs in memory
/// This demonstrates the extensibility of the hexagonal architecture
#[derive(Debug)]
struct MemoryLogger {
    logs: Arc<Mutex<Vec<StoredLog>>>,
}

#[derive(Debug, Clone)]
struct StoredLog {
    level: LogLevel,
    message: String,
    timestamp: u64,
    fields: HashMap<String, serde_json::Value>,
}

impl MemoryLogger {
    fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_logs(&self) -> Vec<StoredLog> {
        self.logs.lock().unwrap().clone()
    }

    fn clear(&self) {
        self.logs.lock().unwrap().clear();
    }

    fn count(&self) -> usize {
        self.logs.lock().unwrap().len()
    }

    fn filter_by_level(&self, level: LogLevel) -> Vec<StoredLog> {
        self.logs
            .lock()
            .unwrap()
            .iter()
            .filter(|log| log.level == level)
            .cloned()
            .collect()
    }
}

impl Logger for MemoryLogger {
    fn log(&self, record: &LogRecord) {
        let stored_log = StoredLog {
            level: record.level(),
            message: record.message().to_string(),
            timestamp: record.timestamp(),
            fields: record.fields().clone(),
        };

        self.logs.lock().unwrap().push(stored_log);
    }
}

/// Example of a filtering logger that only logs certain levels
struct FilteringLogger<L: Logger> {
    inner: L,
    min_level: LogLevel,
}

impl<L: Logger> FilteringLogger<L> {
    fn new(inner: L, min_level: LogLevel) -> Self {
        Self { inner, min_level }
    }
}

impl<L: Logger> Logger for FilteringLogger<L> {
    fn log(&self, record: &LogRecord) {
        if record.level() >= self.min_level {
            self.inner.log(record);
        }
    }
}

fn main() {
    println!("=== Custom Logger Examples ===\n");

    memory_logger_example();
    filtering_logger_example();
}

fn memory_logger_example() {
    println!("--- Memory Logger ---");

    let logger = MemoryLogger::new();

    // Log various messages
    logger.log(&LogRecord::new(LogLevel::Info, "Service started"));
    logger.log(&LogRecord::new(LogLevel::Warn, "Low disk space"));
    logger.log(&LogRecord::new(
        LogLevel::Error,
        "Database connection failed",
    ));

    let mut structured_record = LogRecord::new(LogLevel::Info, "User login");
    structured_record.add_field("user_id", serde_json::json!("user123"));
    structured_record.add_field("ip", serde_json::json!("192.168.1.1"));
    logger.log(&structured_record);

    // Demonstrate memory logger capabilities
    println!("Total logs: {}", logger.count());
    println!(
        "Error logs: {}",
        logger.filter_by_level(LogLevel::Error).len()
    );
    println!(
        "Warning logs: {}",
        logger.filter_by_level(LogLevel::Warn).len()
    );

    // Display all logs
    println!("\nStored logs:");
    for (i, log) in logger.get_logs().iter().enumerate() {
        println!(
            "  {}: [{:?}] {} (timestamp: {})",
            i + 1,
            log.level,
            log.message,
            log.timestamp
        );
        if !log.fields.is_empty() {
            println!("      Fields: {:?}", log.fields);
        }
    }

    logger.clear();
    println!("After clear: {} logs", logger.count());
    println!();
}

fn filtering_logger_example() {
    println!("--- Filtering Logger ---");

    let memory_logger = MemoryLogger::new();
    let filtering_logger = FilteringLogger::new(memory_logger, LogLevel::Warn);

    // Log messages at different levels
    filtering_logger.log(&LogRecord::new(LogLevel::Trace, "Trace message (filtered)"));
    filtering_logger.log(&LogRecord::new(LogLevel::Debug, "Debug message (filtered)"));
    filtering_logger.log(&LogRecord::new(LogLevel::Info, "Info message (filtered)"));
    filtering_logger.log(&LogRecord::new(LogLevel::Warn, "Warning message (passed)"));
    filtering_logger.log(&LogRecord::new(LogLevel::Error, "Error message (passed)"));

    // The memory logger should only have warnings and errors
    println!(
        "Logs that passed filter: {}",
        filtering_logger.inner.count()
    );

    for log in filtering_logger.inner.get_logs() {
        println!("  [{:?}] {}", log.level, log.message);
    }
}
