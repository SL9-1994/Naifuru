use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, ValueHint};

use crate::{
    error::{ErrorContext, Module},
    logging::LogLevel,
};

const ERROR_MODULE: Module = Module::CliArgsValidation;

/// This module defines the command-line interface (CLI) for the application using the `clap` crate.
/// It includes the `Args` struct which represents the parsed command-line arguments and provides
/// methods for validation of these arguments.
///
/// # Structs
///
/// - `Args`: Represents the command-line arguments and provides methods for parsing and validation.
///
/// # Methods
///
/// - `Args::new() -> Self`: Parses the command-line arguments and returns an instance of `Args`.
/// - `Args::validate(&self) -> Result<()>`: Validate the path of the input file and the path of the output directory, which is the entry point for the validation check.
/// - `Args::validate_input_file_path(&self, path: &Path) -> Result<()>`: Validates the input file path ensuring it exists and has a valid extension.
/// - `Args::validate_output_dir_path(&self, path: &Path) -> Result<()>`: Validates the output directory path ensuring it exists or creates it if it does not.
///
/// # Errors
///
/// - `ErrorContext`: Struct representing possible validation errors including IO errors, invalid file extensions, and path type mismatches.
#[derive(Debug, PartialEq, Eq, Clone, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the file describing the file to be converted.
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub input_file_path: PathBuf,

    /// Path of the output directory of the converted file.
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub output_dir_path: PathBuf,

    /// Sets the logging level
    #[clap(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_input_file_path(&self.input_file_path)
            .with_context(|| {
                format!(
                    "Failed to validate input file: {}",
                    self.input_file_path.display()
                )
            })?;

        self.validate_output_dir_path(&self.output_dir_path)
            .with_context(|| {
                format!(
                    "Failed to validate output directory: {}",
                    self.output_dir_path.display()
                )
            })?;

        Ok(())
    }

    fn validate_input_file_path(&self, path: &Path) -> Result<()> {
        let mut errors = Vec::new();
        let valid_extensions: [&str; 1] = ["toml"];

        let extension_result = path
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase());

        match extension_result {
            None => {
                errors.push(ErrorContext {
                    message: "File has no extension".to_string(),
                    module: ERROR_MODULE,
                });
            }
            Some(extension) => {
                if !valid_extensions.contains(&extension.as_str()) {
                    errors.push(ErrorContext {
                        message: format!(
                            "Invalid file extension: {}, expected one of: {}",
                            extension,
                            valid_extensions.join(", ")
                        ),
                        module: ERROR_MODULE,
                    });
                }
            }
        }

        if !path.exists() {
            errors.push(ErrorContext {
                message: format!("Path does not exist: {}", path.display()),
                module: ERROR_MODULE,
            });
        } else if !path.is_file() {
            errors.push(ErrorContext {
                message: format!("Path is not a file: {}", path.display()),
                module: ERROR_MODULE,
            });
        }

        if !errors.is_empty() {
            let error_messages = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join("\n");
            return Err(anyhow::anyhow!(
                "Multiple validation errors:\n{}",
                error_messages
            ));
        }

        Ok(())
    }

    fn validate_output_dir_path(&self, path: &Path) -> Result<()> {
        let mut errors = Vec::new();

        if !path.exists() {
            errors.push(ErrorContext {
                message: format!("Path does not exist: {}", path.display()),
                module: ERROR_MODULE,
            });
        } else if !path.is_dir() {
            errors.push(ErrorContext {
                message: format!("Path is not a directory: {}", path.display()),
                module: ERROR_MODULE,
            });
        }

        if !errors.is_empty() {
            let error_messages = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join("\n");
            return Err(anyhow::anyhow!(
                "Multiple validation errors:\n{}",
                error_messages
            ));
        }

        Ok(())
    }
}

