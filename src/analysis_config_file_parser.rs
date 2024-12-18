use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::errors::{ConfigParseError, CustomIoError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversion {
    /// Before conversion
    from: ConversionFrom,
    /// After conversion
    to: ConversionTo,
    /// File configuration groups
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionFrom {
    JpNiedKnet,
    UsScsnV2,
    NzGeonetV1a,
    NzGeonetV2a,
    TwPalertSac,
    TkAfadAsc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionTo {
    JpJmaCsv,
    JpStera3dTxt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub path: String,
    /// direction component identifier(ns, ew, ud)
    pub component: Option<String>,
    /// identifier key for grouping
    pub g_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub conversions: Vec<Conversion>,
    pub global: GlobalConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub config: GlobalSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalSettings {
    /// output file name format
    name_format: NameFormat,
    /// acceleration calculate option bool
    acc_calculate: bool,
    /// unit conversion option bool
    unit_conversion: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NameFormat {
    /// ## **Example: 20240101-161018-ISK005-knet.csv**
    /// - yyyymmdd:  Date and time of observation start date and time
    /// - hhmmss: Hour, minute, second of the observation start date and time
    /// - sn: Observation station name(ISK005, WVAS, etc...)
    /// - n: Institution name(knet, geonet, etc...)
    YyyymmddHhmmssSnN,
}

pub fn read_config_from_input_file(
    input_file_path: &Path,
) -> Result<String, Vec<ConfigParseError>> {
    match fs::read_to_string(input_file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(vec![ConfigParseError::Io(CustomIoError::from((
            e,
            input_file_path.to_path_buf(),
        )))]),
    }
}

impl Config {
    /// Groups the `Group` objects within each `Conversion` by their `g_key` value.
    pub fn group_by_key(&self) -> Vec<Vec<&Group>> {
        let mut result: Vec<Vec<&Group>> = Vec::new();

        for conversion in &self.conversions {
            let mut grouped: HashMap<Option<u32>, Vec<&Group>> = HashMap::new();

            // Grouping by g_key
            for group in &conversion.groups {
                grouped.entry(group.g_key).or_default().push(group);
            }

            // Groups with g_key of None are handled individually
            for (_, groups) in grouped {
                result.push(groups);
            }
        }

        result
    }
}
