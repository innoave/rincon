
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use arango::protocol::{Handle, HandleOption};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DocumentIdOption {
    Qualified(DocumentId),
    Local(DocumentKey),
}

impl DocumentIdOption {
    pub fn from_str(value: &str) -> Result<Self, String> {
        let handle_option = HandleOption::from_str("document id", value)?;
        Ok(match handle_option {
            HandleOption::Qualified(handle) => {
                let (collection_name, document_key) = handle.deconstruct();
                DocumentIdOption::Qualified(DocumentId {
                    collection_name,
                    document_key,
                })
            },
            HandleOption::Local(handle_key) => {
                let value = handle_key.deconstruct();
                DocumentIdOption::Local(DocumentKey(value))
            },
        })
    }
}

impl Serialize for DocumentIdOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::DocumentIdOption::*;
        match *self {
            Qualified(ref document_id) => document_id.serialize(serializer),
            Local(ref document_key) => document_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DocumentIdOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        DocumentIdOption::from_str(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentId {
    collection_name: String,
    document_key: String,
}

impl DocumentId {
    pub fn new<C, K>(collection_name: C, document_key: K) -> Self
        where C: Into<String>, K: Into<String>
    {
        let collection_name = collection_name.into();
        assert!(!collection_name.contains('/'), "A collection name must not contain any '/' character");
        let document_key = document_key.into();
        assert!(!document_key.contains('/'), "A document key must not contain any '/' character");
        DocumentId {
            collection_name,
            document_key,
        }
    }

    pub fn from_str(value: &str) -> Result<Self, String> {
        let handle = Handle::from_str("document id", value)?;
        let (collection_name, document_key) = handle.deconstruct();
        Ok(DocumentId {
            collection_name,
            document_key,
        })
    }

    pub fn as_string(&self) -> String {
        self.collection_name.to_owned() + "/" + &self.document_key
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn document_key(&self) -> &str {
        &self.document_key
    }
}

impl From<DocumentId> for DocumentIdOption {
    fn from(document_id: DocumentId) -> Self {
        DocumentIdOption::Qualified(document_id)
    }
}

impl Serialize for DocumentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.as_string())
    }
}

impl<'de> Deserialize<'de> for DocumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        DocumentId::from_str(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DocumentKey(String);

impl DocumentKey {
    pub fn new<K>(document_key: K) -> Self
        where K: Into<String>
    {
        let document_key = document_key.into();
        assert!(!document_key.contains('/'), "A document key must not contain any '/' character, but got: {:?}", &document_key);
        DocumentKey(document_key)
    }

    pub fn from_string(value: String) -> Result<Self, String> {
        if value.contains('/') {
            Err(format!("A document key must not contain any '/' character, but got {:?}", &value))
        } else {
            Ok(DocumentKey(value))
        }
    }

    pub fn from_str(value: &str) -> Result<Self, String> {
        DocumentKey::from_string(value.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<DocumentKey> for DocumentIdOption {
    fn from(document_key: DocumentKey) -> Self {
        DocumentIdOption::Local(document_key)
    }
}

impl Serialize for DocumentKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for DocumentKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        DocumentKey::from_string(value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Revision(String);

impl Revision {
    pub fn new<R>(value: R) -> Self
        where R: Into<String>
    {
        Revision(value.into())
    }

    pub fn from_string(value: String) -> Self {
        Revision(value)
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn from_str(value: &str) -> Self {
        Revision(value.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document<T> {
    #[serde(rename = "_id")]
    id: DocumentId,
    #[serde(rename = "_key")]
    key: DocumentKey,
    #[serde(rename = "_rev")]
    revision: Revision,
    content: T,
}

impl<T> Document<T> {
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn unwrap_content(self) -> T {
        self.content
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDocument<T> {
    #[serde(rename = "_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<DocumentKey>,
    content: T,
}

impl<T> NewDocument<T> {
    pub fn from_content(content: T) -> Self {
        NewDocument {
            key: None,
            content,
        }
    }

    pub fn with_key<K>(mut self, key: K) -> Self
        where K: Into<Option<DocumentKey>>
    {
        self.key = key.into();
        self
    }

    pub fn key(&self) -> Option<&DocumentKey> {
        self.key.as_ref()
    }

    pub fn content(&self) -> &T {
        &self.content
    }
}

impl<T> From<T> for NewDocument<T> {
    fn from(content: T) -> Self {
        NewDocument::from_content(content)
    }
}
