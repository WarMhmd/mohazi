// Note: for adding any type, check the todo[Add]: Types
// todo[Add]: Types create new file in crates\core\src\ast folder

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml_ng as serde_yaml;
use std::collections::HashMap;

// todo[Add]: Types add new mod here and pub use
mod array;
mod boolean;
mod r#enum;
mod file;
mod number;
mod string;

pub use array::{ArrayRules, ArrayTransform};
pub use boolean::{BooleanRules, BooleanTransform};
pub use file::{FileRules, FileTransform};
pub use number::{NumberRules, NumberTransform};
pub use r#enum::{EnumRules, EnumTransform};
pub use string::{StringRules, StringTransform};

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
    pub rules: Vec<Rule>,
    pub transform: Vec<Transform>,
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
                let rule: Rule = match field_type {
                    FieldType::String => {
                        let string_rule: StringRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::String(string_rule)
                    }
                    FieldType::Number => {
                        let number_rule: NumberRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Number(number_rule)
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
                if let Some(cast) = transform_item.get_cast() {
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
pub enum Rule {
    String(StringRules),
    Number(NumberRules),
    File(FileRules),
    Enum(EnumRules),
    Boolean(BooleanRules),
    Array(ArrayRules),
    // todo[Add]: Types more types here
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
    File(FileTransform),
    Enum(EnumTransform),
    Boolean(BooleanTransform),
    Array(ArrayTransform),
    // todo[Add]: Types more types here
}

macro_rules! match_types {
    ($val:expr, $($variant:pat),+ $(,)?) => {
        matches!($val, $(Some($variant))|+)
    };
}

use paste::paste;

macro_rules! make_transform {
    // $t: The type name (e.g., String, Boolean)
    // $obj: The object containing the cast field (e.g., f)
    ($t:ident, $obj:expr) => {
        paste! {
            Transform::$t([<$t Transform>] {
                cast: $obj.cast,
                ..Default::default()
            })
        }
    };
}

impl Transform {
    pub fn get_cast(&self) -> Option<FieldType> {
        match self {
            Transform::String(s) => s.cast,
            Transform::Number(n) => n.cast,
            Transform::File(f) => f.cast,
            Transform::Enum(e) => e.cast,
            Transform::Boolean(b) => b.cast,
            Transform::Array(a) => a.cast,
            // todo[Add]: Types more types here
        }
    }

    pub fn is_valid_cast(&self, array_type: Option<FieldType>) -> bool {
        match self {
            Transform::String(f) => match_types!(f.cast, FieldType::Number, FieldType::Boolean),
            Transform::Number(f) => match_types!(
                f.cast,
                FieldType::Number,
                FieldType::String,
                FieldType::Boolean,
                FieldType::Hex
            ),
            Transform::Boolean(f) => match_types!(f.cast, FieldType::Number, FieldType::String),
            Transform::Enum(f) => match_types!(f.cast, FieldType::Number, FieldType::String,),
            Transform::File(f) => match_types!(f.cast, FieldType::Image, FieldType::Base64,),
            // todo[Add]: Types more types here
            Transform::Array(f) => {
                // create a transform from array_type
                let transform = match array_type {
                    Some(FieldType::String) => make_transform!(String, f),
                    Some(FieldType::Boolean) => make_transform!(Boolean, f),
                    Some(FieldType::Enum) => make_transform!(Enum, f),
                    Some(FieldType::Number) => make_transform!(Number, f),
                    Some(FieldType::File) => make_transform!(File, f),

                    // todo[Add]: Types your mock transform here
                    _ => Transform::Array(ArrayTransform {
                        cast: f.cast,
                        join: None,
                        sum: None,
                    }),
                };
                match transform {
                    Transform::Array(_) => false,
                    _ => transform.is_valid_cast(array_type),
                }
            }
        }
    }
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
    File,
    Enum,
    Image,
    Mail,
    Password,
    Username,
    Url,
    Uuid,
    HttpUrl,
    Base64,
    Jwt,
    Hex,
    Cidrv4,
    Cidrv6,
    Ulid,
    Cuid2,
    Hash,
    Date,
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
