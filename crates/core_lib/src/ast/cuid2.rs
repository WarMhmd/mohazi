use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::TransformTrait;
use super::parse_val;
use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cuid2Rules {
    #[serde(alias = "min", alias = "min_length")]
    pub min_length: Option<RuleType<u128>>,
    #[serde(alias = "max", alias = "max_length")]
    pub max_length: Option<RuleType<u128>>,
    #[serde(alias = "pattern", alias = "regex")]
    pub regex: Option<RuleType<String>>,
}

impl RuleTrait for Cuid2Rules {
    fn new() -> Self {
        Self {
            min_length: Some(RuleType {
                value: 9,
                error: None,
            }),
            max_length: Some(RuleType {
                value: 31,
                error: None,
            }),
            regex: Some(RuleType {
                value: "^[a-z][a-z0-9]{8,30}$".to_string(),
                error: None,
            }),
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        match key {
            "minLength" | "min" | "min_length" => {
                self.min_length = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "maxLength" | "max" | "max_length" => {
                self.max_length = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => return Err(format!("Unknown cuid2 rule: {}", key)),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "minLength" | "min" | "min_length" | "maxLength" | "max" | "max_length" | "pattern" | "regex" => true,
            _ => false,
        }
    }
}

impl Mergeable for Cuid2Rules {
    fn merge(&mut self, other: Cuid2Rules) -> Result<(), String> {
        if other.min_length.is_some() {
            self.min_length = other.min_length;
        }
        if other.max_length.is_some() {
            self.max_length = other.max_length;
        }
        if other.regex.is_some() {
            self.regex = other.regex;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cuid2Transform {
    pub trim: Option<bool>,
    pub cast: Option<FieldType>,
}

impl Mergeable for Cuid2Transform {
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

impl TransformTrait for Cuid2Transform {
    fn new() -> Self {
        Cuid2Transform {
            trim: None,
            cast: Some(FieldType::String),
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
            _ => return Err(format!("Unknown cuid2 transform: {}", key)),
        }
        Ok(())
    }
}
