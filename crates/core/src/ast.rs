use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use serde_yaml_ng as serde_yaml;
use std::collections::HashMap;

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
            if key.starts_with("rules") {
                all_entries.push((key, value));
            } else if key.starts_with("transform") {
                all_entries.push((key, value));
            }
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringRules {
    #[serde(alias = "min", alias = "min_length")]
    pub min_length: Option<RuleType<u128>>,
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

impl StringRules {
    pub fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            regex: None,
            starts_with: None,
            ends_with: None,
            includes: None,
            uppercase: None,
            lowercase: None,
        }
    }

    pub fn set_rule(
        &mut self,
        key: &str,
        value: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        let rule_err = error; // alias for clarity

        match key {
            "minLength" | "min_length" => {
                self.min_length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "maxLength" | "max_length" => {
                self.max_length = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            // ... Add cases for starts_with, uppercase, etc.
            _ => return Err(format!("Unknown string rule: {}", key)),
        }
        Ok(())
    }
    /// Checks if a string key is a valid rule name, respecting
    /// camelCase renaming and your custom aliases.
    pub fn is_valid_key(key: &str) -> bool {
        match key {
            // Field: min_length (camelCase: minLength)
            "minLength" | "min" | "min_length" => true,

            // Field: max_length (camelCase: maxLength)
            "maxLength" | "max" | "max_length" => true,

            // Field: length (camelCase: length)
            "length" => true,

            // Field: regex (camelCase: regex)
            "regex" | "pattern" => true,

            // Field: starts_with (camelCase: startsWith)
            "startsWith" | "starts_with" => true,

            // Field: ends_with (camelCase: endsWith)
            "endsWith" | "ends_with" => true,

            // Field: includes (camelCase: includes)
            "includes" => true,

            // Field: uppercase (camelCase: uppercase)
            "uppercase" => true,

            // Field: lowercase (camelCase: lowercase)
            "lowercase" => true,

            _ => false,
        }
    }

    pub fn merge(&mut self, other: StringRules) {
        if other.min_length.is_some() {
            self.min_length = other.min_length;
        }
        if other.max_length.is_some() {
            self.max_length = other.max_length;
        }
        if other.length.is_some() {
            self.length = other.length;
        }
        if other.regex.is_some() {
            self.regex = other.regex;
        }
        if other.starts_with.is_some() {
            self.starts_with = other.starts_with;
        }
        if other.ends_with.is_some() {
            self.ends_with = other.ends_with;
        }
        if other.includes.is_some() {
            self.includes = other.includes;
        }
        if other.uppercase.is_some() {
            self.uppercase = other.uppercase;
        }
        if other.lowercase.is_some() {
            self.lowercase = other.lowercase;
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberRules {
    #[serde(rename = "lt", alias = "lessThan")]
    lt: Option<RuleType<i128>>,
    #[serde(rename = "max", alias = "lte", alias = "lessThanOrEqual")]
    max: Option<RuleType<i128>>,
    #[serde(rename = "gt", alias = "greaterThan")]
    gt: Option<RuleType<i128>>,
    #[serde(rename = "min", alias = "gte", alias = "greaterThanOrEqual")]
    min: Option<RuleType<i128>>,
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

impl NumberRules {
    pub fn new() -> Self {
        Self {
            lt: None,
            max: None,
            gt: None,
            min: None,
            equal: None,
            positive: None,
            nonpositive: None,
            negative: None,
            nonnegative: None,
            multiple_of: None,
        }
    }

    pub fn set_rule(
        &mut self,
        key: &str,
        value: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        let rule_err = error;

        match key {
            "min" | "gte" => {
                self.min = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            "max" | "lte" => {
                self.max = Some(RuleType {
                    value: parse_val(value)?,
                    error: rule_err,
                });
            }
            // ... Add cases for positive, multipleOf, etc.
            _ => return Err(format!("Unknown number rule: {}", key)),
        }
        Ok(())
    }

    pub fn is_valid_key(key: &str) -> bool {
        match key {
            // Field: lt (Renamed to "lt", Alias: "lessThan")
            "lt" | "lessThan" => true,

            // Field: max (Renamed to "max", Aliases: "lte", "lessThanOrEqual")
            "max" | "lte" | "lessThanOrEqual" => true,

            // Field: gt (Renamed to "gt", Alias: "greaterThan")
            "gt" | "greaterThan" => true,

            // Field: min (Renamed to "min", Aliases: "gte", "greaterThanOrEqual")
            "min" | "gte" | "greaterThanOrEqual" => true,

            // Field: equal (camelCase: equal)
            "equal" => true,

            // Field: positive (camelCase: positive)
            "positive" => true,

            // Field: nonpositive (camelCase: nonpositive, Aliases: "nonPositive", "non_positive")
            "nonpositive" | "nonPositive" | "non_positive" => true,

            // Field: negative (camelCase: negative)
            "negative" => true,

            // Field: nonnegative (camelCase: nonnegative, Aliases: "nonNegative", "non_negative")
            "nonnegative" | "nonNegative" | "non_negative" => true,

            // Field: multiple_of (camelCase: multipleOf, Aliases: "divisibleBy", "multipleOf", "multiple_of")
            "multipleOf" | "divisibleBy" | "multiple_of" => true,

            _ => false,
        }
    }

    pub fn merge(&mut self, other: NumberRules) {
        if other.lt.is_some() {
            self.lt = other.lt;
        }
        if other.max.is_some() {
            self.max = other.max;
        }
        if other.gt.is_some() {
            self.gt = other.gt;
        }
        if other.min.is_some() {
            self.min = other.min;
        }
        if other.equal.is_some() {
            self.equal = other.equal;
        }
        if other.positive.is_some() {
            self.positive = other.positive;
        }
        if other.nonpositive.is_some() {
            self.nonpositive = other.nonpositive;
        }
        if other.negative.is_some() {
            self.negative = other.negative;
        }
        if other.nonnegative.is_some() {
            self.nonnegative = other.nonnegative;
        }
        if other.multiple_of.is_some() {
            self.multiple_of = other.multiple_of;
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
    pub cast: Option<FieldType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberTransform {
    pub cast: Option<FieldType>,
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
