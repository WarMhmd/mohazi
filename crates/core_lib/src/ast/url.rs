use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::TransformTrait;
use super::parse_val;
use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrlRules {
    #[serde(alias = "min", alias = "min_length")]
    pub min_length: Option<RuleType<u128>>,
    #[serde(alias = "max", alias = "max_length")]
    pub max_length: Option<RuleType<u128>>,
    pub length: Option<RuleType<u128>>,
    #[serde(alias = "pattern", alias = "regex")]
    pub regex: Option<RuleType<String>>,
    #[serde(alias = "starts_with")]
    pub starts_with: Option<RuleType<String>>,
    #[serde(alias = "ends_with")]
    pub ends_with: Option<RuleType<String>>,
    pub includes: Option<RuleType<String>>,
    pub uppercase: Option<RuleType<bool>>,
    pub lowercase: Option<RuleType<bool>>,
    
    // URL specific
    pub allowed_hosts: Option<RuleType<Vec<String>>>,
    pub https_only: Option<RuleType<bool>>,
}

impl RuleTrait for UrlRules {
    fn new() -> Self {
        Self {
            min_length: None,
            max_length: None,
            length: None,
            starts_with: None,
            ends_with: None,
            includes: None,
            uppercase: None,
            lowercase: None,
            regex: Some(RuleType {
                value: "^https?:\\/\\/(?:www\\.)?[-a-zA-Z0-9@:%.\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b(?:[-a-zA-Z0-9()@:%\\+.~#?&\\/=]*)$".to_string(),
                error: None,
            }),
            allowed_hosts: None,
            https_only: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        match key {
            "minLength" | "min_length" | "min" => {
                self.min_length = Some(RuleType { value: parse_val(value)?, error });
            }
            "maxLength" | "max_length" | "max" => {
                self.max_length = Some(RuleType { value: parse_val(value)?, error });
            }
            "length" => {
                self.length = Some(RuleType { value: parse_val(value)?, error });
            }
            "startsWith" | "starts_with" => {
                self.starts_with = Some(RuleType { value: parse_val(value)?, error });
            }
            "endsWith" | "ends_with" => {
                self.ends_with = Some(RuleType { value: parse_val(value)?, error });
            }
            "includes" => {
                self.includes = Some(RuleType { value: parse_val(value)?, error });
            }
            "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" => {
                self.lowercase = Some(RuleType { value: parse_val(value)?, error });
            }
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" => {
                self.uppercase = Some(RuleType { value: parse_val(value)?, error });
            }
            "pattern" | "regex" => {
                self.regex = Some(RuleType { value: parse_val(value)?, error });
            }
            "allowedHosts" | "allowed_hosts" => {
                let parsed_val: Value = value;
                let final_val: Vec<String> = if parsed_val.is_sequence() {
                    parse_val(parsed_val)?
                } else if let Some(s) = parsed_val.as_str() {
                    vec![s.to_string()]
                } else if let Ok(s) = parse_val::<String>(parsed_val.clone()) {
                    vec![s]
                } else {
                    return Err("allowedHosts must be a string or array of strings".to_string());
                };
                self.allowed_hosts = Some(RuleType { value: final_val, error });
            }
            "httpsOnly" | "https_only" => {
                self.https_only = Some(RuleType { value: parse_val(value)?, error });
            }
            _ => return Err(format!("Unknown url rule: {}", key)),
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "minLength" | "min" | "min_length" | "maxLength" | "max" | "max_length" | "length" |
            "startsWith" | "starts_with" | "endsWith" | "ends_with" | "includes" |
            "uppercase" | "lowercase" | "pattern" | "regex" | 
            "allowedHosts" | "allowed_hosts" | "httpsOnly" | "https_only" => true,
            _ => false,
        }
    }
}

impl Mergeable for UrlRules {
    fn merge(&mut self, other: UrlRules) -> Result<(), String> {
        if other.min_length.is_some() { self.min_length = other.min_length; }
        if other.max_length.is_some() { self.max_length = other.max_length; }
        if other.length.is_some() { self.length = other.length; }
        if other.starts_with.is_some() { self.starts_with = other.starts_with; }
        if other.ends_with.is_some() { self.ends_with = other.ends_with; }
        if other.includes.is_some() { self.includes = other.includes; }
        if other.uppercase.is_some() { self.uppercase = other.uppercase; }
        if other.lowercase.is_some() { self.lowercase = other.lowercase; }
        if other.regex.is_some() { self.regex = other.regex; }
        if other.allowed_hosts.is_some() { self.allowed_hosts = other.allowed_hosts; }
        if other.https_only.is_some() { self.https_only = other.https_only; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UrlTransform {
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

impl Mergeable for UrlTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.trim.is_some() { self.trim = other.trim; }
        if other.to_lowercase.is_some() { self.to_lowercase = other.to_lowercase; }
        if other.to_uppercase.is_some() { self.to_uppercase = other.to_uppercase; }
        if other.split.is_some() { self.split = other.split; }
        if other.cast.is_some() { self.cast = other.cast; }
        Ok(())
    }
}

impl TransformTrait for UrlTransform {
    fn new() -> Self {
        UrlTransform {
            trim: None,
            to_lowercase: None,
            to_uppercase: None,
            split: None,
            cast: None,
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "trim" | "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" |
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" | "split" | "cast" => true,
            _ => false,
        }
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        match key {
            "trim" => self.trim = Some(parse_val(value)?),
            "toLowerCase" | "to_lower_case" | "lowercase" | "to_lowercase" => {
                self.to_lowercase = Some(parse_val(value)?)
            }
            "toUpperCase" | "to_upper_case" | "uppercase" | "to_uppercase" => {
                self.to_uppercase = Some(parse_val(value)?)
            }
            "split" => self.split = Some(parse_val(value)?),
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown url transform: {}", key)),
        }
        Ok(())
    }
}
