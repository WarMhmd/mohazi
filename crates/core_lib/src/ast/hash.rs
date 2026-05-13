use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use super::{parse_val, FieldType, Mergeable, RuleTrait, RuleType, TransformTrait};
use super::string::{StringRules, StringTransform};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashRules {
    #[serde(flatten)]
    pub string_rules: StringRules,
    
    pub algorithm: Option<RuleType<String>>,
}

impl RuleTrait for HashRules {
    fn new() -> Self {
        Self {
            string_rules: StringRules::new(),
            algorithm: None,
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
            _ => {
                if StringRules::is_valid_key(key) {
                    self.string_rules.set_rule(key, value, error)
                } else {
                    Err(format!("Unknown hash rule: {}", key))
                }
            }
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "algorithm" => true,
            _ => StringRules::is_valid_key(key),
        }
    }
}

impl Mergeable for HashRules {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)?;
        if other.algorithm.is_some() {
            if self.algorithm.is_some() {
                return Err("Duplicate rule: algorithm".to_string());
            } else {
                self.algorithm = other.algorithm;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HashTransform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Mergeable for HashTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for HashTransform {
    fn new() -> Self {
        Self {
            string_transform: StringTransform::new(),
        }
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        if StringTransform::is_valid_key(key) {
            self.string_transform.set_transform(key, value)
        } else {
            Err(format!("Unknown hash transform: {}", key))
        }
    }
}