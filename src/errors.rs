use std::{io::ErrorKind, path::PathBuf};

use thiserror::Error;

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
