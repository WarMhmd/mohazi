use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberRules {
    #[serde(rename = "lt", alias = "lessThan")]
    pub lt: Option<RuleType<i128>>,
    #[serde(rename = "max", alias = "lte", alias = "lessThanOrEqual")]
    pub max: Option<RuleType<i128>>,
    #[serde(rename = "gt", alias = "greaterThan")]
    pub gt: Option<RuleType<i128>>,
    #[serde(rename = "min", alias = "gte", alias = "greaterThanOrEqual")]
    pub min: Option<RuleType<i128>>,
    pub equal: Option<RuleType<i128>>,
    pub positive: Option<RuleType<bool>>,
    #[serde(alias = "nonPositive", alias = "non_positive")]
    pub nonpositive: Option<RuleType<bool>>,
    pub negative: Option<RuleType<bool>>,
    #[serde(alias = "nonNegative", alias = "non_negative")]
    pub nonnegative: Option<RuleType<bool>>,
    #[serde(alias = "divisibleBy", alias = "multipleOf", alias = "multiple_of")]
    pub multiple_of: Option<RuleType<i128>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NumberTransform {
    pub cast: Option<FieldType>,
}
