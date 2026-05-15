use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::ast::parse_val;
use crate::ast::TransformTrait;

use super::FieldType;
use super::Mergeable;
use super::RuleTrait;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageRules {
    #[serde(rename = "maxSize", alias = "max_size")]
    pub max_size: Option<RuleType<u64>>,
    #[serde(rename = "minSize", alias = "min_size")]
    pub min_size: Option<RuleType<u64>>,
    #[serde(alias = "mime")]
    pub extension: Option<RuleType<Vec<String>>>,
    pub width: Option<RuleType<u64>>,
    pub height: Option<RuleType<u64>>,
    #[serde(rename = "minWidth")]
    pub min_width: Option<RuleType<u64>>,
    #[serde(rename = "maxWidth")]
    pub max_width: Option<RuleType<u64>>,
    #[serde(rename = "minHeight")]
    pub min_height: Option<RuleType<u64>>,
    #[serde(rename = "maxHeight")]
    pub max_height: Option<RuleType<u64>>,
    pub ratio: Option<RuleType<String>>,
}

impl RuleTrait for ImageRules {
    fn new() -> Self {
        Self {
            max_size: None,
            min_size: None,
            extension: Some(RuleType {
                value: vec![
                    "jpg".to_string(),
                    "jpeg".to_string(),
                    "png".to_string(),
                    "webp".to_string(),
                    "avif".to_string(),
                    "heic".to_string(),
                    "heif".to_string(),
                ],
                error: None,
            }),
            width: None,
            height: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            ratio: None,
        }
    }

    fn set_rule(
        &mut self,
        key: &str,
        value: Value,
        error: Option<String>,
    ) -> Result<(), String> {
        match key {
            "maxSize" | "max_size" => {
                self.max_size = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "minSize" | "min_size" => {
                self.min_size = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "extension" | "mime" => {
                self.extension = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "width" => {
                self.width = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "height" => {
                self.height = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "minWidth" | "min_width" => {
                self.min_width = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "maxWidth" | "max_width" => {
                self.max_width = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "minHeight" | "min_height" => {
                self.min_height = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "maxHeight" | "max_height" => {
                self.max_height = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            "ratio" => {
                self.ratio = Some(RuleType {
                    value: parse_val(value)?,
                    error,
                });
            }
            _ => {
                return Err(format!("Unknown rule: {}", key));
            }
        }
        Ok(())
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "maxSize" | "max_size" | "minSize" | "min_size" | "extension" | "mime" | "width" | "height" | "minWidth" | "min_width" | "maxWidth" | "max_width"
            | "minHeight" | "min_height" | "maxHeight" | "max_height" | "ratio" => true,
            _ => false,
        }
    }
}

impl Mergeable for ImageRules {
    fn merge(&mut self, other: ImageRules) -> Result<(), String> {
        if other.max_size.is_some() {
            self.max_size = other.max_size;
        }
        if other.min_size.is_some() {
            self.min_size = other.min_size;
        }
        if other.extension.is_some() {
            self.extension = other.extension;
        }
        if other.width.is_some() {
            if self.width.is_some() {
                return Err("Duplicate rule: width".to_string());
            }
            self.width = other.width;
        }
        if other.height.is_some() {
            if self.height.is_some() {
                return Err("Duplicate rule: height".to_string());
            }
            self.height = other.height;
        }
        if other.min_width.is_some() {
            if self.min_width.is_some() {
                return Err("Duplicate rule: minWidth".to_string());
            }
            self.min_width = other.min_width;
        }
        if other.max_width.is_some() {
            if self.max_width.is_some() {
                return Err("Duplicate rule: maxWidth".to_string());
            }
            self.max_width = other.max_width;
        }
        if other.min_height.is_some() {
            if self.min_height.is_some() {
                return Err("Duplicate rule: minHeight".to_string());
            }
            self.min_height = other.min_height;
        }
        if other.max_height.is_some() {
            if self.max_height.is_some() {
                return Err("Duplicate rule: maxHeight".to_string());
            }
            self.max_height = other.max_height;
        }
        if other.ratio.is_some() {
            if self.ratio.is_some() {
                return Err("Duplicate rule: ratio".to_string());
            }
            self.ratio = other.ratio;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageTransform {
    pub cast: Option<FieldType>,
    pub rename: Option<String>,
}

impl Mergeable for ImageTransform {
    fn merge(&mut self, other: Self) -> Result<(), String> {
        if other.cast.is_some() {
            if self.cast.is_some() {
                return Err("Duplicate transform: cast".to_string());
            }
            self.cast = other.cast;
        }
        if other.rename.is_some() {
            if self.rename.is_some() {
                return Err("Duplicate transform: rename".to_string());
            }
            self.rename = other.rename;
        }

        Ok(())
    }
}

impl TransformTrait for ImageTransform {
    fn new() -> Self {
        ImageTransform {
            cast: None,
            rename: None,
        }
    }

    fn is_valid_key(key: &str) -> bool {
        match key {
            "cast" | "rename" => true,
            _ => false,
        }
    }

    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String> {
        match key {
            "cast" => self.cast = Some(parse_val(value)?),
            "rename" => self.rename = Some(parse_val(value)?),
            _ => return Err(format!("Unknown transform {}", key)),
        }
        Ok(())
    }
}
