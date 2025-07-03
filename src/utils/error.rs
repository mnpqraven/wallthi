use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("File operation error")]
    Io(#[from] std::io::Error),

    #[error("Invalid configuration format")]
    ConfigFormat(#[from] toml::de::Error),
}
