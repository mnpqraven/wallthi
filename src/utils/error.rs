use crate::command::state::AppState;
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("bruh idk some dumbshit happened")]
    General,
    #[error("File operation error")]
    Io(#[from] std::io::Error),

    #[error("Invalid configuration format")]
    ConfigFormat(#[from] toml::de::Error),

    #[error("Daemon error")]
    Daemon(#[from] daemonize::Error),

    #[error("RwLock is poisoned!")]
    Lock,

    #[error("Failed to serialize/deserialize data from/to bytes")]
    BytesSerde(#[from] serde_json::Error),
}

impl From<PoisonError<RwLockWriteGuard<'_, AppState>>> for AppError {
    fn from(_value: PoisonError<RwLockWriteGuard<'_, AppState>>) -> Self {
        Self::Lock
    }
}

impl From<PoisonError<RwLockReadGuard<'_, AppState>>> for AppError {
    fn from(_value: PoisonError<RwLockReadGuard<'_, AppState>>) -> Self {
        Self::Lock
    }
}
