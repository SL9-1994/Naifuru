use std::{fs, path::Path};

use clap::{Parser, ValueHint};
use log::{debug, info};

use crate::{
    errors::{CustomIoError, ValidationError},
    logging::LogLevel,
};

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

    pub fn validate(&self) -> Result<(), ValidationError> {
        let validate_input = Args::validate_input_file_path(&self.input_file_path)?;
        let validate_output = Args::validate_output_dir_path(&self.output_dir_path)?;

        info!("Input file validated: {}", validate_input);
        info!("Output directory validated: {}", validate_output);

        Ok(())
    }

    fn validate_path(path: &Path, is_file: bool) -> Result<String, ValidationError> {
        if !path.exists() {
            return Err(ValidationError::Io(CustomIoError::NotFound {
                path: path.to_path_buf(),
            }));
        }

        if is_file && !path.is_file() {
            return Err(ValidationError::PathIsNotFile(path.to_path_buf()));
        }

        if !is_file && !path.is_dir() {
            return Err(ValidationError::PathIsNotDir(path.to_path_buf()));
        }

        Ok(path.to_string_lossy().to_string())
    }

    fn validate_input_file_path<'src>(path: &'src str) -> Result<String, ValidationError> {
        let valid_extensions: [&'src str; 1] = ["csv"];
        let path = Path::new(path);

        Args::validate_path(path, true)?;

        match path.extension() {
            Some(ext) => {
                let ext = ext.to_string_lossy().to_lowercase();
                if valid_extensions.contains(&ext.as_str()) {
                    debug!("fn validate_input_file_path is Successful");
                    Ok(path.to_string_lossy().to_string())
                } else {
                    Err(ValidationError::InvalidFileExt(
                        ext,
                        valid_extensions[0].to_string(),
                    ))
                }
            }
            None => Err(ValidationError::ExtNotFound),
        }
    }

    fn validate_output_dir_path<'src>(path: &'src str) -> Result<String, ValidationError> {
        let path = Path::new(path);

        if !path.exists() {
            fs::create_dir_all(path)
                .map_err(|e| ValidationError::Io(CustomIoError::from((e, path.to_path_buf()))))?;
            info!("{:?} did not exist, so the directory was created.", path);
        }

        Args::validate_path(path, false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    Stera3d,
    Jma,
}
