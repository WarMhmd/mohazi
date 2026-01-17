use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;
use serde::de::{self};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumRules {
    #[serde(rename = "values", alias = "value")]
    values: Option<RuleType<Vec<StringOrVec>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnumTransform {
    pub cast: Option<FieldType>,
}
