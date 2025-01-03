/// This module defines custom error types and utilities for handling errors in the application.
use std::fmt;

#[macro_export]
macro_rules! bail_on_error {
    ($exit_code:expr) => {{
        std::process::exit($exit_code);
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    CliArgsValidation,
    ConfigFileAnalysis,
}

impl Module {
    pub fn exit_code(&self) -> i32 {
        match self {
            Module::CliArgsValidation => 1,
            Module::ConfigFileAnalysis => 2,
        }
    }
}

#[derive(Debug)]
pub struct ErrorContext {
    pub message: String,
    pub module: Module,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
