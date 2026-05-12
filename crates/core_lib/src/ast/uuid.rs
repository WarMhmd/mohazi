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
pub struct UuidRules {
    #[serde(flatten)]
    pub string_rules: StringRules,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<RuleType<String>>,
}

impl RuleTrait for UuidRules {
    fn new() -> Self {
        Self {
            string_rules: StringRules::new(),
            version: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        if key == "version" {
            let parsed_version: String = parse_val(value)?;
            let valid_versions = ["v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8"];
            if !valid_versions.contains(&parsed_version.as_str()) {
                return Err(format!("Invalid UUID version '{}'. Only v1 through v8 are supported.", parsed_version));
            }
            self.version = Some(RuleType {
                value: parsed_version,
                error,
            });
            return Ok(());
        }
        self.string_rules.set_rule(key, value, error)
    }

    fn is_valid_key(key: &str) -> bool {
        key == "version" || StringRules::is_valid_key(key)
    }
}

impl Mergeable for UuidRules {
    fn merge(&mut self, other: UuidRules) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)?;
        if other.version.is_some() {
            self.version = other.version;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UuidTransform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Mergeable for UuidTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for UuidTransform {
    fn new() -> Self {
        let mut string_transform = StringTransform::new();
        string_transform.cast = Some(FieldType::String);
        UuidTransform { string_transform }
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        self.string_transform.set_transform(key, value)
    }
}
