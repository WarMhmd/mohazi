use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use super::parse_val;
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

impl StringRules {
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            regex: None,
            starts_with: None,
            ends_with: None,
            includes: None,
            uppercase: None,
            lowercase: None,
        }
    }

    pub fn set_rule(
        &mut self,
        key: &str,
        value: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        let rule_err = error; // alias for clarity

        match key {
            "minLength" | "min_length" => {
                self.min_length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "maxLength" | "max_length" => {
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
            // ... Add cases for starts_with, uppercase, etc.
            _ => return Err(format!("Unknown string rule: {}", key)),
        }
        Ok(())
    }
    /// Checks if a string key is a valid rule name, respecting
    /// camelCase renaming and your custom aliases.
    pub fn is_valid_key(key: &str) -> bool {
        match key {
            // Field: min_length (camelCase: minLength)
            "minLength" | "min" | "min_length" => true,

            // Field: max_length (camelCase: maxLength)
            "maxLength" | "max" | "max_length" => true,

            // Field: length (camelCase: length)
            "length" => true,

            // Field: regex (camelCase: regex)
            "regex" | "pattern" => true,

            // Field: starts_with (camelCase: startsWith)
            "startsWith" | "starts_with" => true,

            // Field: ends_with (camelCase: endsWith)
            "endsWith" | "ends_with" => true,

            // Field: includes (camelCase: includes)
            "includes" => true,

            // Field: uppercase (camelCase: uppercase)
            "uppercase" => true,

            // Field: lowercase (camelCase: lowercase)
            "lowercase" => true,

            _ => false,
        }
    }

    pub fn merge(&mut self, other: StringRules, erros: &mut Vec<String>) {
        if other.min_length.is_some() {
            if self.min_length.is_some() {
                erros.push("Duplicate rule: min_length".to_string());
            } else {
                self.min_length = other.min_length;
            }
        }
        if other.max_length.is_some() {
            if self.max_length.is_some() {
                erros.push("Duplicate rule: max_length".to_string());
            } else {
                self.max_length = other.max_length;
            }
        }
        if other.length.is_some() {
            if self.length.is_some() {
                erros.push("Duplicate rule: length".to_string());
            } else {
                self.length = other.length;
            }
        }
        if other.regex.is_some() {
            if self.regex.is_some() {
                erros.push("Duplicate rule: regex".to_string());
            } else {
                self.regex = other.regex;
            }
        }
        if other.starts_with.is_some() {
            if self.starts_with.is_some() {
                erros.push("Duplicate rule: starts_with".to_string());
                return;
            }
            self.starts_with = other.starts_with;
        }
        if other.ends_with.is_some() {
            if self.ends_with.is_some() {
                erros.push("Duplicate rule: ends_with".to_string());
                return;
            }
            self.ends_with = other.ends_with;
        }
        if other.includes.is_some() {
            if self.includes.is_some() {
                erros.push("Duplicate rule: includes".to_string());
            } else {
                self.includes = other.includes;
            }
        }
        if other.uppercase.is_some() {
            if self.uppercase.is_some() {
                erros.push("Duplicate rule: uppercase".to_string());
            } else {
                self.uppercase = other.uppercase;
            }
        }
        if other.lowercase.is_some() {
            if self.lowercase.is_some() {
                erros.push("Duplicate rule: lowercase".to_string());
            } else {
                self.lowercase = other.lowercase;
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
