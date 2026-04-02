use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("{0}")]
    Message(String),
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("TOML deserialize error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Binary '{product}' not found. Run: ever install {product}")]
    ProductNotInstalled { product: String },
}

pub type RouterResult<T> = Result<T, RouterError>;
