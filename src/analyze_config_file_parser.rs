use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::errors::{ConfigParseError, CustomIoError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversion {
    from: String,
    to: String,
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub path: String,
    pub component: Option<String>,
    #[serde(default)]
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
    converted_name: String,
    /// acceleration calculate option bool
    acc_calculate: bool,
    /// unit conversion option bool
    unit_conversion: bool,
}

impl Config {
    pub fn read_config_from_input_file(
        &self,
        input_file_path: &Path,
    ) -> Result<String, ConfigParseError> {
        match fs::read_to_string(input_file_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(ConfigParseError::Io(CustomIoError::from((
                e,
                input_file_path.to_path_buf(),
            )))),
        }
    }

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
