use std::{fs, path::Path};

use clap::{Parser, ValueHint};
use log::info;

use crate::{
    errors::{CustomErrors, CustomIoError, ValidationError},
    logging::LogLevel,
};

/// This module defines the command-line interface (CLI) for the application using the `clap` crate.
/// It includes the `Args` struct which represents the parsed command-line arguments and provides
/// methods for validation of these arguments.
///
/// # Structs
///
/// - `Args`: Represents the command-line arguments and provides methods for parsing and validation.
/// - `OutputFormat`: Enum representing the possible output formats.
///
/// # Methods
///
/// - `Args::new() -> Self`: Parses the command-line arguments and returns an instance of `Args`.
/// - `Args::validate(&self) -> Result<(), ValidationError>`: Validates the input file path and output directory path.
/// - `Args::validate_path(path: &Path, is_file: bool) -> Result<String, ValidationError>`: Validates a given path as either a file or directory.
/// - `Args::validate_input_file_path<'src>(path: &'src str) -> Result<String, ValidationError>`: Validates the input file path ensuring it exists and has a valid extension.
/// - `Args::validate_output_dir_path<'src>(path: &'src str) -> Result<String, ValidationError>`: Validates the output directory path ensuring it exists or creates it if it does not.
///
/// # Enums
///
/// - `OutputFormat`: Enum representing the possible output formats (Stera3d, Jma).
///
/// # Errors
///
/// - `ValidationError`: Enum representing possible validation errors including IO errors, invalid file extensions, and path type mismatches.
///
/// # Tests
///
/// - `test_validate_input_file_path_valid`: Tests that a valid CSV file path is correctly validated.
/// - `test_validate_input_file_path_invalid_extension`: Tests that a file with an invalid extension is correctly identified and returns an error.
/// - `test_validate_input_file_path_not_found`: Tests that a non-existent file path returns a not found error.
/// - `test_validate_output_dir_path_valid`: Tests that a valid directory path is correctly validated.
/// - `test_validate_output_dir_path_create`: Tests that a new directory is created if it does not exist and is correctly validated.
/// - `test_validate_output_dir_path_not_dir`: Tests that a file path is correctly identified as not being a directory and returns an error.
#[derive(Debug, PartialEq, Eq, Clone, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the file describing the file to be converted.
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub input_file_path: String,

    /// Path of the output directory of the converted file.
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub output_dir_path: String,

    /// Selection of output format (JMA, Stera3D)
    #[clap(short = 'f', long, value_enum, default_value_t = OutputFormat::Jma)]
    pub output_format: OutputFormat,

    /// Sets the logging level
    #[clap(short, long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }

    pub fn validate(&self) -> Result<(), CustomErrors> {
        let mut errors: Vec<ValidationError> = Vec::new();
        if let Err(e) = Args::validate_input_file_path(&self.input_file_path) {
            errors.extend(e);
        };
        if let Err(e) = Args::validate_output_dir_path(&self.output_dir_path) {
            errors.extend(e);
        };

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into())
        }
    }

    fn validate_path(path: &Path, is_file: bool) -> Result<(), Vec<ValidationError>> {
        let mut errors: Vec<ValidationError> = Vec::new();

        if !path.exists() {
            errors.push(ValidationError::Io(CustomIoError::NotFound {
                path: path.to_path_buf(),
            }));
        }

        if is_file && !path.is_file() {
            errors.push(ValidationError::PathIsNotFile(path.to_path_buf()));
        }

        if !is_file && !path.is_dir() {
            errors.push(ValidationError::PathIsNotDir(path.to_path_buf()));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_input_file_path<'src>(path: &'src str) -> Result<(), Vec<ValidationError>> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let valid_extensions: [&'src str; 1] = ["csv"];
        let path = Path::new(path);

        Args::validate_path(path, true)?;

        match path.extension() {
            Some(ext) => {
                let ext = ext.to_string_lossy().to_lowercase();
                if valid_extensions.contains(&ext.as_str()) {
                    Ok(())
                } else {
                    errors.push(ValidationError::InvalidFileExt(
                        ext,
                        valid_extensions[0].to_string(),
                    ));
                    Err(errors)
                }
            }
            None => {
                errors.push(ValidationError::ExtNotFound);
                Err(errors)
            }
        }
    }

    fn validate_output_dir_path<'src>(path: &'src str) -> Result<(), Vec<ValidationError>> {
        let mut errors: Vec<ValidationError> = Vec::new();
        let path = Path::new(path);

        if !path.exists() {
            if let Err(e) = fs::create_dir_all(path)
                .map_err(|e| ValidationError::Io(CustomIoError::from((e, path.to_path_buf()))))
            {
                errors.push(e);
            }
            info!("{:?} did not exist, so the directory was created.", path);
        }

        Args::validate_path(path, false)?;

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    Stera3d,
    Jma,
}

/// This module contains unit tests for the `Args` struct's validation methods.
///
/// # Tests
///
/// - `test_validate_input_file_path_valid`: Tests that a valid CSV file path is correctly validated.
/// - `test_validate_input_file_path_invalid_extension`: Tests that a file with an invalid extension is correctly identified and returns an error.
/// - `test_validate_input_file_path_not_found`: Tests that a non-existent file path returns a not found error.
/// - `test_validate_output_dir_path_valid`: Tests that a valid directory path is correctly validated.
/// - `test_validate_output_dir_path_create`: Tests that a new directory is created if it does not exist and is correctly validated.
/// - `test_validate_output_dir_path_not_dir`: Tests that a file path is correctly identified as not being a directory and returns an error.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    #[test]
    fn test_validate_input_file_path_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csv");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test data").unwrap();

        let result = Args::validate_input_file_path(file_path.to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_validate_input_file_path_invalid_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test data").unwrap();

        let result = Args::validate_input_file_path(file_path.to_str().unwrap());
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| *e == ValidationError::InvalidFileExt("txt".to_string(), "csv".to_string())));
    }

    #[test]
    fn test_validate_input_file_path_not_found() {
        let result = Args::validate_input_file_path("non_existent.csv");
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::Io(CustomIoError::NotFound { .. }))));
    }

    #[test]
    fn test_validate_output_dir_path_valid() {
        let dir = tempdir().unwrap();
        let result = Args::validate_output_dir_path(dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_validate_output_dir_path_create() {
        let dir = tempdir().unwrap();
        let new_dir_path = dir.path().join("new_dir");

        let result = Args::validate_output_dir_path(new_dir_path.to_str().unwrap());
        assert!(result.is_ok());
        assert!(new_dir_path.exists());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_validate_output_dir_path_not_dir() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csv");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test data").unwrap();

        let result = Args::validate_output_dir_path(file_path.to_str().unwrap());
        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::PathIsNotDir(_))));
    }
}
