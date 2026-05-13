// Note: for adding any type, check the todo[Add]: Types
// todo[Add]: Types create new file in crates\core\src\ast folder

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use serde_yaml_ng as serde_yaml;
use std::collections::HashMap;

// todo[Add]: Types add new mod here and pub use
mod array;
mod base64;
mod boolean;
mod r#enum;
mod file;
mod image;
mod mail;
mod number;
mod string;
mod username;
mod uuid;

pub use array::{ArrayRules, ArrayTransform};
pub use base64::{Base64Rules, Base64Transform};
pub use boolean::{BooleanRules, BooleanTransform};
pub use file::{FileRules, FileTransform};
pub use image::{ImageRules, ImageTransform};
pub use mail::{MailRules, MailTransform};
pub use number::{NumberRules, NumberTransform};
pub use r#enum::{EnumRules, EnumTransform};
pub use string::{StringRules, StringTransform};
pub use username::{UsernameRules, UsernameTransform};
pub use uuid::{UuidRules, UuidTransform};

/// This trait will be used for each rule to define the merge behaviour
pub trait Mergeable {
    fn merge(&mut self, other: Self) -> Result<(), String>;
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
    pub rules: Vec<Rule>,
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
                    FieldType::File => {
                        let file_rule: FileRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::File(file_rule)
                    }
                    FieldType::Image => {
                        let image_rule: ImageRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Image(image_rule)
                    }
                    FieldType::Mail => {
                        let mail_rule: MailRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Mail(mail_rule)
                    }
                    FieldType::Username => {
                        let username_rule: UsernameRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Username(username_rule)
                    }
                    FieldType::Uuid => {
                        let uuid_rule: UuidRules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Uuid(uuid_rule)
                    }
                    FieldType::Base64 => {
                        let base64_rule: Base64Rules =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Rule::Base64(base64_rule)
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
                    FieldType::File => {
                        let file_transform: FileTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::File(file_transform)
                    }
                    FieldType::Image => {
                        let image_transform: ImageTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Image(image_transform)
                    }
                    FieldType::Mail => {
                        let mail_transform: MailTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Mail(mail_transform)
                    }
                    FieldType::Username => {
                        let username_transform: UsernameTransform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Username(username_transform)
                    }
                    FieldType::Base64 => {
                        let base64_transform: Base64Transform =
                            serde_yaml::from_value(value).map_err(D::Error::custom)?;
                        Transform::Base64(base64_transform)
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Rule {
    String(StringRules),
    Number(NumberRules),
    File(FileRules),
    Enum(EnumRules),
    Boolean(BooleanRules),
    Array(ArrayRules),
    Image(ImageRules),
    Mail(MailRules),
    Username(UsernameRules),
    Uuid(UuidRules),
    Base64(Base64Rules),
    // todo[Add]: Type
}

impl Rule {
    pub fn is_same_type(&self, other: &Rule) -> bool {
        // std::mem::discriminant checks for enum variant regradless of the inner value
        // (StringRules or NumberRules).
        // in other words, this is the same as:
        // if (self is Rule::T and other is Rule::T), where T is just any entry of the enum
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// This function merges the rules of the same type and returns an error on anything else
    pub fn merge(&mut self, other: Rule) -> Result<(), String> {
        match (self, other) {
            (Rule::String(a), Rule::String(b)) => a.merge(b),
            (Rule::Number(a), Rule::Number(b)) => a.merge(b),
            (Rule::File(a), Rule::File(b)) => a.merge(b),
            (Rule::Boolean(a), Rule::Boolean(b)) => a.merge(b),
            (Rule::Array(a), Rule::Array(b)) => a.merge(b),
            (Rule::Enum(a), Rule::Enum(b)) => a.merge(b),
            (Rule::Image(a), Rule::Image(b)) => a.merge(b),
            (Rule::Mail(a), Rule::Mail(b)) => a.merge(b),
            (Rule::Username(a), Rule::Username(b)) => a.merge(b),
            (Rule::Uuid(a), Rule::Uuid(b)) => a.merge(b),
            (Rule::Base64(a), Rule::Base64(b)) => a.merge(b),
            // todo[Add]: Type
            _ => Err("Unknown rule type to be merged.".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RuleType<T> {
    pub value: T,
    pub error: Option<String>,
}
// #endregion

// #region Transform
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Transform {
    String(StringTransform),
    Number(NumberTransform),
    File(FileTransform),
    Enum(EnumTransform),
    Boolean(BooleanTransform),
    Array(ArrayTransform),
    Image(ImageTransform),
    Mail(MailTransform),
    Username(UsernameTransform),
    Uuid(UuidTransform),
    Base64(Base64Transform),
    // todo[Add]: Type
}

pub trait TransformTrait {
    fn new() -> Self;
    fn is_valid_key(key: &str) -> bool;
    fn set_transform(&mut self, key: &str, value: Value) -> Result<(), String>;
}

impl Transform {
    pub fn is_same_type(&self, other: &Transform) -> bool {
        // std::mem::discriminant checks for enum variant regradless of the inner value
        // (StringRules or NumberRules).
        // in other words, this is the same as:
        // if (self is Rules::T and other is Rules::T), where T is just any entry of the enum
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    /// This function merges the rules of the same type and returns an error on anything else
    pub fn merge(&mut self, other: Transform) -> Result<(), String> {
        match (self, other) {
            // todo[Add]: Type
            (Transform::String(a), Transform::String(b)) => a.merge(b),
            (Transform::Number(a), Transform::Number(b)) => a.merge(b),
            (Transform::File(a), Transform::File(b)) => a.merge(b),
            (Transform::Image(a), Transform::Image(b)) => a.merge(b),
            (Transform::Mail(a), Transform::Mail(b)) => a.merge(b),
            (Transform::Username(a), Transform::Username(b)) => a.merge(b),
            (Transform::Uuid(a), Transform::Uuid(b)) => a.merge(b),
            (Transform::Base64(a), Transform::Base64(b)) => a.merge(b),
            _ => Err("Unknown rule type to be merged.".to_string()),
        }
    }
}

macro_rules! match_types {
    ($val:expr, $($variant:pat),+ $(,)?) => {
        matches!($val, $(Some($variant))|+)
    };
}

use paste::paste;

use crate::vis_parser::ParserError;

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
            Transform::Image(i) => i.cast,
            Transform::Mail(m) => m.string_transform.cast,
            Transform::Username(u) => u.string_transform.cast,
            Transform::Uuid(u) => u.string_transform.cast,
            Transform::Base64(b) => b.string_transform.cast,
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
            Transform::Image(f) => {
                match_types!(f.cast, FieldType::File, FieldType::Base64)
            }
            Transform::Mail(f) => {
                match_types!(f.string_transform.cast, FieldType::String)
            }
            Transform::Username(f) => {
                match_types!(f.string_transform.cast, FieldType::String)
            }
            Transform::Uuid(f) => {
                match_types!(f.string_transform.cast, FieldType::String)
            }
            Transform::Base64(f) => {
                match_types!(f.string_transform.cast, FieldType::String)
            }
            // todo[Add]: Types more types here
            Transform::Array(f) => {
                // create a transform from array_type
                let transform = match array_type {
                    Some(FieldType::String) => make_transform!(String, f),
                    Some(FieldType::Boolean) => make_transform!(Boolean, f),
                    Some(FieldType::Enum) => make_transform!(Enum, f),
                    Some(FieldType::Number) => make_transform!(Number, f),
                    Some(FieldType::File) => make_transform!(File, f),
                    Some(FieldType::Image) => make_transform!(Image, f),
                    Some(FieldType::Mail) => Transform::Mail(MailTransform {
                        string_transform: StringTransform {
                            cast: f.cast,
                            ..Default::default()
                        }
                    }),
                    Some(FieldType::Username) => Transform::Username(UsernameTransform {
                        string_transform: StringTransform {
                            cast: f.cast,
                            ..Default::default()
                        }
                    }),
                    Some(FieldType::Uuid) => Transform::Uuid(UuidTransform {
                        string_transform: StringTransform {
                            cast: f.cast,
                            ..Default::default()
                        }
                    }),
                    Some(FieldType::Base64) => Transform::Base64(Base64Transform {
                        string_transform: StringTransform {
                            cast: f.cast,
                            ..Default::default()
                        }
                    }),

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

impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        match s {
            "string" => Some(FieldType::String),
            "number" => Some(FieldType::Number),
            "boolean" => Some(FieldType::Boolean),
            "array" => Some(FieldType::Array),
            "file" => Some(FieldType::File),
            "enum" => Some(FieldType::Enum),
            "image" => Some(FieldType::Image),
            "mail" => Some(FieldType::Mail),
            "password" => Some(FieldType::Password),
            "username" => Some(FieldType::Username),
            "url" => Some(FieldType::Url),
            "uuid" => Some(FieldType::Uuid),
            "http_url" => Some(FieldType::HttpUrl),
            "base64" => Some(FieldType::Base64),
            "jwt" => Some(FieldType::Jwt),
            "hex" => Some(FieldType::Hex),
            "cidrv4" => Some(FieldType::Cidrv4),
            "cidrv6" => Some(FieldType::Cidrv6),
            "ulid" => Some(FieldType::Ulid),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::String => "string",
            FieldType::Number => "number",
            FieldType::Boolean => "boolean",
            FieldType::Array => "array",
            FieldType::File => "file",
            FieldType::Enum => "enum",
            FieldType::Image => "image",
            FieldType::Mail => "mail",
            FieldType::Password => "password",
            FieldType::Username => "username",
            FieldType::Url => "url",
            FieldType::Uuid => "uuid",
            FieldType::HttpUrl => "http_url",
            FieldType::Base64 => "base64",
            FieldType::Jwt => "jwt",
            FieldType::Hex => "hex",
            FieldType::Cidrv4 => "cidrv4",
            FieldType::Cidrv6 => "cidrv6",
            FieldType::Ulid => "ulid",
            FieldType::Cuid2 => "cuid2",
            FieldType::Hash => "hash",
            FieldType::Date => "date",
        }
    }
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
