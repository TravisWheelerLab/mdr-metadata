use serde::{Deserialize, Serialize};
use toml::value::Value as TomlValue;

pub const MIN_TEMP_K: u32 = 273;
pub const MAX_TEMP_K: u32 = 374;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Datelike {
    Stringy(String),
    TomlDate(toml::value::Datetime),
}

impl Datelike {
    pub fn to_string(&self) -> String {
        match &self {
            Datelike::TomlDate(dt) => dt.to_string(),
            Datelike::Stringy(val) => val.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Numlike {
    Stringy(String),
    TomlVal(TomlValue),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Software {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RequiredFile {
    pub trajectory_file_name: String,

    pub structure_file_name: String,

    pub topology_file_name: String,
}
