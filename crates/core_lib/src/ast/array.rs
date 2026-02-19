use serde::{Deserialize, Serialize};

use crate::ast::parse_val;
use crate::ast::Mergeable;
use crate::ast::Rule;
use crate::ast::RuleTrait;
use crate::ast::TransformTrait;

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayRules {
    #[serde(rename = "type", alias = "lessThan")]
    pub array_type: RuleType<FieldType>,
    #[serde(rename = "length", alias = "size")]
    pub length: Option<RuleType<usize>>,
    #[serde(rename = "minLength", alias = "min")]
    pub min_length: Option<RuleType<usize>>,
    #[serde(rename = "maxLength", alias = "max")]
    pub max_length: Option<RuleType<usize>>,
}

impl RuleTrait for ArrayRules {
    fn new() -> Self {
        ArrayRules {
            array_type: RuleType {
                value: FieldType::Number, // just as a default value
                error: None,
            },
            length: None,
            min_length: None,
            max_length: None,
        }
    }

    fn set_rule(
        &mut self,
        key: &str,
        value: serde_yaml_ng::Value,
        error: Option<String>,
    ) -> Result<(), String> {
        match key {
            "type" | "lessThan" => {
                self.array_type = RuleType {
                    value: parse_val(value)?,
                    error,
                }
            }
            "length" => {
                self.length = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "min_length" | "minLength" => {
                self.min_length = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "max_length" | "maxLength" => {
                self.max_length = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => {
                return Err(format!("Unknown rule {}", key));
            }
        }

        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "type" | "lessThan" => true,
            "length" => true,
            "min_length" => true,
            "max_length" => true,
            _ => false,
        }
    }
}

impl Mergeable for ArrayRules {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.length.is_some() {
            if self.length.is_some() {
                return Err("Duplicate rule: length".to_string());
            } else {
                self.length = other.length;
            }
        }
        if other.min_length.is_some() {
            if self.min_length.is_some() {
                return Err("Duplicate rule: min_length".to_string());
            } else {
                self.min_length = other.min_length;
            }
        }
        if other.max_length.is_some() {
            if self.max_length.is_some() {
                return Err("Duplicate rule: max_length".to_string());
            } else {
                self.max_length = other.max_length;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArrayTransform {
    pub join: Option<String>,
    pub sum: Option<bool>,
    pub cast: Option<FieldType>,
}

impl TransformTrait for ArrayTransform {
    fn new() -> Self {
        ArrayTransform {
            join: None,
            sum: None,
            cast: None,
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "join" => true,
            "sum" => true,
            "cast" => true,
            _ => false,
        }
    }

    fn set_transform(&mut self, key: &str, value: serde_yaml_ng::Value) -> Result<(), String> {
        match key {
            "join" => self.join = Some(parse_val(value)?),
            "sum" => self.sum = Some(parse_val(value)?),
            "cast" => self.cast = Some(parse_val(value)?),
            _ => return Err(format!("Unknown transform {}", key)),
        }

        Ok(())
    }
}

impl Mergeable for ArrayTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.cast.is_some() {
            if self.cast.is_some() {
                return Err("Duplicate transform: cast".to_string());
            } else {
                self.cast = other.cast;
            }
        }
        if other.join.is_some() {
            if self.join.is_some() {
                return Err("Duplicate transform: join".to_string());
            } else {
                self.join = other.join;
            }
        }
        if other.sum.is_some() {
            if self.sum.is_some() {
                return Err("Duplicate transform: sum".to_string());
            } else {
                self.sum = other.sum;
            }
        }

        Ok(())
    }
}
