/// This module defines custom error types and utilities for handling errors in the application.
use std::path::PathBuf;

use thiserror::Error;

#[macro_export]
macro_rules! bail_on_error {
    ($exit_code:expr) => {{
        std::process::exit($exit_code);
    }};
}

// トップレベルカスタムエラー
#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AppError {
    #[error("CLI error> {0}")]
    Cli(#[from] CliErr),
    #[error("AnalysisConfig error> {0}")]
    AnalysisConfig(#[from] AnalysisConfigErr),
    #[error("Analysis error> {0}")]
    Analysis(#[from] AnalysisErr),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Cli(e) => e.exit_code(),
            Self::AnalysisConfig(e) => e.exit_code(),
            Self::Analysis(e) => e.exit_code(),
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum CliErr {
    #[error("Args validation error> {0}")]
    Validation(#[from] ArgsValidationErr),
}

impl CliErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 2,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AnalysisConfigErr {
    #[error("Analysis config validation error> {0}")]
    Validation(#[from] ConfigValidationErr),
    #[error("Analysis config parse error> {0}")]
    Parse(#[from] toml::de::Error),
    #[error("I/O error> {0}")]
    Io(#[from] IoErrWrapper),
}

impl AnalysisConfigErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Validation(_) => 3,
            Self::Parse(_) => 5,
            Self::Io(_) => 4,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ArgsValidationErr {
    #[error("File has no extension: '{0}'")]
    NoExtension(PathBuf),
    #[error("Invalid file extension: '{0}', expected one of: '{1}'")]
    InvalidExtension(String, String),
    #[error("Path does not exist: '{0}'")]
    PathDoesNotExist(PathBuf),
    #[error("Path is not a file: '{0}'")]
    PathIsNotFile(PathBuf),
    #[error("Path is not a directory: '{0}'")]
    PathIsNotDirectory(PathBuf),
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ConfigValidationErr {
    #[error("The extension is '{0}' even though the possible extensions for this From are '{1}'")]
    InvalidExtension(String, String),
    #[error("File has no extension: '{0}'")]
    NoExtension(PathBuf),
    #[error("Path does not exist: '{0}'")]
    PathDoesNotExist(PathBuf),
    #[error("Path is not a file: '{0}'")]
    PathIsNotFile(PathBuf),
    #[error("'{0}' does not require acc_axis but was set: name:'{1}', id:'{2}'")]
    MismatchedAccAxis(String, String, usize),
    #[error(
        "'{0}' format requires acc_axis to be 'ns, ew, ud' but was not set: name:'{1}', id:'{2}'"
    )]
    DuplicateAccAxis(String, String, usize),
    #[error("acc_axis does not exist: name:'{0}', id:'{1}'")]
    RequiredAccAxis(String, usize),
    #[error("Duplicate names, each NAME must be unique: '{0}'")]
    DuplicateNames(String),
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AnalysisErr {
    #[error("Data extraction error> {0}")]
    Extraction(#[from] DataExtractionErr),
}

impl AnalysisErr {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Extraction(_) => 6,
        }
    }
}

#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum DataExtractionErr {
    #[error("Invalid data structure: path{0}")]
    InvalidStructure(PathBuf),
}

// PartialEq, Eqの実装を行うための、std::io::ErrorをラップするカスタムI/Oエラー型
#[derive(Debug)]
pub struct IoErrWrapper(pub std::io::Error);

impl From<std::io::Error> for IoErrWrapper {
    fn from(err: std::io::Error) -> Self {
        IoErrWrapper(err)
    }
}

// IoErrWrapperにPartialEqを実装
impl PartialEq for IoErrWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.kind() == other.0.kind() && self.0.to_string() == other.0.to_string()
    }
}

// IoErrWrapperにEqを実装
impl Eq for IoErrWrapper {}

impl std::fmt::Display for IoErrWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I/O error> {}", self.0)
    }
}

// IoErrWrapperにErrorトレイトを実装（エラーをそのまま扱えるようにする）
impl std::error::Error for IoErrWrapper {}
