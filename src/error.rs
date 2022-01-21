//! Common error types for this crate.
use once_cell::sync::OnceCell;
use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt;
use std::num::NonZeroU16;

/// Handy wrapper against [`Error`]s.
pub type Result<T, E = Error> = std::result::Result<T, E>;
pub(crate) type UntaggedResult<T, E = Error> = crate::utils::UntaggedResult<T, E>;

/// Bad things that could happen when calling the `textsynth` API.
#[derive(Clone, Eq, PartialEq, Deserialize)]
pub struct Error {
    status: NonZeroU16,
    error: String,

    #[serde(skip)]
    status_code: OnceCell<StatusCode>,
}

impl Error {
    /// Returns the HTTP status code associated with this error.
    pub fn status_code(&self) -> StatusCode {
        *self.status_code.get_or_init(|| {
            StatusCode::from_u16(self.status.get()).expect("invalid status code from error")
        })
    }

    /// Returns the message associated with this error.
    pub fn message(&self) -> &str {
        &self.error
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_code = self.status_code();
        let message = self.message();
        write!(f, "{status_code}, {message}")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("status_code", &self.status_code())
            .field("error", &self.message())
            .finish()
    }
}

impl StdError for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::ops::Deref;

    static ERROR: Lazy<Error> = Lazy::new(|| Error {
        status: NonZeroU16::new(400).unwrap(),
        error: "Bad Request".to_string(),
        status_code: OnceCell::new(),
    });

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", ERROR.deref()), "400 Bad Request, Bad Request");
    }

    #[test]
    fn test_error_debug() {
        assert_eq!(
            format!("{:?}", ERROR.deref()),
            "Error { status_code: 400, error: \"Bad Request\" }"
        );
    }

    #[test]
    fn test_status_code() {
        let _ = ERROR.status_code();
    }

    #[test]
    fn test_message() {
        let _ = ERROR.message();
    }
}
