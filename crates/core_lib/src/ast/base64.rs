use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::TransformTrait;
use crate::ast::Mergeable;
use crate::ast::RuleTrait;
use crate::ast::RuleType;
use crate::ast::StringRules;
use crate::ast::StringTransform;
use super::parse_val;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Base64Rules {
    #[serde(flatten)]
    pub string_rules: StringRules,
    pub url: Option<RuleType<bool>>,
    pub min_size: Option<RuleType<u128>>,
    pub max_size: Option<RuleType<u128>>,
}

impl RuleTrait for Base64Rules {
    fn new() -> Self {
        Self {
            string_rules: StringRules::new(),
            url: None,
            min_size: None,
            max_size: None,
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
            _ => return self.string_rules.set_rule(key, value, error),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "url" | "minSize" | "min_size" | "maxSize" | "max_size" => true,
            _ => StringRules::is_valid_key(key),
        }
    }
}

impl Mergeable for Base64Rules {
    fn merge(&mut self, other: Base64Rules) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)?;
        
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
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Base64Transform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Mergeable for Base64Transform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for Base64Transform {
    fn new() -> Self {
        Self {
            string_transform: StringTransform::new(),
        }
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        self.string_transform.set_transform(key, value)
    }
}
