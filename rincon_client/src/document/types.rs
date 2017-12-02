
use std::fmt::{self, Debug};
use std::marker::PhantomData;

use serde::de::{Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use rincon_core::arango::protocol::{FIELD_ENTITY_ID, FIELD_ENTITY_KEY,
    FIELD_ENTITY_REVISION, FIELD_ENTITY_NEW, FIELD_ENTITY_OLD,
    FIELD_ENTITY_OLD_REVISION, Handle, HandleOption};

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

impl From<DocumentId> for DocumentIdOption {
    fn from(document_id: DocumentId) -> Self {
        DocumentIdOption::Qualified(document_id)
    }
}

impl From<DocumentKey> for DocumentIdOption {
    fn from(document_key: DocumentKey) -> Self {
        DocumentIdOption::Local(document_key)
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

    pub fn deconstruct(self) -> (String, String) {
        (self.collection_name, self.document_key)
    }

    pub fn into_string(self) -> String {
        self.collection_name + "/" + &self.document_key
    }

    pub fn to_string(&self) -> String {
        self.collection_name.to_owned() + "/" + &self.document_key
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn document_key(&self) -> &str {
        &self.document_key
    }
}

impl Serialize for DocumentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
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

    pub fn deconstruct(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    pub fn deconstruct(self) -> String {
        self.0
    }

    pub fn from_str(value: &str) -> Self {
        Revision(value.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DocumentSelector {
    Id(DocumentId),
    Key(DocumentKey),
    Header(DocumentHeader),
}

impl From<DocumentId> for DocumentSelector {
    fn from(document_id: DocumentId) -> Self {
        DocumentSelector::Id(document_id)
    }
}

impl From<DocumentKey> for DocumentSelector {
    fn from(document_key: DocumentKey) -> Self {
        DocumentSelector::Key(document_key)
    }
}

impl From<DocumentHeader> for DocumentSelector {
    fn from(document_header: DocumentHeader) -> Self {
        DocumentSelector::Header(document_header)
    }
}

impl Serialize for DocumentSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::DocumentSelector::*;
        match *self {
            Id(ref document_id) => document_id.serialize(serializer),
            Key(ref document_key) => document_key.serialize(serializer),
            Header(ref header) => header.serialize(serializer),
        }
    }
}

/// All the possible keys, to avoid allocating memory if it is a key we
/// recognize. Private.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum DocumentField {
    Id,
    Key,
    Revision,
    OldRevision,
    New,
    Old,
    Other(String),
}

impl<'de> Deserialize<'de> for DocumentField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>,
    {
        use serde::de::Error;

        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = DocumentField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a document field name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: Error,
            {
                Ok(match value {
                    FIELD_ENTITY_ID => DocumentField::Id,
                    FIELD_ENTITY_KEY => DocumentField::Key,
                    FIELD_ENTITY_REVISION => DocumentField::Revision,
                    FIELD_ENTITY_OLD_REVISION => DocumentField::OldRevision,
                    FIELD_ENTITY_NEW => DocumentField::New,
                    FIELD_ENTITY_OLD => DocumentField::Old,
                    _ => DocumentField::Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentHeader {
    #[serde(rename = "_id")]
    id: DocumentId,
    #[serde(rename = "_key")]
    key: DocumentKey,
    #[serde(rename = "_rev")]
    revision: Revision,
}

impl DocumentHeader {
    pub fn new(id: DocumentId, key: DocumentKey, revision: Revision) -> Self {
        DocumentHeader {
            id,
            key,
            revision,
        }
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn deconstruct(self) -> (DocumentId, DocumentKey, Revision) {
        (self.id, self.key, self.revision)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UpdatedDocumentHeader {
    #[serde(rename = "_id")]
    id: DocumentId,
    #[serde(rename = "_key")]
    key: DocumentKey,
    #[serde(rename = "_rev")]
    revision: Revision,
    #[serde(rename = "_oldRev")]
    old_revision: Revision,
}

impl UpdatedDocumentHeader {
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn old_revision(&self) -> &Revision {
        &self.old_revision
    }

    pub fn deconstruct(self) -> (DocumentId, DocumentKey, Revision, Revision) {
        (self.id, self.key, self.revision, self.old_revision)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document<T> {
    id: DocumentId,
    key: DocumentKey,
    revision: Revision,
    content: T,
}

impl<T> Document<T> {
    pub fn new(
        id: DocumentId,
        key: DocumentKey,
        revision: Revision,
        content: T,
    ) -> Self {
        Document {
            id,
            key,
            revision,
            content,
        }
    }

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

    pub fn unwrap_header(self) -> DocumentHeader {
        DocumentHeader {
            id: self.id,
            key: self.key,
            revision: self.revision,
        }
    }

    pub fn unwrap_content(self) -> T {
        self.content
    }
}

impl<'de, T> Deserialize<'de> for Document<T>
    where T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use serde_json::{Map, Value, from_value};

        struct DocumentVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for DocumentVisitor<T>
            where T: DeserializeOwned,
        {
            type Value = Document<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("at least fields '_id', '_key' and '_rev'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut content: Option<Value> = None;
                let mut other = Map::new();

                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        DocumentField::Id => {
                            id = fields.next_value()?;
                        },
                        DocumentField::Key => {
                            key = fields.next_value()?;
                        },
                        DocumentField::Revision => {
                            revision = fields.next_value()?;
                        },
                        DocumentField::OldRevision => {
                            other.insert(FIELD_ENTITY_OLD_REVISION.to_owned(), fields.next_value()?);
                        },
                        DocumentField::New => {
                            content = fields.next_value()?;
                        },
                        DocumentField::Old => {
                            content = fields.next_value()?;
                        },
                        DocumentField::Other(name) => {
                            other.insert(name, fields.next_value()?);
                        },
                    }
                }

                match (id, key, revision, content) {
                    (Some(id), Some(key), Some(revision), Some(content)) => {
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        let content = from_value(content).map_err(A::Error::custom)?;
                        Ok(Document {
                            id,
                            key,
                            revision,
                            content,
                        })
                    },
                    (Some(id), Some(key), Some(revision), None) => {
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        let content = from_value(Value::Object(other)).map_err(A::Error::custom)?;
                        Ok(Document {
                            id,
                            key,
                            revision,
                            content,
                        })
                    },
                    (None, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, None, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_KEY)),
                    (_, _, None, _) => Err(A::Error::missing_field(FIELD_ENTITY_REVISION)),
                }
            }
        }

        deserializer.deserialize_map(DocumentVisitor(PhantomData))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewDocument<T> {
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

impl<T> Serialize for NewDocument<T>
    where T: Serialize + Debug
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::Error;
        use serde_json::{self, Value};

        if let Some(ref key) = self.key {
            let mut json_value = serde_json::to_value(&self.content).map_err(S::Error::custom)?;
            match json_value {
                Value::Object(ref mut fields) => {
                    fields.insert(FIELD_ENTITY_KEY.to_owned(), Value::String(key.as_str().to_owned()));
                },
                _ => return Err(S::Error::custom(format!("Invalid document content! Only types that serialize into valid Json objects are supported. But got: {:?}", &self.content))),
            }
            let json_value_with_key = json_value;
            json_value_with_key.serialize(serializer)
        } else {
            self.content.serialize(serializer)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpdatedDocument<Old, New> {
    id: DocumentId,
    key: DocumentKey,
    revision: Revision,
    old_revision: Revision,
    old_content: Option<Old>,
    new_content: Option<New>,
}

impl<Old, New> UpdatedDocument<Old, New> {
    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> &Revision {
        &self.revision
    }

    pub fn old_revision(&self) -> &Revision {
        &self.old_revision
    }

    pub fn old_content(&self) -> Option<&Old> {
        self.old_content.as_ref()
    }

    pub fn new_content(&self) -> Option<&New> {
        self.new_content.as_ref()
    }
}

impl<'de, Old, New> Deserialize<'de> for UpdatedDocument<Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use serde_json::{Map, Value, from_value};

        struct DocumentVisitor<Old, New>(PhantomData<Old>, PhantomData<New>);

        impl<'de, Old, New> Visitor<'de> for DocumentVisitor<Old, New>
            where Old: DeserializeOwned, New: DeserializeOwned
        {
            type Value = UpdatedDocument<Old, New>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("at least fields '_id', '_key' and '_rev'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut old_revision: Option<String> = None;
                let mut new_content: Option<Value> = None;
                let mut old_content: Option<Value> = None;
                let mut other = Map::new();

                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        DocumentField::Id => {
                            id = fields.next_value()?;
                        },
                        DocumentField::Key => {
                            key = fields.next_value()?;
                        },
                        DocumentField::Revision => {
                            revision = fields.next_value()?;
                        },
                        DocumentField::OldRevision => {
                            old_revision = fields.next_value()?;
                        },
                        DocumentField::New => {
                            new_content = fields.next_value()?;
                        },
                        DocumentField::Old => {
                            old_content = fields.next_value()?;
                        },
                        DocumentField::Other(name) => {
                            other.insert(name, fields.next_value()?);
                        },
                    }
                }

                match (id, key, revision, old_revision) {
                    (Some(id), Some(key), Some(revision), Some(old_revision)) => {
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        let old_revision = Revision::from_string(old_revision);
                        let old_content = if let Some(old_content) = old_content {
                            Some(from_value(old_content).map_err(A::Error::custom)?)
                        } else {
                            None
                        };
                        let new_content = if let Some(new_content) = new_content {
                            Some(from_value(new_content).map_err(A::Error::custom)?)
                        } else {
                            None
                        };
                        Ok(UpdatedDocument {
                            id,
                            key,
                            revision,
                            old_revision,
                            old_content,
                            new_content,
                        })
                    },
                    (None, _, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, None, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_KEY)),
                    (_, _, None, _) => Err(A::Error::missing_field(FIELD_ENTITY_REVISION)),
                    (_, _, _, None) => Err(A::Error::missing_field(FIELD_ENTITY_OLD_REVISION)),
                }
            }
        }

        deserializer.deserialize_map(DocumentVisitor(PhantomData, PhantomData))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentUpdate<Upd> {
    key: DocumentKey,
    revision: Option<Revision>,
    content: Upd,
}

impl<Upd> DocumentUpdate<Upd> {
    pub fn new(key: DocumentKey, content: Upd) -> Self {
        DocumentUpdate {
            key,
            revision: None,
            content,
        }
    }

    pub fn with_revision<Rev>(mut self, revision: Rev) -> Self
        where Rev: Into<Option<Revision>>
    {
        self.revision = revision.into();
        self
    }

    pub fn key(&self) -> &DocumentKey {
        &self.key
    }

    pub fn revision(&self) -> Option<&Revision> {
        self.revision.as_ref()
    }

    pub fn content(&self) -> &Upd {
        &self.content
    }
}

impl<Upd> Serialize for DocumentUpdate<Upd>
    where Upd: Serialize + Debug
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::Error;
        use serde_json::{self, Value};
        let mut json_value = serde_json::to_value(&self.content).map_err(S::Error::custom)?;
        match json_value {
            Value::Object(ref mut fields) => {
                fields.insert(FIELD_ENTITY_KEY.to_owned(), Value::String(self.key.as_str().to_owned()));
                if let Some(ref revision) = self.revision {
                    fields.insert(FIELD_ENTITY_REVISION.to_owned(), Value::String(revision.as_str().to_owned()));
                }
            },
            _ => return Err(S::Error::custom(format!("Invalid document content! Only types that serialize into valid Json objects are supported. But got: {:?}", &self.content))),
        };
        let json_value_with_header_fields = json_value;
        json_value_with_header_fields.serialize(serializer)
    }
}
