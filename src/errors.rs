/// This module defines custom error types and utilities for handling errors in the application.
use std::{io::ErrorKind, path::PathBuf};

use log::error;
use thiserror::Error;

#[macro_export]
macro_rules! exit_on_error {
    ($exit_code:expr) => {{
        std::process::exit($exit_code);
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomErrors {
    ValidationError(Vec<ValidationError>),
    AnalysisConfigError(Vec<AnalysisConfigError>),
}

impl CustomErrors {
    pub fn exit_code(&self) -> i32 {
        match self {
            CustomErrors::ValidationError(_) => 2,
            CustomErrors::AnalysisConfigError(_) => 3,
        }
    }

    pub fn log_errors(&self) {
        match self {
            CustomErrors::ValidationError(errors) => {
                for (index, error) in errors.iter().enumerate() {
                    error!("{}. ValidationError: {}", index + 1, error);
                }
            }
            CustomErrors::AnalysisConfigError(errors) => {
                for (index, error) in errors.iter().enumerate() {
                    error!("{}. ConfigFileError: {}", index + 1, error);
                }
            }
        }
    }
}

impl From<Vec<ValidationError>> for CustomErrors {
    fn from(errors: Vec<ValidationError>) -> Self {
        CustomErrors::ValidationError(errors)
    }
}

impl From<Vec<AnalysisConfigError>> for CustomErrors {
    fn from(errors: Vec<AnalysisConfigError>) -> Self {
        CustomErrors::AnalysisConfigError(errors)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Path '{0}' is not a file")]
    PathIsNotFile(PathBuf),

    #[error("Path '{0}' is not a directory")]
    PathIsNotDir(PathBuf),

    #[error("Invalid file extension '{0}'. Expected one of: {1}.")]
    InvalidFileExt(String, String),

    #[error("No file extension found")]
    ExtNotFound,

    #[error("I/O error occurred: {0}")]
    Io(#[from] CustomIoError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnalysisConfigError {
    #[error("Config file '{0}' has the wrong structure.")]
    MismatchFileStructure(PathBuf),

    #[error("Failed to parse: '{0}'")]
    FailedToParse(String),

    #[error("I/O error occurred: {0}")]
    Io(#[from] CustomIoError),
}

/// Custom I/O errors wrapping individual std::io::Error
#[derive(Debug, Error)]
pub enum CustomIoError {
    #[error("File not found: {path:?}")]
    NotFound { path: PathBuf },

    #[error("Permission denied for file: {path:?}")]
    PermissionDenied { path: PathBuf },

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Connection reset by peer")]
    ConnectionReset,

    #[error("Connection aborted")]
    ConnectionAborted,

    #[error("Not connected")]
    NotConnected,

    #[error("Address in use")]
    AddrInUse,

    #[error("Address not available")]
    AddrNotAvailable,

    #[error("Broken pipe")]
    BrokenPipe,

    #[error("Already exists: {path:?}")]
    AlreadyExists { path: PathBuf },

    #[error("Would block")]
    WouldBlock,

    #[error("Invalid input/output")]
    InvalidInput,

    #[error("Timed out")]
    TimedOut,

    #[error("Write zero")]
    WriteZero,

    #[error("Interrupted")]
    Interrupted,

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Other I/O error: {source}")]
    Other { source: std::io::Error },
}

impl From<(std::io::Error, PathBuf)> for CustomIoError {
    fn from((error, path): (std::io::Error, PathBuf)) -> Self {
        match error.kind() {
            ErrorKind::NotFound => CustomIoError::NotFound { path },
            ErrorKind::PermissionDenied => CustomIoError::PermissionDenied { path },
            ErrorKind::AlreadyExists => CustomIoError::AlreadyExists { path },
            ErrorKind::ConnectionRefused => CustomIoError::ConnectionRefused,
            ErrorKind::ConnectionReset => CustomIoError::ConnectionReset,
            ErrorKind::ConnectionAborted => CustomIoError::ConnectionAborted,
            ErrorKind::NotConnected => CustomIoError::NotConnected,
            ErrorKind::AddrInUse => CustomIoError::AddrInUse,
            ErrorKind::AddrNotAvailable => CustomIoError::AddrNotAvailable,
            ErrorKind::BrokenPipe => CustomIoError::BrokenPipe,
            ErrorKind::WouldBlock => CustomIoError::WouldBlock,
            ErrorKind::InvalidInput => CustomIoError::InvalidInput,
            ErrorKind::TimedOut => CustomIoError::TimedOut,
            ErrorKind::WriteZero => CustomIoError::WriteZero,
            ErrorKind::Interrupted => CustomIoError::Interrupted,
            ErrorKind::UnexpectedEof => CustomIoError::UnexpectedEof,
            _ => CustomIoError::Other { source: error },
        }
    }
}

impl PartialEq for CustomIoError {
    fn eq(&self, other: &Self) -> bool {
        use CustomIoError::*;
        match (self, other) {
            (NotFound { path: p1 }, NotFound { path: p2 }) => p1 == p2,
            (PermissionDenied { path: p1 }, PermissionDenied { path: p2 }) => p1 == p2,
            (ConnectionRefused, ConnectionRefused) => true,
            (ConnectionReset, ConnectionReset) => true,
            (ConnectionAborted, ConnectionAborted) => true,
            (NotConnected, NotConnected) => true,
            (AddrInUse, AddrInUse) => true,
            (AddrNotAvailable, AddrNotAvailable) => true,
            (BrokenPipe, BrokenPipe) => true,
            (AlreadyExists { path: p1 }, AlreadyExists { path: p2 }) => p1 == p2,
            (WouldBlock, WouldBlock) => true,
            (InvalidInput, InvalidInput) => true,
            (TimedOut, TimedOut) => true,
            (WriteZero, WriteZero) => true,
            (Interrupted, Interrupted) => true,
            (UnexpectedEof, UnexpectedEof) => true,
            (Other { source: s1 }, Other { source: s2 }) => s1.kind() == s2.kind(),
            _ => false,
        }
    }
}

impl Eq for CustomIoError {}

/// This module contains unit tests for the error handling functionality.
///
/// # Tests
///
/// - `test_custom_io_error_from_io_error`: Tests conversion from std::io::Error to CustomIoError
/// - `test_custom_errors_exit_code`: Tests that CustomErrors returns correct exit code
/// - `test_custom_errors_log_errors`: Tests error logging functionality
/// - `test_validation_error_display`: Tests display formatting of ValidationError
/// - `test_custom_io_error_partial_eq`: Tests equality comparison of CustomIoError variants
/// - `test_custom_io_error_other` : Confirmation that the Other variant of CustomIoError is correctly generated when the source of std::io::error is Other.
#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_custom_io_error_from_io_error() {
        let path = PathBuf::from("/some/path");
        let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
        let custom_error: CustomIoError = (io_error, path.clone()).into();
        assert_eq!(custom_error, CustomIoError::NotFound { path });
    }

    #[test]
    fn test_custom_errors_exit_code() {
        let errors = vec![ValidationError::ExtNotFound];
        let custom_error = CustomErrors::from(errors);
        assert_eq!(custom_error.exit_code(), 2);
    }

    #[test]
    fn test_custom_errors_log_errors() {
        let errors = vec![ValidationError::ExtNotFound];
        let custom_error = CustomErrors::from(errors);
        custom_error.log_errors();
        // Check the logs manually or use a logging test framework
    }

    #[test]
    fn test_validation_error_display() {
        let path = PathBuf::from("/some/path");
        let error = ValidationError::PathIsNotFile(path.clone());
        assert_eq!(
            format!("{}", error),
            format!("Path '{}' is not a file", path.display())
        );
    }

    #[test]
    fn test_custom_io_error_partial_eq() {
        let path1 = PathBuf::from("/some/path1");
        let path2 = PathBuf::from("/some/path2");
        let error1 = CustomIoError::NotFound {
            path: path1.clone(),
        };
        let error2 = CustomIoError::NotFound {
            path: path1.clone(),
        };
        let error3 = CustomIoError::NotFound {
            path: path2.clone(),
        };
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_custom_io_error_other() {
        let io_error = io::Error::new(ErrorKind::Other, "other error");
        let custom_error: CustomIoError = (io_error, PathBuf::new()).into();
        if let CustomIoError::Other { source } = custom_error {
            assert_eq!(source.kind(), ErrorKind::Other);
        } else {
            panic!("Expected CustomIoError::Other");
        }
    }
}
