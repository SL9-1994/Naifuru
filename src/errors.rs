use std::{io::ErrorKind, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
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

/// 個々のstd::io::ErrorをラップするカスタムI/Oエラー
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
