use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRules {
    #[serde(rename = "maxSize", alias = "max_size")]
    pub max_size: Option<RuleType<u64>>,
    #[serde(rename = "minSize", alias = "min_size")]
    pub min_size: Option<RuleType<u64>>,
    #[serde(alias = "extension", alias = "mime")]
    pub extension: Option<RuleType<Vec<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileTransform {
    pub cast: Option<FieldType>,
}
