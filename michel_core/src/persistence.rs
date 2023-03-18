use crate::plugins::wasi::types;
use crate::plugins::wasi::types::Value;
use anyhow::Result;

pub(crate) type PersistedDocument = serde_json::Map<String, serde_json::Value>;

impl From<types::Document> for PersistedDocument {
    fn from(value: types::Document) -> Self {
        let mut map: PersistedDocument = PersistedDocument::new();

        for field in value.fields {
            map.insert(field.name, from_wasi_to_json(&field.value));
        }

        map
    }
}

fn from_wasi_to_json(wasi_value: &Value) -> serde_json::Value {
    match wasi_value {
        Value::Text(text) => text.clone().into(),
        Value::Datetime(date) => date.clone().into(),
        Value::Number(number) => number.clone().into(),
    }
}

pub struct Index {
    pub name: String,
}

pub trait MichelPersistence: Send + Sync {
    fn add_document(&self, index: Index, document: PersistedDocument) -> Result<()>;
    fn search_document(
        &self,
        index: Index,
        query: String,
        limit: Option<u32>,
    ) -> Result<Vec<PersistedDocument>>;
}
