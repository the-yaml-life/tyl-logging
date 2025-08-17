# tyl-logging

Simple structured logging library for the TYL framework with hexagonal architecture.

## Features

- **Hexagonal Architecture** - Clean ports and adapters pattern
- **Multiple Output Formats** - Console and JSON structured logging  
- **Environment Detection** - Automatic dev/prod configuration
- **Request Correlation** - UUID-based request tracking
- **Extensible** - Easy to add custom loggers without modifying core
- **Zero Dependencies** - Minimal runtime dependencies

## Quick Start

```rust
use tyl_logging::{Logger, ConsoleLogger, LogLevel, LogRecord};

let logger = ConsoleLogger::new();
let record = LogRecord::new(LogLevel::Info, "Hello, world!");
logger.log(&record);
```

## Structured Logging

```rust
use tyl_logging::{Logger, JsonLogger, LogLevel, LogRecord};
use serde_json::json;

let logger = JsonLogger::new();
let mut record = LogRecord::new(LogLevel::Info, "User login");
record.add_field("user_id", json!("user123"));
record.add_field("ip", json!("192.168.1.1"));
logger.log(&record);
```

## Architecture

### Port (Interface)
```rust
trait Logger {
    fn log(&self, record: &LogRecord);
}
```

### Built-in Adapters
- **`ConsoleLogger`** - Human-readable output for development
- **`JsonLogger`** - Structured JSON output for production

### Custom Adapters
Easy to implement custom loggers:

```rust
struct MyLogger;

impl Logger for MyLogger {
    fn log(&self, record: &LogRecord) {
        // Your custom implementation
    }
}
```

## Request Correlation

```rust
use tyl_logging::{generate_request_id, LogRecord, LogLevel};

let request_id = generate_request_id();
let record = LogRecord::new(LogLevel::Info, "Processing request")
    .with_request_id(request_id);
```

## Environment Configuration

```rust
use tyl_logging::{LoggingConfig, Environment, LogLevel};

let config = LoggingConfig::new("my-service")
    .with_level(LogLevel::Debug)
    .with_environment(Environment::Production);
```

## Examples

Run examples:
```bash
cargo run --example basic_usage
cargo run --example custom_logger
```

## Testing

```bash
cargo test
```