use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::parse_val;
use crate::ast::Mergeable;
use crate::ast::RuleTrait;
use crate::ast::TransformTrait;

use super::FieldType;
use super::RuleType;
use serde::de::{self};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumRules {
    #[serde(rename = "values", alias = "value")]
    values: Option<RuleType<Vec<StringOrVec>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

impl RuleTrait for EnumRules {
    fn new() -> Self {
        Self { values: None }
    }

    fn set_rule(
        &mut self,
        key: &str,
        value: serde_yaml_ng::Value,
        error: Option<String>,
    ) -> Result<(), String> {
        match key {
            "values" | "value" => {
                self.values = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => return Err(format!("Unknown rule {}", key)),
        };

        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "values" | "value" => true,
            _ => false,
        }
    }
}

impl Mergeable for EnumRules {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.values.is_some() {
            if self.values.is_some() {
                return Err("Duplicate rule: values".to_string());
            } else {
                self.values = other.values;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumTransform {
    pub cast: Option<FieldType>,
}

impl TransformTrait for EnumTransform {
    fn new() -> Self {
        EnumTransform { cast: None }
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        match key {
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown transform {}", key)),
        };
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "cast" => true,
            _ => false,
        }
    }
}

impl Mergeable for EnumTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.cast.is_some() {
            if self.cast.is_some() {
                return Err("Duplicate transform: cast".to_string());
            } else {
                self.cast = other.cast;
            }
        }

        Ok(())
    }
}
