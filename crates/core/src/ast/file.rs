use serde::{Deserialize, Serialize};

use crate::ast::parse_val;

use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRules {
    #[serde(rename = "maxSize", alias = "max_size")]
    pub max_size: Option<RuleType<u64>>,
    #[serde(rename = "minSize", alias = "min_size")]
    pub min_size: Option<RuleType<u64>>,
    #[serde(alias = "extension", alias = "mime")]
    pub extension: Option<RuleType<Vec<String>>>,
}

impl RuleTrait for FileRules {
    fn new() -> Self {
        Self {
            max_size: None,
            min_size: None,
            extension: None,
        }
    }

    fn set_rule(
        &mut self,
        key: &str,
        value: serde_yaml_ng::Value,
        error: Option<String>,
    ) -> Result<(), String> {
        let rule_err = error;

        match key {
            "maxSize" | "max_size" => {
                self.max_size = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "minSize" | "min_size" => {
                self.min_size = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "extension" | "mime" => {
                self.extension = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            _ => return Err(format!("Unknown rule: {}", key)),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "maxSize" | "max_size" => true,
            "minSize" | "min_size" => true,
            "extension" => true,
            _ => false,
        }
    }
}

impl Mergeable for FileRules {
    fn merge(&mut self, other: FileRules, errors: &mut Vec<String>) {
        if other.max_size.is_some() {
            if self.max_size.is_some() {
                errors.push("Duplicate rule: maxSize".to_string());
            } else {
                self.max_size = other.max_size;
            }
        }
        if other.min_size.is_some() {
            if self.min_size.is_some() {
                errors.push("Duplicate rule: minSize".to_string());
            } else {
                self.min_size = other.min_size;
            }
        }
        if other.extension.is_some() {
            if self.extension.is_some() {
                errors.push("Duplicate rule: extension".to_string());
            } else {
                self.extension = other.extension;
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTransform {
    pub cast: Option<FieldType>,
}
