use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::IoErrWrapper;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub name_format: NameFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub id: u16,
    pub from: From,
    pub to: To,
    pub group: Vec<GroupConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupConfig {
    pub id: u16,
    pub files: Vec<FileConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub path: PathBuf,
    pub acc_axis: Option<AccAxis>,
}

pub fn read_config_from_input_file(input_file_path: &Path) -> Result<String, IoErrWrapper> {
    let config: String = std::fs::read_to_string(input_file_path)?;

    Ok(config)
}
