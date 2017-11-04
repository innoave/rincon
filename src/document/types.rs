
use std::fmt::{self, Debug};
use std::marker::PhantomData;

use serde::de::{Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use arango::protocol::{FIELD_ENTITY_ID, FIELD_ENTITY_KEY, FIELD_ENTITY_REVISION,
    Handle, HandleOption};

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

/// Marker trait for content of documents. Types that implement this trait
/// can be used as content in documents.
///
/// Documents in *ArangoDB* must be valid Json documents. As simple types such
/// as primitive types do not serialize into Json objects they must be wrapped
/// within some struct enum or map.
///
/// This trait is useful if one wants to check at compile time whether the
/// content of a document is supported by *ArangoDB*.
pub trait Content {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawJson(String);

impl Content for RawJson {}

impl RawJson {
    pub fn new<J>(value: J) -> Self
        where J: Into<String>
    {
        RawJson(value.into())
    }

    pub fn from_string(value: String) -> Self {
        RawJson(value)
    }

    pub fn from_str(value: &str) -> Self {
        RawJson(value.to_owned())
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

/// All the possible keys, to avoid allocating memory if it is a key we
/// recognize. Private.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum DocumentField {
    Id,
    Key,
    Revision,
    Other(String),
}

impl<'de> Deserialize<'de> for DocumentField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;

        struct FieldVisitor;

        impl<'v> Visitor<'v> for FieldVisitor {
            type Value = DocumentField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string representing a field name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: Error,
            {
                Ok(match value {
                    FIELD_ENTITY_ID => DocumentField::Id,
                    FIELD_ENTITY_KEY => DocumentField::Key,
                    FIELD_ENTITY_REVISION => DocumentField::Revision,
                    _ => DocumentField::Other(value.to_owned()),
                })
            }
        }

        deserializer.deserialize_str(FieldVisitor)
    }
}

impl<'de, T> Deserialize<'de> for Document<T>
    where T: DeserializeOwned
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use serde_json::{Map, Value, from_value};

        struct DocumentVisitor<T> {
            content: PhantomData<T>,
        }

        impl<'v, T> Visitor<'v> for DocumentVisitor<T>
            where T: DeserializeOwned
        {
            type Value = Document<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a valid Json document with at least the fields 'id', 'key' and 'revision'")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'v>,
            {
                let mut id: Option<String> = None;
                let mut key: Option<String> = None;
                let mut revision: Option<String> = None;
                let mut content = Map::new();
                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        DocumentField::Id => {
                            id = Some(fields.next_value()?);
                        },
                        DocumentField::Key => {
                            key = Some(fields.next_value()?);
                        },
                        DocumentField::Revision => {
                            revision = Some(fields.next_value()?);
                        },
                        DocumentField::Other(name) => {
                            content.insert(name, fields.next_value()?);
                        }
                    }
                }
                let content = from_value(Value::Object(content)).map_err(A::Error::custom)?;
                match (id, key, revision) {
                    (Some(id), Some(key), Some(revision)) => {
                        let id = DocumentId::from_str(&id).map_err(A::Error::custom)?;
                        let key = DocumentKey::from_string(key).map_err(A::Error::custom)?;
                        let revision = Revision::from_string(revision);
                        Ok(Document {
                            id,
                            key,
                            revision,
                            content,
                        })
                    },
                    (None, _, _) => Err(A::Error::missing_field(FIELD_ENTITY_ID)),
                    (_, None, _) => Err(A::Error::missing_field(FIELD_ENTITY_KEY)),
                    (_, _, None) => Err(A::Error::missing_field(FIELD_ENTITY_REVISION)),
                }
            }
        }

        deserializer.deserialize_map(DocumentVisitor { content: PhantomData })
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
            };
            let json_value_with_key = json_value;
            json_value_with_key.serialize(serializer)
        } else {
            self.content.serialize(serializer)
        }
    }
}

