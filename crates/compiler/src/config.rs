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
    pub enabled: Option<bool>,
    pub output: String,
}
