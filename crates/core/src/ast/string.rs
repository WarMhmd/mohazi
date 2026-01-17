use std::default;

use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringRules {
    #[serde(alias = "min", alias = "min_length")]
    pub min_length: Option<RuleType<u128>>,
    #[serde(alias = "max", alias = "max_length")]
    pub max_length: Option<RuleType<u128>>,
    pub length: Option<RuleType<u128>>,
    #[serde(alias = "pattern", alias = "regex")]
    pub regex: Option<RuleType<String>>,
    #[serde(alias = "starts_with")]
    pub starts_with: Option<RuleType<String>>,
    #[serde(alias = "ends_with")]
    pub ends_with: Option<RuleType<String>>,
    pub includes: Option<RuleType<String>>,
    pub uppercase: Option<RuleType<bool>>,
    pub lowercase: Option<RuleType<bool>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StringTransform {
    pub trim: Option<bool>,
    #[serde(
        alias = "toLowerCase",
        alias = "to_lower_case",
        alias = "lowercase",
        alias = "to_lowercase"
    )]
    pub to_lowercase: Option<bool>,
    #[serde(
        alias = "toUpperCase",
        alias = "to_upper_case",
        alias = "uppercase",
        alias = "to_uppercase"
    )]
    pub to_uppercase: Option<bool>,
    pub split: Option<String>,
    pub cast: Option<FieldType>,
}