impl Serialize for NewDocument<RawJson> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::Error;
        use serde_json::{self, Value};
        let mut json_value = serde_json::from_str(&self.content.0).map_err(S::Error::custom)?;
        if let Some(ref key) = self.key {
            match json_value {
                Value::Object(ref mut fields) => {
                    fields.insert(FIELD_ENTITY_KEY.to_owned(), Value::String(key.as_str().to_owned()));
                },
                _ => return Err(S::Error::custom(format!("Invalid document content! Only strings that contain a valid Json object are supported. But got: {:?}", &self.content))),
            };
        }
        json_value.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json;
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct MyContent {
        a: String,
        b: i32,
    }

    #[test]
    fn serialize_struct_document_without_key() {
        let content = MyContent {
            a: "Hugo".to_owned(),
            b: 42,
        };

        let new_document = NewDocument::from_content(content);
        let json = serde_json::to_string(&new_document).unwrap();

        assert_eq!(r#"{"a":"Hugo","b":42}"#, &json);
    }

    #[test]
    fn serialize_struct_document_with_key() {
        let content = MyContent {
            a: "Hugo".to_owned(),
            b: 42,
        };

        let new_document = NewDocument::from_content(content)
            .with_key(DocumentKey::new("29384"));
        let json = serde_json::to_string(&new_document).unwrap();

        assert_eq!(r#"{"_key":"29384","a":"Hugo","b":42}"#, &json);
    }

    #[test]
    fn serialize_hashmap_document_without_key() {
        let mut content = HashMap::new();
        content.insert("a", json!("Hugo"));
        content.insert("b", json!(42));
        let content = content;

        let new_document = NewDocument::from_content(content);
        let json = serde_json::to_string(&new_document).unwrap();

        assert!(r#"{"a":"Hugo","b":42}"# == &json
            || r#"{"b":42,"a":"Hugo"}"# == &json
        );
    }

    #[test]
    fn serialize_hashmap_document_with_key() {
        let mut content = HashMap::new();
        content.insert("a", json!("Hugo"));
        content.insert("b", json!(42));
        let content = content;

        let new_document = NewDocument::from_content(content)
            .with_key(DocumentKey::new("29384"));
        let json = serde_json::to_string(&new_document).unwrap();

        assert!(r#"{"_key":"29384","a":"Hugo","b":42}"# == &json
            || r#"{"_key":"29384","b":42,"a":"Hugo"}"# == &json
        );
    }

    #[test]
    fn serialize_raw_json_document_without_key() {
        let content = RawJson::from_str(r#"{"a":"Hugo","b":42}"#);

        let new_document = NewDocument::from_content(content);
        let json = serde_json::to_string(&new_document).unwrap();

        assert_eq!(r#"{"a":"Hugo","b":42}"#, &json);
    }

    #[test]
    fn serialize_raw_json_document_with_key() {
        let content = RawJson::from_str(r#"{"a":"Hugo","b":42}"#);

        let new_document = NewDocument::from_content(content)
            .with_key(DocumentKey::new("29384"));
        let json = serde_json::to_string(&new_document).unwrap();

        assert_eq!(r#"{"_key":"29384","a":"Hugo","b":42}"#, &json);
    }

    #[test]
    fn serialize_primitive_content_document_without_key() {
        let content = 42;

        let new_document = NewDocument::from_content(content);
        let json = serde_json::to_string(&new_document).unwrap();

        assert_eq!("42", &json);
    }

    #[test]
    fn serialize_primitive_content_document_with_key() {
        let content = 42;

        let new_document = NewDocument::from_content(content)
            .with_key(DocumentKey::new("29384"));
        let result = serde_json::to_string(&new_document);

        if let Err(error) = result {
            assert_eq!("ErrorImpl { code: Message(\"Invalid document content! Only types that serialize into valid Json objects are supported. But got: 42\"), line: 0, column: 0 }", format!("{:?}", error));
        } else {
            panic!("Error expected, but got: {:?}", result);
        }
    }

    #[test]
    fn deserialize_document() {
        let json_string = r#"{"_id":"customers/29384","_key":"29384","_rev":"aOIey283aew","a":"Hugo","b":42}"#;

        let document = serde_json::from_str(json_string).unwrap();

        let expected = Document {
            id: DocumentId::new("customers", "29384"),
            key: DocumentKey::new("29384"),
            revision: Revision::new("aOIey283aew"),
            content: MyContent {
                a: "Hugo".to_owned(),
                b: 42,
            }
        };
        assert_eq!(expected, document);
    }
}
