use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayRules {
    #[serde(rename = "type", alias = "lessThan")]
    pub array_type: RuleType<FieldType>,
    #[serde(rename = "length", alias = "size")]
    pub length: Option<RuleType<usize>>,
    #[serde(rename = "minLength", alias = "min")]
    pub min_length: Option<RuleType<usize>>,
    #[serde(rename = "maxLength", alias = "max")]
    pub max_length: Option<RuleType<usize>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTransform {
    pub join: Option<String>,
    pub sum: Option<bool>,
    pub cast: Option<FieldType>,
}
