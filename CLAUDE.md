# CLAUDE.md - tyl-logging

## 📋 **Module Context**

**tyl-logging** is the structured logging module for the TYL framework. It follows hexagonal architecture with simple ports and adapters for extensible logging.

## 🏗️ **Architecture**

### **Port (Interface)**
```rust
trait Logger {
    fn log(&self, record: &LogRecord);
}
```

### **Adapters (Implementations)**
- `ConsoleLogger` - Human-readable console output
- `JsonLogger` - Structured JSON output

### **Core Types**
- `LogRecord` - Main log data structure
- `LogLevel` - Severity levels (Trace, Debug, Info, Warn, Error)
- `LoggingConfig` - Configuration with builder pattern
- `Environment` - Runtime environment detection

## 🧪 **Testing**

```bash
cargo test -p tyl-logging
cargo test --doc -p tyl-logging
cargo run --example basic_usage -p tyl-logging
cargo run --example custom_logger -p tyl-logging
```

## 📂 **File Structure**

```
tyl-logging/
├── src/lib.rs                 # Core implementation (~400 lines)
├── examples/
│   ├── basic_usage.rs         # Console and JSON logging
│   └── custom_logger.rs       # Custom logger implementations
├── README.md                  # Main documentation
├── CLAUDE.md                  # This file
└── Cargo.toml                 # Package metadata
```

## 🔧 **How to Use**

### **Basic Logging**
```rust
use tyl_logging::{Logger, ConsoleLogger, LogLevel, LogRecord};

let logger = ConsoleLogger::new();
logger.log(&LogRecord::new(LogLevel::Info, "Service started"));
```

### **Structured Logging**
```rust
use tyl_logging::{JsonLogger, LogRecord, LogLevel};
use serde_json::json;

let logger = JsonLogger::new();
let mut record = LogRecord::new(LogLevel::Info, "User action");
record.add_field("user_id", json!("user123"));
record.add_field("action", json!("login"));
logger.log(&record);
```

### **Request Correlation**
```rust
use tyl_logging::{generate_request_id, LogRecord, LogLevel};

let request_id = generate_request_id();
let record = LogRecord::new(LogLevel::Info, "Processing")
    .with_request_id(request_id);
```

### **Custom Logger**
```rust
struct MemoryLogger {
    logs: Vec<String>,
}

impl Logger for MemoryLogger {
    fn log(&self, record: &LogRecord) {
        // Custom implementation
    }
}
```

## 🛠️ **Useful Commands**

```bash
cargo clippy -p tyl-logging
cargo fmt -p tyl-logging  
cargo doc --no-deps -p tyl-logging --open
cargo test -p tyl-logging --verbose
```

## 📦 **Dependencies**

### **Runtime**
- `serde` - Serialization for structured data
- `serde_json` - JSON support for fields
- `uuid` - Request ID generation
- `thiserror` - Error handling

## 🎯 **Design Principles**

1. **Hexagonal Architecture** - Clean separation of ports and adapters
2. **Simplicity** - Minimal API surface, easy to understand
3. **Extensibility** - Easy to add custom loggers
4. **Performance** - Lightweight with minimal allocations
5. **Structured Data** - First-class support for key-value fields

## ⚠️ **Known Limitations**

- Only stdout output (no file logging yet)
- Basic timestamp formatting
- No log level filtering in core (implement in custom loggers)

## 📝 **Notes for Contributors**

- Follow TDD approach
- Keep the Logger trait simple
- Add custom functionality via adapter pattern
- Document all public APIs with examples
- Maintain backwards compatibility