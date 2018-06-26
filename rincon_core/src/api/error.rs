use std::fmt;

pub use arango::ErrorCode;

/// The `api::Error` is returned by functions of the rincon driver whenever an
/// error is returned by the ArangoDB server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Fail)]
pub struct Error {
    #[serde(rename = "code")]
    status_code: u16,
    #[serde(rename = "errorNum")]
    error_code: ErrorCode,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    /// Creates a new `Error` with the given status code, error code and
    /// message.
    pub fn new<M>(status_code: u16, error_code: ErrorCode, message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            status_code,
            error_code,
            message: message.into(),
        }
    }

    /// Returns the status code of this error.
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Returns the error code of this error.
    pub fn error_code(&self) -> ErrorCode {
        self.error_code
    }

    /// Returns the message of this error.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!(
            "Error {}: {} (Status: {})",
            &self.error_code.as_u16(),
            &self.message,
            &self.status_code
        ))
    }
}
