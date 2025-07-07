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
}
