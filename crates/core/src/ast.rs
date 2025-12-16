use serde::de::Error;
use serde::{ser, Deserialize, Deserializer, Serialize};
use serde_yaml_ng as serde_yaml;
use std::collections::HashMap;

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
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(alias = "default_error")]
    pub default_error: Option<String>,
    pub rules: Vec<Rules>,
    pub transform: Vec<Transform>,
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // A "Shadow" struct to capture the raw data
        // It looks exactly like Field, but 'rules' is replaced by 'extra'
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct FieldShadow {
            #[serde(rename = "type")]
            field_type: String,
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

        // Filter and collect keys that start with "rules"
        // We collect them into a vector first so we can sort them (rules -> rules1 -> rules2)
        let mut all_entries: Vec<_> = Vec::new();

        for (key, value) in shadow.extra.into_iter() {
            if key.starts_with("rules") {
                all_entries.push((key, value));
            } else if key.starts_with("transform") {
                all_entries.push((key, value));
            }
        }

        // Sort by key to ensure execution order: rules, rules1, rules2...
        all_entries.sort_by(|a, b| a.0.cmp(&b.0));

        let mut field_type = shadow.field_type.as_str();
        for (key, value) in all_entries {
            if key.starts_with("rules") {
                let rule: Rules = match field_type {
                    "string" => {
                        let string_rule: StringRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rules::Single(string_rule)
                    }
                    "number" => {
                        let number_rule: NumberRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rules::Number(number_rule)
                    }
                    _ => {
                        return Err(D::Error::custom(format!(
                            "Unsupported field type '{}' for rules",
                            field_type
                        )));
                    }
                };
                rules.push(rule);
            } else if key.starts_with("transform") {
                let transform_item: Transform = match field_type {
                    "string" => {
                        let string_transform: StringTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::String(string_transform)
                    }
                    "number" => {
                        let number_transform: NumberTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Number(number_transform)
                    }
                    _ => {
                        return Err(D::Error::custom(format!(
                            "Unsupported field type '{}' for transform",
                            field_type
                        )));
                    }
                };
                // check if transform_item has cast
                if let Some(cast) = match &transform_item {
                    Transform::String(s) => s.cast,
                    Transform::Number(n) => n.cast,
                } {
                    field_type = match cast {
                        Cast::String => "string",
                        Cast::Number => "number",
                        Cast::Boolean => "boolean",
                        Cast::Array => "array",
                    };
                }

                // check if transform_item has split and current field_type is string
                if let Transform::String(s) = &transform_item {
                    if s.split.is_some() {
                        field_type = "array";
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
    Single(StringRules),
    Number(NumberRules),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringRules {
    #[serde(alias = "min", alias = "min_length")]
    min_length: Option<RuleType<u128>>,
    #[serde(alias = "max", alias = "max_length")]
    max_length: Option<RuleType<u128>>,
    length: Option<RuleType<u128>>,
    #[serde(alias = "pattern", alias = "regex")]
    regex: Option<RuleType<String>>,
    #[serde(alias = "starts_with")]
    starts_with: Option<RuleType<String>>,
    #[serde(alias = "ends_with")]
    ends_with: Option<RuleType<String>>,
    includes: Option<RuleType<String>>,
    uppercase: Option<RuleType<bool>>,
    lowercase: Option<RuleType<bool>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberRules {
    #[serde(rename = "lt", alias = "lessThan")]
    lt: Option<RuleType<i128>>,
    #[serde(rename = "min", alias = "lte", alias = "lessThanOrEqual")]
    min: Option<RuleType<i128>>,
    #[serde(rename = "gt", alias = "greaterThan")]
    gt: Option<RuleType<i128>>,
    #[serde(rename = "max", alias = "gte", alias = "greaterThanOrEqual")]
    max: Option<RuleType<i128>>,
    equal: Option<RuleType<i128>>,
    positive: Option<RuleType<bool>>,
    #[serde(alias = "nonPositive", alias = "non_positive")]
    nonpositive: Option<RuleType<bool>>,
    negative: Option<RuleType<bool>>,
    #[serde(alias = "nonNegative", alias = "non_negative")]
    nonnegative: Option<RuleType<bool>>,
    #[serde(alias = "divisibleBy", alias = "multipleOf", alias = "multiple_of")]
    multiple_of: Option<RuleType<i128>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RuleType<T> {
    value: T,
    error: Option<String>,
}
// #endregion

// #region Transform
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
    String(StringTransform),
    Number(NumberTransform),
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
    pub cast: Option<Cast>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberTransform {
    pub cast: Option<Cast>,
}
// #endregion

// #region cast
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Cast {
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
