use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use serde_yaml_ng as serde_yaml;
use std::collections::HashMap;

mod file;
mod number;
mod string;

pub use file::{FileRules, FileTransform};
pub use number::{NumberRules, NumberTransform};
pub use string::{StringRules, StringTransform};

/// This trait will be used for each rule to define the merge behaviour
pub trait Mergeable {
    fn merge(&mut self, other: Self, errors: &mut Vec<String>);
}

// this trait will contain the base rule behaviour
pub trait RuleTrait {
    fn new() -> Self;
    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String>;
    fn is_valid_key(key: &str) -> bool;
}

fn parse_val<T>(v: Value) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    serde_yaml::from_value(v).map_err(|e| e.to_string())
}

// A simple example struct to test the build
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct File {
    pub forms: HashMap<String, Form>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Form {
    pub fields: HashMap<String, Field>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(alias = "default_error")]
    pub default_error: Option<String>,
    pub rules: Vec<Rules>,
    pub transform: Vec<Transform>,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_type: FieldType::String,
            required: true,
            default_error: None,
            rules: Vec::new(),
            transform: Vec::new(),
        }
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct FieldShadow {
            #[serde(rename = "type")]
            field_type: FieldType,
            #[serde(default)]
            required: bool,
            default_error: Option<String>,

            // This captures "rules", "rules1", "rules2", etc.
            #[serde(flatten)]
            extra: HashMap<String, serde_yaml::Value>,
        }

        // Parse into the shadow struct first
        let shadow = FieldShadow::deserialize(deserializer)?;

        let mut rules = Vec::new();
        let mut transform = Vec::new();

        let mut all_entries: Vec<_> = Vec::new();

        for (key, value) in shadow.extra.into_iter() {
            if !key.starts_with("rules") && !key.starts_with("transform") {
                // unexpected key
                eprintln!("Warning: unexpected key '{}'", key);
                continue;
            }
            all_entries.push((key, value));
        }

        // Sort by key to ensure execution order: rules, rules1, rules2...
        all_entries.sort_by(|a, b| a.0.cmp(&b.0));

        let mut field_type = shadow.field_type;
        for (key, value) in all_entries {
            if key.starts_with("rules") {
                let rule: Rules = match field_type {
                    FieldType::String => {
                        let string_rule: StringRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rules::String(string_rule)
                    }
                    FieldType::Number => {
                        let number_rule: NumberRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rules::Number(number_rule)
                    }
                    _ => {
                        return Err(D::Error::custom(format!(
                            "Unsupported field type '{:?}' for rules",
                            field_type
                        )));
                    }
                };
                rules.push(rule);
            } else if key.starts_with("transform") {
                let transform_item: Transform = match field_type {
                    FieldType::String => {
                        let string_transform: StringTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::String(string_transform)
                    }
                    FieldType::Number => {
                        let number_transform: NumberTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Number(number_transform)
                    }
                    _ => {
                        return Err(D::Error::custom(format!(
                            "Unsupported field type '{:?}' for transform",
                            field_type
                        )));
                    }
                };
                // check if transform_item has cast
                if let Some(cast) = match &transform_item {
                    Transform::String(s) => s.cast,
                    Transform::Number(n) => n.cast,
                } {
                    field_type = cast;
                }

                // check if transform_item has split and current field_type is string
                if let Transform::String(s) = &transform_item {
                    if s.split.is_some() {
                        field_type = FieldType::Array;
                    }
                }

                transform.push(transform_item);
            }
        }

        Ok(Field {
            field_type: shadow.field_type,
            required: shadow.required,
            default_error: shadow.default_error,
            rules,
            transform,
        })
    }
}

// #region Rules
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Rules {
    String(StringRules),
    Number(NumberRules),
}

impl Rules {
    pub fn is_same_type(&self, other: &Rules) -> bool {
        // std::mem::discriminant checks for enum variant regradless of the inner value
        // (StringRules or NumberRules).
        // in other words, this is the same as:
        // if (self is Rules::T and other is Rules::T), where T is just any entry of the enum
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// This function merges the rules of the same type and returns an error on anything else
    pub fn merge(&mut self, other: Rules, errors: &mut Vec<String>) {
        match (self, other) {
            (Rules::String(a), Rules::String(b)) => a.merge(b, errors),
            (Rules::Number(a), Rules::Number(b)) => a.merge(b, errors),
            // Add new types here (Boolean, Array, etc...)
            _ => {
                errors.push("Unknown rule type to be merged.".to_string());
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleType<T> {
    pub value: T,
    pub error: Option<String>,
}
// #endregion

// #region Transform
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
    String(StringTransform),
    Number(NumberTransform),
}

// #endregion

// #region cast
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
}

// #endregion

#[test]
pub fn parse_demo() {
    use crate::ast::File;

    let yaml_str = include_str!("../test/dump.test.yaml");

    let parsed = serde_yaml::from_str::<File>(yaml_str);

    match parsed {
        Ok(file) => {
            println!("Parsed successfully: {:?}", file);
        }
        Err(e) => {
            println!("Failed to parse YAML: {}", e);
            assert!(false, "YAML parsing failed");
        }
    }
}
