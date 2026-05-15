use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use super::{parse_val, FieldType, Mergeable, RuleTrait, RuleType, TransformTrait};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashRules {
    pub algorithm: Option<RuleType<String>>,
    #[serde(alias = "pattern", alias = "regex")]
    pub regex: Option<RuleType<String>>,
}

impl RuleTrait for HashRules {
    fn new() -> Self {
        Self {
            algorithm: None,
            regex: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        let rule_err = error.clone();
        match key {
            "algorithm" => {
                self.algorithm = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
                Ok(())
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
                Ok(())
            }
            _ => Err(format!("Unknown hash rule: {}", key)),
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "algorithm" | "pattern" | "regex" => true,
            _ => false,
        }
    }
}

impl Mergeable for HashRules {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.algorithm.is_some() {
            if self.algorithm.is_some() {
                return Err("Duplicate rule: algorithm".to_string());
            } else {
                self.algorithm = other.algorithm;
            }
        }
        if other.regex.is_some() {
            if self.regex.is_some() {
                return Err("Duplicate rule: regex".to_string());
            } else {
                self.regex = other.regex;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashTransform {
    pub trim: Option<bool>,
    pub cast: Option<FieldType>,
}

impl Mergeable for HashTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.trim.is_some() {
            self.trim = other.trim;
        }
        if other.cast.is_some() {
            self.cast = other.cast;
        }
        Ok(())
    }
}

impl TransformTrait for HashTransform {
    fn new() -> Self {
        Self {
            trim: None,
            cast: None,
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "trim" | "cast" => true,
            _ => false,
        }
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        match key {
            "trim" => self.trim = Some(parse_val(value)?),
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown hash transform: {}", key)),
        }
        Ok(())
    }
}