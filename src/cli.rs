use std::path::{Path, PathBuf};

use clap::{Parser, ValueHint};

use crate::{
    error::{AppError, ArgsValidationErr, CliErr},
    logging::LogLevel,
};

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

    pub fn validate(&self) -> Result<(), Vec<AppError>> {
        let mut errors: Vec<AppError> = Vec::new();

        let _ = self
            .validate_input_file_path(&self.input_file_path)
            .map_err(|e| {
                errors.extend(e.into_iter().map(AppError::from));
            });

        let _ = self
            .validate_output_dir_path(&self.output_dir_path)
            .map_err(|e| {
                errors.extend(e.into_iter().map(AppError::from));
            });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_input_file_path(&self, path: &Path) -> Result<(), Vec<CliErr>> {
        let mut errors: Vec<CliErr> = Vec::new();
        let valid_extensions: [&str; 1] = ["toml"];

        if let Some(extension) = path
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
        {
            if !valid_extensions.contains(&extension.as_str()) {
                errors.push(
                    ArgsValidationErr::InvalidExtension(extension, valid_extensions.join(", "))
                        .into(),
                );
            }
        } else {
            errors.push(ArgsValidationErr::NoExtension(path.to_path_buf()).into());
        }

        if !path.exists() {
            errors.push(ArgsValidationErr::PathDoesNotExist(path.to_path_buf()).into());
        } else if !path.is_file() {
            errors.push(ArgsValidationErr::PathIsNotFile(path.to_path_buf()).into());
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_output_dir_path(&self, path: &Path) -> Result<(), Vec<CliErr>> {
        let mut errors: Vec<CliErr> = Vec::new();

        if !path.exists() {
            errors.push(ArgsValidationErr::PathDoesNotExist(path.to_path_buf()).into());
        } else if !path.is_dir() {
            errors.push(ArgsValidationErr::PathIsNotDirectory(path.to_path_buf()).into());
        }

        if !errors.is_empty() {
            return Err(errors);
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
            let errors = result.unwrap_err();

            if ext.is_empty() {
                assert!(
                    errors.contains(&CliErr::Validation(ArgsValidationErr::NoExtension(
                        file_path
                    ))),
                    "Expected 'NoExtension' error, got: {:?}",
                    errors
                );
            } else {
                assert!(
                    errors.contains(&CliErr::Validation(ArgsValidationErr::InvalidExtension(
                        ext.to_string(),
                        "toml".to_string()
                    ))),
                    "Expected 'InvalidExtension' error, got: {:?}",
                    errors
                );
            }
        }
    }

    #[test]
    fn test_validate_input_file_path_not_found() {
        let non_existent_file_path = PathBuf::from("non_existent_file.toml");

        let args = Args {
            input_file_path: non_existent_file_path.clone(),
            output_dir_path: PathBuf::from("."),
            log_level: LogLevel::Info,
        };

        let result = args.validate_input_file_path(&non_existent_file_path);
        assert!(result.is_err());
        let errors = result.unwrap_err();

        assert!(
            errors.contains(&CliErr::Validation(ArgsValidationErr::PathDoesNotExist(
                non_existent_file_path.to_path_buf()
            ))),
            "Expected 'PathDoesNotExist' error, got: {:?}",
            errors
        );
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
        let errors = result.unwrap_err();

        assert!(
            errors.contains(&CliErr::Validation(ArgsValidationErr::PathIsNotFile(
                dir.path().display().to_string().into()
            ))),
            "Expected 'PathIsNotFile' error, got: {:?}",
            errors
        );
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
        let errors = result.unwrap_err();

        assert!(
            errors.contains(&CliErr::Validation(ArgsValidationErr::PathDoesNotExist(
                non_existent_dir.to_path_buf()
            ))),
            "Expected 'PathDoesNotExist' error, got: {:?}",
            errors
        );
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
        let errors = result.unwrap_err();

        assert!(
            errors.contains(&CliErr::Validation(ArgsValidationErr::PathIsNotDirectory(
                file_path.to_path_buf()
            ))),
            "Expected 'PathIsNotDirectory' error, got: {:?}",
            errors
        );
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