/// This module contains unit tests for the `Args` struct's validation methods.
///
/// # Test Categories
///
/// ## Input File Path Validation Tests
/// - `test_validate_input_file_path_valid`: Tests validation of a valid TOML file
/// - `test_validate_input_file_path_invalid_extensions`: Tests various invalid file extensions
/// - `test_validate_input_file_path_not_found`: Tests handling of non-existent file paths
/// - `test_validate_input_file_path_is_directory`: Tests rejection of directories as input files
/// - `test_paths_with_special_chars`: Tests paths containing spaces, Unicode characters, and special symbols
///
/// ## Output Directory Path Validation Tests
/// - `test_validate_output_dir_path_valid`: Tests validation of a valid directory path
/// - `test_validate_output_dir_path_not_found`: Tests handling of non-existent directory paths
/// - `test_validate_output_dir_path_is_file`: Tests rejection of files as output directories
///
/// # Test Coverage
///
/// The tests cover:
/// - Normal cases: Valid input files and output directories
/// - Error cases: Invalid extensions, missing files/directories, wrong path types
/// - Edge cases: Special characters in paths, empty extensions
///
/// Each test uses temporary directories to ensure isolation and cleanup.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_validate_input_file_path_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.toml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test data").unwrap();

        let args = Args {
            input_file_path: file_path.clone(),
            output_dir_path: PathBuf::from("."),
            log_level: LogLevel::Info,
        };

        assert!(args.validate_input_file_path(&file_path).is_ok());
    }

    #[test]
    fn test_validate_input_file_path_invalid_extensions() {
        let invalid_extensions = vec!["txt", "json", "yaml", ""];
        let dir = tempdir().unwrap();

        for ext in invalid_extensions {
            let file_name = if ext.is_empty() {
                "test".to_string()
            } else {
                format!("test.{}", ext)
            };

            let file_path = dir.path().join(file_name);
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "test data").unwrap();

            let args = Args {
                input_file_path: file_path.clone(),
                output_dir_path: PathBuf::from("."),
                log_level: LogLevel::Info,
            };

            let result = args.validate_input_file_path(&file_path);
            assert!(result.is_err());
            let error_msg = result.unwrap_err().to_string();

            if ext.is_empty() {
                assert!(
                    error_msg.contains("File has no extension"),
                    "Expected 'File has no extension' error, got: {}",
                    error_msg
                );
            } else {
                assert!(
                    error_msg.contains("Invalid file extension"),
                    "Expected 'Invalid file extension' error, got: {}",
                    error_msg
                );
            }
        }
    }

    #[test]
    fn test_validate_input_file_path_not_found() {
        let dir = tempdir().unwrap();
        let non_existent_paths = vec![
            dir.path().join("non_existent.toml"),
            PathBuf::from("/non/existent/path/file.toml"),
        ];

        for path in non_existent_paths {
            let args = Args {
                input_file_path: path.clone(),
                output_dir_path: PathBuf::from("."),
                log_level: LogLevel::Info,
            };

            let result = args.validate_input_file_path(&path);
            assert!(result.is_err());
            let error_msg = result.unwrap_err().to_string();
            assert!(
                error_msg.contains("Path does not exist"),
                "Expected 'Path does not exist' error, got: {}",
                error_msg
            );
        }
    }

    #[test]
    fn test_validate_input_file_path_is_directory() {
        let dir = tempdir().unwrap();
        let args = Args {
            input_file_path: dir.path().to_path_buf(),
            output_dir_path: PathBuf::from("."),
            log_level: LogLevel::Info,
        };

        let result = args.validate_input_file_path(dir.path());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Path is not a file"));
    }

    #[test]
    fn test_validate_output_dir_path_valid() {
        let dir = tempdir().unwrap();
        let args = Args {
            input_file_path: PathBuf::from("test.toml"),
            output_dir_path: dir.path().to_path_buf(),
            log_level: LogLevel::Info,
        };

        assert!(args.validate_output_dir_path(dir.path()).is_ok());
    }

    #[test]
    fn test_validate_output_dir_path_not_found() {
        let dir = tempdir().unwrap();
        let non_existent_dir = dir.path().join("non_existent");

        let args = Args {
            input_file_path: PathBuf::from("test.toml"),
            output_dir_path: non_existent_dir.clone(),
            log_level: LogLevel::Info,
        };

        let result = args.validate_output_dir_path(&non_existent_dir);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Path does not exist"));
    }

    #[test]
    fn test_validate_output_dir_path_is_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test data").unwrap();

        let args = Args {
            input_file_path: PathBuf::from("test.toml"),
            output_dir_path: file_path.clone(),
            log_level: LogLevel::Info,
        };

        let result = args.validate_output_dir_path(&file_path);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Path is not a directory"));
    }

    #[test]
    fn test_paths_with_special_chars() {
        let dir = tempdir().unwrap();
        let special_chars = vec![
            "test with spaces.toml",
            "test_with_日本語.toml",
            "test_with_#$%@.toml",
        ];

        for file_name in special_chars {
            let file_path = dir.path().join(file_name);
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "test data").unwrap();

            let args = Args {
                input_file_path: file_path.clone(),
                output_dir_path: PathBuf::from("."),
                log_level: LogLevel::Info,
            };

            assert!(args.validate_input_file_path(&file_path).is_ok());
        }
    }
}
