//! Configuration module for the TYL logging framework
//!
//! This module provides configuration structures and environment detection
//! with integration to the TYL config plugin system.

use tyl_errors::TylError;
use tyl_config::{ConfigPlugin, ConfigResult};

/// Runtime environment for the service
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Environment {
    Development,
    Production,
    Test,
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

/// Configuration for logging setup with TYL config integration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    pub service_name: String,
    pub level: crate::record::LogLevel,
    pub environment: Environment,
}

impl LoggingConfig {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            level: crate::record::LogLevel::Info,
            environment: Environment::from_env(),
        }
    }

    pub fn with_level(mut self, level: crate::record::LogLevel) -> Self {
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
    
    pub fn level(&self) -> crate::record::LogLevel {
        self.level
    }
    
    pub fn environment(&self) -> Environment {
        self.environment.clone()
    }
}

impl ConfigPlugin for LoggingConfig {
    fn name(&self) -> &'static str {
        "logging"
    }

    fn env_prefix(&self) -> &'static str {
        "LOG"
    }

    fn validate(&self) -> ConfigResult<()> {
        if self.service_name.is_empty() {
            return Err(TylError::validation("service_name", "cannot be empty"));
        }
        Ok(())
    }

    fn from_env(&self) -> ConfigResult<Self> {
        let mut config = Self::new("app");
        config.merge_env()?;
        Ok(config)
    }

    fn merge_env(&mut self) -> ConfigResult<()> {
        // TYL_LOG_LEVEL or LOG_LEVEL
        if let Ok(level_str) = std::env::var("TYL_LOG_LEVEL")
            .or_else(|_| std::env::var("LOG_LEVEL"))
        {
            match level_str.to_uppercase().as_str() {
                "TRACE" => self.level = crate::record::LogLevel::Trace,
                "DEBUG" => self.level = crate::record::LogLevel::Debug,
                "INFO" => self.level = crate::record::LogLevel::Info,
                "WARN" | "WARNING" => self.level = crate::record::LogLevel::Warn,
                "ERROR" => self.level = crate::record::LogLevel::Error,
                _ => return Err(TylError::configuration(format!("invalid log level: {}", level_str))),
            }
        }

        // TYL_SERVICE_NAME or SERVICE_NAME
        if let Ok(service_name) = std::env::var("TYL_SERVICE_NAME")
            .or_else(|_| std::env::var("SERVICE_NAME"))
        {
            self.service_name = service_name;
        }

        // TYL_ENVIRONMENT or ENVIRONMENT
        if let Ok(env_str) = std::env::var("TYL_ENVIRONMENT")
            .or_else(|_| std::env::var("ENVIRONMENT"))
        {
            match env_str.to_lowercase().as_str() {
                "development" | "dev" => self.environment = Environment::Development,
                "production" | "prod" => self.environment = Environment::Production,
                "test" | "testing" => self.environment = Environment::Test,
                _ => return Err(TylError::configuration(format!("invalid environment: {}", env_str))),
            }
        }

        Ok(())
    }
}