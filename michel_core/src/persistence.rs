use crate::plugins::wasi::types;
use anyhow::{anyhow, Result};

pub type PersistedDocument = serde_json::Map<String, serde_json::Value>;

impl From<types::Document> for PersistedDocument {
    fn from(value: types::Document) -> Self {
        let mut map: PersistedDocument = PersistedDocument::new();

        map.insert(
            "id".to_string(),
            serde_json::Value::String(String::from(&value.identifier)),
        );

        for field in value.fields {
            map.insert(field.name, serde_json::Value::from(field.value));
        }

        map
    }
}

impl TryFrom<&PersistedDocument> for types::Document {
    type Error = anyhow::Error;

    fn try_from(value: &PersistedDocument) -> Result<Self> {
        let identifier = value
            .get("id")
            .map(|value| value.as_str())
            .ok_or(anyhow!("Invalid identifier value"))?
            .ok_or(anyhow!("No identifier in document"))?;

        let fields: Vec<types::Field> = value
            .into_iter()
            .filter(|(field_name, _)| field_name.clone() != "id")
            .map(|(field_name, value)| (field_name, types::Value::try_from(value.clone())))
            .filter(|(_field_name, value)| value.is_ok())
            .map(|(field_name, value)| types::Field {
                name: field_name.clone(),
                value: value.unwrap(),
            })
            .collect();

        return Ok(types::Document {
            identifier: String::from(identifier),
            fields,
        });
    }
}

impl TryFrom<serde_json::Value> for types::Value {
    type Error = anyhow::Error;

    fn try_from(value: serde_json::Value) -> Result<Self> {
        match value {
            serde_json::Value::Bool(value) => Ok(types::Value::Boolean(value)),
            serde_json::Value::Number(number) => Ok(types::Value::Number(u32::try_from(
                number.as_u64().ok_or(anyhow!("value is not a number"))?,
            )?)),
            serde_json::Value::String(text) => Ok(types::Value::Text(text)),
            _ => Err(anyhow!("value type not handled")),
        }
    }
}

impl From<types::Value> for serde_json::Value {
    fn from(value: types::Value) -> Self {
        match value {
            types::Value::Text(text) => text.clone().into(),
            types::Value::Datetime(date) => date.clone().into(),
            types::Value::Number(number) => number.clone().into(),
            types::Value::Boolean(bool) => bool.into(),
        }
    }
}

pub struct Index {
    pub name: String,
}

pub trait MichelPersistence: Send + Sync {
    fn init_index(&mut self, name: String) -> Result<()>;
    fn add_document(&self, index: Index, document: PersistedDocument) -> Result<()>;
    fn add_documents(&self, index: Index, documents: Vec<PersistedDocument>) -> Result<()>;
    fn search_document(
        &self,
        index: Index,
        query: String,
        limit: Option<u32>,
    ) -> Result<Vec<PersistedDocument>>;
}
