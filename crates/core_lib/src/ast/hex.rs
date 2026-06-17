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
pub struct HexRules {
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

impl RuleTrait for HexRules {
    fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            regex: Some(RuleType {
                value: "^(0x|0X)?[0-9a-fA-F]+$".to_string(),
                error: None,
            }),
            starts_with: None,
            ends_with: None,
            includes: None,
            uppercase: None,
            lowercase: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        let rule_err = error;

        match key {
            "minLength" | "min_length" | "min" => {
                self.min_length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "maxLength" | "max_length" | "max" => {
                self.max_length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "length" => {
                self.length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            "startsWith" | "starts_with" => {
                self.starts_with = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            "endsWith" | "ends_with" => {
                self.ends_with = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            "includes" => {
                self.includes = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" => {
                self.lowercase = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" => {
                self.uppercase = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                })
            }
            _ => return Err(format!("Unknown hex rule: {}", key)),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "minLength" | "min" | "min_length" => true,
            "maxLength" | "max" | "max_length" => true,
            "length" => true,
            "regex" | "pattern" => true,
            "startsWith" | "starts_with" => true,
            "endsWith" | "ends_with" => true,
            "includes" => true,
            "uppercase" => true,
            "lowercase" => true,
            _ => false,
        }
    }
}

impl Mergeable for HexRules {
    fn merge(&mut self, other: HexRules) -> Result<(), String> {
        if other.min_length.is_some() {
            if self.min_length.is_some() {
                return Err("Duplicate rule: min_length".to_string());
            } else {
                self.min_length = other.min_length;
            }
        }
        if other.max_length.is_some() {
            if self.max_length.is_some() {
                return Err("Duplicate rule: max_length".to_string());
            } else {
                self.max_length = other.max_length;
            }
        }
        if other.length.is_some() {
            if self.length.is_some() {
                return Err("Duplicate rule: length".to_string());
            } else {
                self.length = other.length;
            }
        }
        if other.regex.is_some() {
            if self.regex.is_some() {
                return Err("Duplicate rule: regex".to_string());
            } else {
                self.regex = other.regex;
            }
        }
        if other.starts_with.is_some() {
            if self.starts_with.is_some() {
                return Err("Duplicate rule: starts_with".to_string());
            } else {
                self.starts_with = other.starts_with;
            }
        }
        if other.ends_with.is_some() {
            if self.ends_with.is_some() {
                return Err("Duplicate rule: ends_with".to_string());
            } else {
                self.ends_with = other.ends_with;
            }
        }
        if other.includes.is_some() {
            if self.includes.is_some() {
                return Err("Duplicate rule: includes".to_string());
            } else {
                self.includes = other.includes;
            }
        }
        if other.uppercase.is_some() {
            if self.uppercase.is_some() {
                return Err("Duplicate rule: uppercase".to_string());
            } else {
                self.uppercase = other.uppercase;
            }
        }
        if other.lowercase.is_some() {
            if self.lowercase.is_some() {
                return Err("Duplicate rule: lowercase".to_string());
            } else {
                self.lowercase = other.lowercase;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HexTransform {
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

impl Mergeable for HexTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.cast.is_some() {
            if self.cast.is_some() {
                return Err("Duplicate transform: cast".to_string());
            } else {
                self.cast = other.cast;
            }
        }

        if other.to_uppercase.is_some() {
            if self.to_uppercase.is_some() {
                return Err("Duplicate transform: to_uppercase".to_string());
            } else {
                self.to_uppercase = other.to_uppercase;
            }
        }

        if other.to_lowercase.is_some() {
            if self.to_lowercase.is_some() {
                return Err("Duplicate transform: to_lowercase".to_string());
            } else {
                self.to_lowercase = other.to_lowercase;
            }
        }

        if other.trim.is_some() {
            if self.trim.is_some() {
                return Err("Duplicate transform: trim".to_string());
            } else {
                self.trim = other.trim;
            }
        }

        if other.split.is_some() {
            if self.split.is_some() {
                return Err("Duplicate transform: split".to_string());
            } else {
                self.split = other.split;
            }
        }

        Ok(())
    }
}

impl TransformTrait for HexTransform {
    fn new() -> Self {
        HexTransform {
            trim: None,
            to_lowercase: None,
            to_uppercase: None,
            split: None,
            cast: None,
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "trim" => true,
            "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" => true,
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" => true,
            "split" => true,
            "cast" => true,
            _ => false,
        }
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        match key {
            "trim" => self.trim = Some(parse_val(value)?),
            "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" => {
                self.to_lowercase = Some(parse_val(value)?)
            }
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" => {
                self.to_uppercase = Some(parse_val(value)?)
            }
            "split" => self.split = Some(parse_val(value)?),
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown hex transform {}", key)),
        };
        Ok(())
    }
}
