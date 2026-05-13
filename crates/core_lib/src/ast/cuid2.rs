use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::TransformTrait;
use super::parse_val;
use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;
use super::StringRules;
use super::StringTransform;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cuid2Rules {
    #[serde(flatten)]
    pub string_rules: StringRules,
}

impl RuleTrait for Cuid2Rules {
    fn new() -> Self {
        Self {
            string_rules: StringRules {
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
                ..StringRules::new()
            },
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        self.string_rules.set_rule(key, value, error)
    }

    fn is_valid_key(key: &str) -> bool {
        StringRules::is_valid_key(key)
    }
}

impl Mergeable for Cuid2Rules {
    fn merge(&mut self, other: Cuid2Rules) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cuid2Transform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Mergeable for Cuid2Transform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for Cuid2Transform {
    fn new() -> Self {
        let mut string_transform = StringTransform::new();
        string_transform.cast = Some(FieldType::String);
        Cuid2Transform { string_transform }
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        self.string_transform.set_transform(key, value)
    }
}
