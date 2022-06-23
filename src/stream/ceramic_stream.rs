use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum SignatureStatus {
    GENSIS,
    PARTIAL,
    SIGNED,
}

pub enum AnchorStatus {
    NOTREQUESTED,
    PENDING,
    PROCESSING,
    ANCHORED,
    FAILED,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitHeader {
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
    pub header: CommitHeader,
    pub data: Value,
    pub prev: ContentIdentifier,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentIdentifier {}
