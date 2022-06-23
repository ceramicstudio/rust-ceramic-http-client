use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenesisUniqueValue {
    Null,
    String(String),
    Uint8Array(Vec<u8>),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisHeader {
    controllers: Vec<String>,
    index: HashMap<String, T>,
    #[serde(rename = "forbidControllerChange")]
    forbid_controller_change: bool,

    unique: Option<GenesisUniqueValue>, // Model and ModelInstanceDocument use uint8array instead of string, caip-10 and TileDocument use string
    family: Option<String>,             // deprecated
    model: Option<Vec<u8>>,             // StreamID encoded as byte array js=Uint8Array
    schema: Option<String>,             // deprecated
    tags: Option<Vec<String>>,          // deprecated
}

impl GenesisHeader {
    pub fn new(
        controllers: Vec<String>,
        index: HashMap<String, T>,
        forbid_controller_change: bool,
        unique: Option<GenesisUniqueValue>,
        family: Option<String>,
        model: Option<Vec<u8>>,
        schema: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            controllers,
            index,
            forbid_controller_change,
            unique,
            family,
            model,
            schema,
            tags,
        }
    }

    pub fn controller(&mut self, controller: &str) {
        self.controllers.push(String::from(controller));
    }

    pub fn get_controllers(&self) -> Vec<String> {
        self.controllers.clone()
    }

    pub fn add_index(&mut self, key: &str, value: T) {
        self.index.insert(String::from(key), value);
    }

    pub fn get_index(&self) -> HashMap<String, T> {
        self.index.clone()
    }
    pub fn forbid_controller_change(&self, value: bool) -> bool {
        self.forbid_controller_change = value;
    }
    pub fn get_forbid_controller_change(&self) -> bool {
        self.forbid_controller_change
    }

    pub fn unique(&mut self, value: GenesisUniqueValue) {
        self.unique = Some(value);
    }

    pub fn get_unique(&self) -> Option<GenesisUniqueValue> {
        self.unique.clone()
    }

    pub fn family(&mut self, value: &str) {
        self.family = Some(String::from(value));
    }

    pub fn get_family(&self) -> Option<String> {
        self.family.clone()
    }

    pub fn model(&mut self, value: Vec<u8>) {
        self.model = Some(value);
    }

    pub fn get_model(&self) -> Option<Vec<u8>> {
        self.model.clone()
    }

    pub fn schema(&mut self, value: &str) {
        self.schema = Some(String::from(value));
    }

    pub fn get_schema(&self) -> Option<String> {
        self.schema.clone()
    }
}

impl Default for GenesisHeader {
    fn default() -> Self {
        Self {
            controllers: Vec::new(),
            index: HashMap::new(),
            forbid_controller_change: true,
            unique: None,
            family: None,
            model: None,
            schema: None,
            tags: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisCommit {
    pub header: GenesisHeader,
    pub data: Value,
}

impl GenesisCommit {
    pub fn new(header: GenesisHeader, data: Value) -> Self {
        Self { header, data }
    }

    pub fn get_header(&self) -> &GenesisHeader {
        &self.header
    }

    pub fn get_data(&self) -> &Value {
        &self.data
    }

    pub fn header(&mut self, header: GenesisHeader) {
        self.header = header;
    }

    pub fn data(&mut self, data: Value) {
        self.data = data;
    }
}

impl Default for GenesisCommit {
    fn default() -> Self {
        Self {
            header: GenesisHeader::default(),
            data: Value::Null,
        }
    }
}
