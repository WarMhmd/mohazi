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
pub struct UsernameRules {
    #[serde(flatten)]
    pub string_rules: StringRules,
}

impl RuleTrait for UsernameRules {
    fn new() -> Self {
        let mut string_rules = StringRules::new();
        string_rules.regex = Some(RuleType {
            value: "^[a-zA-Z0-9](?:[._-]?[a-zA-Z0-9]){2,29}$".to_string(),
            error: None,
        });
        Self { string_rules }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        self.string_rules.set_rule(key, value, error)
    }

    fn is_valid_key(key: &str) -> bool {
        StringRules::is_valid_key(key)
    }
}

impl Mergeable for UsernameRules {
    fn merge(&mut self, other: UsernameRules) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsernameTransform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Mergeable for UsernameTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for UsernameTransform {
    fn new() -> Self {
        let mut string_transform = StringTransform::new();
        string_transform.cast = Some(FieldType::String);
        UsernameTransform { string_transform }
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        self.string_transform.set_transform(key, value)
    }
}
