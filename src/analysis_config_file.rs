use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::error::{ErrorContext, Module};

const ERROR_MODULE: Module = Module::ConfigFileAnalysis;

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

pub trait Validate {
    fn validate(&self) -> Result<()>;
}

// NOTE: tomlクレートによって、列挙型での入力値のバリデーションが行われるため、この構造体ではバリデーションの実装を行いません。
#[derive(Debug, Serialize, Deserialize)]
pub struct Conversion {
    /// Before conversion.
    from: From,
    /// After conversion.
    to: To,
    /// File configuration groups.
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    /// Specify file path to be analyzed.
    pub path: PathBuf,
    /// direction component identifier(ns, ew, ud).
    pub direction: Option<AccAxis>,
    /// identifier key for grouping.
    pub g_key: Option<u32>,
}

impl Group {
    pub fn valid_path_in_group(&self) -> Result<()> {
        todo!()
    }

    pub fn valid_direction_component(&self) -> Result<()> {
        todo!()
    }

    pub fn valid_group_key(&self) -> Result<()> {
        todo!()
    }
}

impl Validate for Group {
    fn validate(&self) -> Result<()> {
        self.valid_direction_component()?;
        self.valid_group_key()?;
        self.valid_path_in_group()?;

        Ok(())
    }
}

// NOTE: tomlクレートによって、列挙型での入力値のバリデーションが行われるため、この構造体ではバリデーションの実装を行いません。
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub config: GlobalSettings,
}

impl GlobalConfig {
    pub fn validate(&self) {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalSettings {
    /// output file name format.
    name_format: NameFormat,
    /// acceleration calculate option bool.
    acc_calculate: bool,
    /// unit conversion option bool.
    unit_conversion: bool,
}

impl GlobalSettings {
    pub fn validate(&self) {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub conversions: Vec<Conversion>,
    pub global: GlobalConfig,
}

impl Config {
    pub fn validate(&self) {
        todo!()
    }

    pub fn group_by_key(&self) -> Result<Vec<Vec<&Group>>> {
        let mut result: Vec<Vec<&Group>> = Vec::new();

        for (i, conversion) in self.conversions.iter().enumerate() {
            let mut grouped: HashMap<Option<u32>, Vec<&Group>> = HashMap::new();

            // Group keyによるグループ化を行う。
            for group in &conversion.groups {
                grouped.entry(group.g_key).or_default().push(group);
            }

            // Group keyがNoneである個別要素は、異なる種類としてグルーピングします。
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

pub fn read_config_from_input_file(input_file_path: &Path) -> Result<String> {
    fs::read_to_string(input_file_path).with_context(|| ErrorContext {
        message: format!("Failed to read config file: {}", input_file_path.display()),
        module: ERROR_MODULE,
    })
}
