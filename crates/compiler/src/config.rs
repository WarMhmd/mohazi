use std::collections::HashMap;

use serde::{ser, Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    pub input: String,
    pub languages: HashMap<Language, LanguageConfig>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    Javascript,
    #[serde(rename = "C#")]
    CSharp,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LanguageConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub output: String,
}

fn default_enabled() -> bool {
    true
}
