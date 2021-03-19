use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoreError {
    NotFound(String),
    NotImplemented(String),
    InternalError,
}

impl CoreError {
    pub fn internal_error() -> Self {
        Self::InternalError
    }

    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}
