use serde::{Deserialize, Serialize};

use crate::ast::parse_val;
use crate::ast::Mergeable;
use crate::ast::RuleTrait;
use crate::ast::TransformTrait;

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BooleanRules {
    #[serde(rename = "state", alias = "value")]
    state: Option<RuleType<bool>>,
}

impl RuleTrait for BooleanRules {
    fn new() -> Self {
        Self { state: None }
    }

    fn set_rule(
        &mut self,
        key: &str,
        value: serde_yaml_ng::Value,
        error: Option<String>,
    ) -> Result<(), String> {
        match key {
            "state" | "value" => {
                self.state = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => return Err(format!("Unknown rule {}", key)),
        }

        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "state" | "value" => true,
            _ => false,
        }
    }
}

impl Mergeable for BooleanRules {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.state.is_some() {
            if self.state.is_some() {
                return Err("Duplicate rule: state".to_string());
            } else {
                self.state = other.state;
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BooleanTransform {
    pub cast: Option<FieldType>,
}

impl TransformTrait for BooleanTransform {
    fn new() -> Self {
        BooleanTransform { cast: None }
    }

    fn set_transform(&mut self, key: &str, value: serde_yaml_ng::Value) -> Result<(), String> {
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

impl Mergeable for BooleanTransform {
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
