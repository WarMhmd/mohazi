use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::TransformTrait;
use crate::ast::Mergeable;
use crate::ast::RuleTrait;
use crate::ast::RuleType;
use super::parse_val;
use super::FieldType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Base64Rules {
    pub url: Option<RuleType<bool>>,
    #[serde(alias = "min_size")]
    pub min_size: Option<RuleType<u128>>,
    #[serde(alias = "max_size")]
    pub max_size: Option<RuleType<u128>>,
    pub padding: Option<RuleType<bool>>,
}

impl RuleTrait for Base64Rules {
    fn new() -> Self {
        Self {
            url: None,
            min_size: None,
            max_size: None,
            padding: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        match key {
            "url" => {
                self.url = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "minSize" | "min_size" => {
                self.min_size = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "maxSize" | "max_size" => {
                self.max_size = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "padding" => {
                self.padding = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => return Err(format!("Unknown base64 rule: {}", key)),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "url" | "minSize" | "min_size" | "maxSize" | "max_size" | "padding" => true,
            _ => false,
        }
    }
}

impl Mergeable for Base64Rules {
    fn merge(&mut self, other: Base64Rules) -> Result<(), String> {
        if other.url.is_some() {
            if self.url.is_some() {
                return Err("Duplicate rule: url".to_string());
            }
            self.url = other.url;
        }
        if other.min_size.is_some() {
            if self.min_size.is_some() {
                return Err("Duplicate rule: min_size".to_string());
            }
            self.min_size = other.min_size;
        }
        if other.max_size.is_some() {
            if self.max_size.is_some() {
                return Err("Duplicate rule: max_size".to_string());
            }
            self.max_size = other.max_size;
        }
        if other.padding.is_some() {
            if self.padding.is_some() {
                return Err("Duplicate rule: padding".to_string());
            }
            self.padding = other.padding;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Base64Transform {
    pub trim: Option<bool>,
    pub cast: Option<FieldType>,
}

impl Mergeable for Base64Transform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.trim.is_some() {
            if self.trim.is_some() {
                return Err("Duplicate transform: trim".to_string());
            }
            self.trim = other.trim;
        }
        if other.cast.is_some() {
            if self.cast.is_some() {
                return Err("Duplicate transform: cast".to_string());
            }
            self.cast = other.cast;
        }
        Ok(())
    }
}

impl TransformTrait for Base64Transform {
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
            _ => return Err(format!("Unknown base64 transform: {}", key)),
        }
        Ok(())
    }
}
