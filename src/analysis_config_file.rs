use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorContext, Module};

const ERROR_MODULE: Module = Module::ConfigFileAnalysis;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversion {
    /// Before conversion
    from: From,
    /// After conversion
    to: To,
    /// File configuration groups
    groups: Vec<Group>,
}

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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum To {
    JpJmaCsv,
    JpStera3dTxt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub path: PathBuf,
    /// direction component identifier(ns, ew, ud)
    pub direction: Option<AccDirection>,
    /// identifier key for grouping
    pub g_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccDirection {
    Ns,
    Ew,
    Ud,
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

pub fn read_config_from_input_file(input_file_path: &Path) -> Result<String> {
    fs::read_to_string(input_file_path).with_context(|| ErrorContext {
        message: format!("Failed to read config file: {}", input_file_path.display()),
        module: ERROR_MODULE,
    })
}

impl Config {
    pub fn group_by_key(&self) -> Result<Vec<Vec<&Group>>> {
        let mut result: Vec<Vec<&Group>> = Vec::new();

        for (i, conversion) in self.conversions.iter().enumerate() {
            let mut grouped: HashMap<Option<u32>, Vec<&Group>> = HashMap::new();

            // Grouping by g_key
            for group in &conversion.groups {
                grouped.entry(group.g_key).or_default().push(group);
            }

            // Groups with g_key of None are handled individually
            for (key, groups) in grouped {
                if groups.is_empty() {
                    return Err(anyhow::anyhow!(ErrorContext {
                        message: format!("Empty group found for key {:?} in conversion {}", key, i),
                        module: ERROR_MODULE,
                    }));
                }
                result.push(groups);
            }
        }

        if result.is_empty() {
            return Err(anyhow::anyhow!(ErrorContext {
                message: "No valid groups found in configuration".to_string(),
                module: ERROR_MODULE,
            }));
        }

        Ok(result)
    }
}
