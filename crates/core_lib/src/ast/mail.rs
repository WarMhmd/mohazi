use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::parse_val;
use crate::ast::TransformTrait;

use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;
use super::StringRules;
use super::StringTransform;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MailRules {
    #[serde(flatten)]
    pub string_rules: StringRules,
    pub allowed_domains: Option<RuleType<Vec<String>>>,
    pub forbidden_domains: Option<RuleType<Vec<String>>>,
}

impl RuleTrait for MailRules {
    fn new() -> Self {
        let mut string_rules = StringRules::new();
        string_rules.regex = Some(RuleType {
            value: "^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9]?(?:\\.[a-zA-Z0-9]?)*$".to_string(),
            error: None,
        });

        Self {
            string_rules,
            allowed_domains: None,
            forbidden_domains: None,
        }
    }

    fn set_rule(&mut self, key: &str, value: Value, error: Option<String>) -> Result<(), String> {
        match key {
            "allowedDomains" | "allowed_domains" => {
                let domains = match value {
                    Value::String(s) => vec![s],
                    Value::Sequence(seq) => {
                        let mut res = Vec::new();
                        for v in seq {
                            res.push(parse_val(v)?);
                        }
                        res
                    }
                    _ => return Err("allowedDomains must be a string or an array of strings".to_string()),
                };
                self.allowed_domains = Some(RuleType {
                    value: domains,
                    error,
                });
            }
            "forbiddenDomains" | "forbidden_domains" => {
                let domains = match value {
                    Value::String(s) => vec![s],
                    Value::Sequence(seq) => {
                        let mut res = Vec::new();
                        for v in seq {
                            res.push(parse_val(v)?);
                        }
                        res
                    }
                    _ => return Err("forbiddenDomains must be a string or an array of strings".to_string()),
                };
                self.forbidden_domains = Some(RuleType {
                    value: domains,
                    error,
                });
            }
            _ => {
                if StringRules::is_valid_key(key) {
                    self.string_rules.set_rule(key, value, error)?;
                } else {
                    return Err(format!("Unknown rule: {}", key));
                }
            }
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "allowedDomains" | "allowed_domains" | "forbiddenDomains" | "forbidden_domains" => true,
            _ => StringRules::is_valid_key(key),
        }
    }
}

impl Mergeable for MailRules {
    fn merge(&mut self, other: MailRules) -> Result<(), String> {
        self.string_rules.merge(other.string_rules)?;

        if other.allowed_domains.is_some() {
            if self.allowed_domains.is_some() {
                return Err("Duplicate rule: allowedDomains".to_string());
            }
            self.allowed_domains = other.allowed_domains;
        }
        if other.forbidden_domains.is_some() {
            if self.forbidden_domains.is_some() {
                return Err("Duplicate rule: forbiddenDomains".to_string());
            }
            self.forbidden_domains = other.forbidden_domains;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MailTransform {
    #[serde(flatten)]
    pub string_transform: StringTransform,
}

impl Default for MailTransform {
    fn default() -> Self {
        Self {
            string_transform: StringTransform {
                cast: Some(FieldType::String),
                ..Default::default()
            },
        }
    }
}

impl Mergeable for MailTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        self.string_transform.merge(other.string_transform)
    }
}

impl TransformTrait for MailTransform {
    fn new() -> Self {
        Self::default()
    }

    fn is_valid_key(key: &str) -> bool {
        StringTransform::is_valid_key(key)
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        self.string_transform.set_transform(key, value)
    }
}
