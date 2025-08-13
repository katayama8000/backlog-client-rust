use serde::{Deserialize, Serialize};
use thiserror::Error;

/// https://developer.nulab.com/ja/docs/backlog/error-response/
#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorResponse: ...")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub struct Error {
    pub message: String,
    pub code: u32,
    pub more_info: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {{ code: {}, message: {}, more_info: {} }}",
            self.code, self.message, self.more_info
        )
    }
}
