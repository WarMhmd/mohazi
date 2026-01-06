use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use super::parse_val;
use super::FieldType;
use super::Mergeable;
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

impl NumberRules {
    pub fn new() -> Self {
        Self {
            lt: None,
            max: None,
            gt: None,
            min: None,
            equal: None,
            positive: None,
            nonpositive: None,
            negative: None,
            nonnegative: None,
            multiple_of: None,
        }
    }

    pub fn set_rule(
        &mut self,
        key: &str,
        value: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        let rule_err = error;

        match key {
            "min" | "gte" => {
                self.min = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "max" | "lte" => {
                self.max = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            // ... Add cases for positive, multipleOf, etc.
            _ => return Err(format!("Unknown number rule: {}", key)),
        }
        Ok(())
    }

    pub fn is_valid_key(key: &str) -> bool {
        match key {
            // Field: lt (Renamed to "lt", Alias: "lessThan")
            "lt" | "lessThan" => true,

            // Field: max (Renamed to "max", Aliases: "lte", "lessThanOrEqual")
            "max" | "lte" | "lessThanOrEqual" => true,

            // Field: gt (Renamed to "gt", Alias: "greaterThan")
            "gt" | "greaterThan" => true,

            // Field: min (Renamed to "min", Aliases: "gte", "greaterThanOrEqual")
            "min" | "gte" | "greaterThanOrEqual" => true,

            // Field: equal (camelCase: equal)
            "equal" => true,

            // Field: positive (camelCase: positive)
            "positive" => true,

            // Field: nonpositive (camelCase: nonpositive, Aliases: "nonPositive", "non_positive")
            "nonpositive" | "nonPositive" | "non_positive" => true,

            // Field: negative (camelCase: negative)
            "negative" => true,

            // Field: nonnegative (camelCase: nonnegative, Aliases: "nonNegative", "non_negative")
            "nonnegative" | "nonNegative" | "non_negative" => true,

            // Field: multiple_of (camelCase: multipleOf, Aliases: "divisibleBy", "multipleOf", "multiple_of")
            "multipleOf" | "divisibleBy" | "multiple_of" => true,

            _ => false,
        }
    }
}

impl Mergeable for NumberRules {
    fn merge(&mut self, other: NumberRules, errors: &mut Vec<String>) {
        if other.lt.is_some() {
            if self.lt.is_some() {
                errors.push("Duplicate rule: lt".to_string());
            } else {
                self.lt = other.lt;
            }
        }
        if other.max.is_some() {
            if self.max.is_some() {
                errors.push("Duplicate rule: max".to_string());
            } else {
                self.max = other.max;
            }
        }
        if other.gt.is_some() {
            if self.gt.is_some() {
                errors.push("Duplicate rule: gt".to_string());
            } else {
                self.gt = other.gt;
            }
        }
        if other.min.is_some() {
            if self.min.is_some() {
                errors.push("Duplicate rule: min".to_string());
            } else {
                self.min = other.min;
            }
        }
        if other.equal.is_some() {
            if self.equal.is_some() {
                errors.push("Duplicate rule: equal".to_string());
            } else {
                self.equal = other.equal;
            }
        }
        if other.positive.is_some() {
            if self.positive.is_some() {
                errors.push("Duplicate rule: positive".to_string());
            } else {
                self.positive = other.positive;
            }
        }
        if other.nonpositive.is_some() {
            if self.nonpositive.is_some() {
                errors.push("Duplicate rule: nonpositive".to_string());
            } else {
                self.nonpositive = other.nonpositive;
            }
        }
        if other.negative.is_some() {
            if self.negative.is_some() {
                errors.push("Duplicate rule: negative".to_string());
            } else {
                self.negative = other.negative;
            }
        }
        if other.nonnegative.is_some() {
            if self.nonnegative.is_some() {
                errors.push("Duplicate rule: nonnegative".to_string());
            } else {
                self.nonnegative = other.nonnegative;
            }
        }
        if other.multiple_of.is_some() {
            if self.multiple_of.is_some() {
                errors.push("Duplicate rule: multiple_of".to_string());
            } else {
                self.multiple_of = other.multiple_of;
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberTransform {
    pub cast: Option<FieldType>,
}
