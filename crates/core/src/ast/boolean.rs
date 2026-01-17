use serde::{Deserialize, Serialize};

use super::FieldType;
use super::RuleType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanRules {
    #[serde(rename = "state", alias = "value")]
    state: Option<RuleType<bool>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BooleanTransform {
    pub cast: Option<FieldType>,
}
