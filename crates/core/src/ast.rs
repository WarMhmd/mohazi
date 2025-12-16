use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// A simple example struct to test the build
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub forms: HashMap<String, Form>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Form {
    pub fields: HashMap<String, Field>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
    pub default_error: Option<String>,
    pub rules: HashMap<String, Rule>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub message: Option<String>,
    pub params: Option<HashMap<String, String>>,
}

#[test]
pub fn parse_demo() {
    use crate::ast::File;
    use serde_yaml_ng as serde_yaml;

    let yaml_str = include_str!("../test/dump.test.yaml");
    let parsed: File = serde_yaml::from_str(yaml_str).unwrap();
}
