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
pub struct UuidRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<RuleType<String>>,
    #[serde(alias = "pattern", alias = "regex")]
    pub regex: Option<RuleType<String>>,
}

impl RuleTrait for UuidRules {
    fn new() -> Self {
        Self {
            version: None,
            regex: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        match key {
            "version" => {
                let parsed_version: String = parse_val(value)?;
                let valid_versions = ["v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8"];
                if !valid_versions.contains(&parsed_version.as_str()) {
                    return Err(format!("Invalid UUID version '{}'. Only v1 through v8 are supported.", parsed_version));
                }
                self.version = Some(RuleType {
                    value: parsed_version,
                    error,
                });
                Ok(())
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
                Ok(())
            }
            _ => Err(format!("Unknown uuid rule: {}", key)),
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "version" | "pattern" | "regex" => true,
            _ => false,
        }
    }
}

impl Mergeable for UuidRules {
    fn merge(&mut self, other: UuidRules) -> Result<(), String> {
        if other.version.is_some() {
            self.version = other.version;
        }
        if other.regex.is_some() {
            self.regex = other.regex;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UuidTransform {
    pub trim: Option<bool>,
    pub to_lowercase: Option<bool>,
    pub to_uppercase: Option<bool>,
    pub cast: Option<FieldType>,
}

impl Mergeable for UuidTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.trim.is_some() {
            self.trim = other.trim;
        }
        if other.to_lowercase.is_some() {
            self.to_lowercase = other.to_lowercase;
        }
        if other.to_uppercase.is_some() {
            self.to_uppercase = other.to_uppercase;
        }
        if other.cast.is_some() {
            self.cast = other.cast;
        }
        Ok(())
    }
}

impl TransformTrait for UuidTransform {
    fn new() -> Self {
        UuidTransform {
            trim: None,
            to_lowercase: None,
            to_uppercase: None,
            cast: Some(FieldType::String),
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "trim" | "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" | "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" | "cast" => true,
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
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown uuid transform: {}", key)),
        }
        Ok(())
    }
}
