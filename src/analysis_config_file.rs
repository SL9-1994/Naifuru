use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AnalysisConfigErr, AppError, ConfigValidationErr, IoErrWrapper};

/// File format before conversion.  
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum From {
    JpNiedKnet,
    UsScsnV2,
    NzGeonetV1a,
    NzGeonetV2a,
    TwPalertSac,
    TkAfadAsc,
}

/// File format after conversion.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum To {
    JpJmaCsv,
    JpStera3dTxt,
}

/// File format before conversion.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccAxis {
    Ns,
    Ew,
    Ud,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NameFormat {
    /// ## **Example: 20240101-161018-ISK005-knet.csv.**
    /// - yyyymmdd:  Date and time of observation start date and time.
    /// - hhmmss: Hour, minute, second of the observation start date and time.
    /// - sn: Observation station name(ISK005, WVAS, etc...).
    /// - n: Institution name(knet, geonet, etc...).
    YyyymmddHhmmssSnN,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
    pub conversion: Vec<ConversionConfig>,
}

impl Config {
    pub fn validate(&self) -> Result<(), Vec<AppError>> {
        let mut errors: Vec<AppError> = Vec::new();

        for conv_config in &self.conversion {
            let _ = conv_config.validate().map_err(|e| {
                errors.extend(e.into_iter().map(AppError::from));
            });
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

// MEMO: 列挙型はtomlによってバリデーションが行われるため、この構造体でバリデーション実装は行いません。
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub name_format: NameFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub from: From,
    pub to: To,
    pub group: Vec<GroupConfig>,
}

impl ConversionConfig {
    pub fn validate(&self) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        for group_config in &self.group {
            let acceptable_exts: &[&str] = Self::assign_ext_based_on_from(&self.from);
            let _ = group_config.validate(acceptable_exts).map_err(|e| {
                errors.extend(e.into_iter().map(AnalysisConfigErr::from));
            });
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn assign_ext_based_on_from(from: &From) -> &[&str] {
        match from {
            From::JpNiedKnet => &["ns", "ew", "ud"],
            From::UsScsnV2 => &["v2"],
            From::NzGeonetV1a => &["v1a"],
            From::NzGeonetV2a => &["v2a"],
            From::TwPalertSac => &["sac"],
            From::TkAfadAsc => &["asc"],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConfig {
    pub files: Vec<FileConfig>,
}

impl GroupConfig {
    pub fn validate(&self, acceptable_exts: &[&str]) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        for file in &self.files {
            let _ = file.validate(acceptable_exts).map_err(|e| {
                errors.extend(e.into_iter().map(AnalysisConfigErr::from));
            });
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub path: PathBuf,
    pub acc_axis: Option<AccAxis>,
}

impl FileConfig {
    pub fn validate(&self, acceptable_exts: &[&str]) -> Result<(), Vec<AnalysisConfigErr>> {
        let mut errors: Vec<AnalysisConfigErr> = Vec::new();

        let _ = self
            .validate_extension_for_acceptable_exts(acceptable_exts)
            .map_err(|e| {
                errors.push(e);
            });

        let _ = self.validate_path().map_err(|e| {
            errors.push(e);
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }

    fn validate_path(&self) -> Result<(), AnalysisConfigErr> {
        if !self.path.exists() {
            return Err(ConfigValidationErr::PathDoesNotExist(self.path.to_path_buf()).into());
        } else if !self.path.is_file() {
            return Err(ConfigValidationErr::PathIsNotFile(self.path.to_path_buf()).into());
        }

        Ok(())
    }

    fn validate_extension_for_acceptable_exts(
        &self,
        acceptable_exts: &[&str],
    ) -> Result<(), AnalysisConfigErr> {
        if let Some(extension) = self
            .path
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
        {
            if !acceptable_exts.contains(&extension.as_str()) {
                return Err(ConfigValidationErr::InvalidExtension(
                    acceptable_exts.join(", "),
                    extension,
                )
                .into());
            }
        } else {
            return Err(ConfigValidationErr::NoExtension(self.path.to_path_buf()).into());
        }

        Ok(())
    }
}

pub fn read_config_from_input_file(input_file_path: &Path) -> Result<String, IoErrWrapper> {
    let config: String = std::fs::read_to_string(input_file_path)?;

    Ok(config)
}
