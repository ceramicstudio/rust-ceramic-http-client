use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCommitHeader {
    controllers: Vec<String>,
    index: HashMap<String, Value>,
    #[serde(rename = "forbidControllerChange")]
    family: Option<String>, // deprecated
    schema: Option<String>,    // deprecated
    tags: Option<Vec<String>>, // deprecated
}

impl RawCommitHeader {
    pub fn new(
        controllers: Vec<String>,
        index: HashMap<String, Value>, // This is probably not the correct Value type, but it's hopefully flexible enough for now
        family: Option<String>,
        schema: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            controllers,
            index,
            family,
            schema,
            tags,
        }
    }
}

impl Default for RawCommitHeader {
    fn default() -> Self {
        Self {
            controllers: Vec::new(),
            index: HashMap::new(),
            family: None,
            schema: None,
            tags: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCommit {
    pub id: ContentIdentifier,
    pub header: RawCommitHeader,
    pub data: Value,
    pub prev: ContentIdentifier,
}

impl RawCommit {
    pub fn new(
        id: ContentIdentifier,
        header: RawCommitHeader,
        data: Value,
        prev: ContentIdentifier,
    ) -> Self {
        Self {
            id,
            header,
            data,
            prev,
        }
    }
}

impl Default for RawCommit {
    fn default() -> Self {
        Self {
            id: ContentIdentifier::default(),
            header: RawCommitHeader::default(),
            data: Value::Null,
            prev: ContentIdentifier::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentIdentifier {}

impl Default for ContentIdentifier {
    fn default() -> Self {
        Self {}
    }
}
