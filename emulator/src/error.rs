//! Error types for iOSS Emulator

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Device error: {0}")]
    Device(String),

    #[error("App error: {0}")]
    App(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Framework error: {0}")]
    Framework(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Timeout")]
    Timeout,

    #[error("Interrupted")]
    Interrupted,
}

pub type Result<T> = std::result::Result<T, Error>;
