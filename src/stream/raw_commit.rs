use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCommitHeader {
    pub controllers: Vec<String>,
    pub family: String,
    pub schema: String,
    pub tags: Vec<String>,
    pub index: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCommit {
    pub id: ContentIdentifier,
    pub header: RawCommitHeader,
    pub data: Value,
    pub prev: ContentIdentifier,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentIdentifier {}
