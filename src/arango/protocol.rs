
use std::fmt;
use std::marker::PhantomData;

use regex::Regex;
use serde::de::{Deserialize, DeserializeOwned, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use api::{self, method};

pub const FIELD_CODE: &str = "code";
pub const FIELD_EDGE_DEFINITIONS: &str = "edgeDefinitions";
pub const FIELD_ENTITY_ID: &str = "_id";
pub const FIELD_ENTITY_KEY: &str = "_key";
pub const FIELD_ENTITY_REVISION: &str = "_rev";
pub const FIELD_ENTITY_NEW: &str = "new";
pub const FIELD_ENTITY_OLD: &str = "old";
pub const FIELD_ENTITY_OLD_REVISION: &str = "_oldRev";
pub const FIELD_ERROR: &str = "error";
pub const FIELD_ERROR_MESSAGE: &str = "errorMessage";
pub const FIELD_ERROR_NUMBER: &str = "errorNum";
pub const FIELD_GRAPH: &str = "graph";
pub const FIELD_GRAPHS: &str = "graphs";
pub const FIELD_ID: &str = "id";
pub const FIELD_IS_SMART: &str = "isSmart";
pub const FIELD_NAME: &str = "name";
pub const FIELD_NUMBER_OF_SHARDS: &str = "numberOfShards";
pub const FIELD_ORPHAN_COLLECTIONS: &str = "orphanCollections";
pub const FIELD_REMOVED: &str = "removed";
pub const FIELD_REPLICATION_FACTOR: &str = "replicationFactor";
pub const FIELD_RESULT: &str = "result";
pub const FIELD_SMART_GRAPH_ATTRIBUTE: &str = "smartGraphAttribute";

pub const HEADER_IF_MATCH: &str = "If-Match";
pub const HEADER_IF_NON_MATCH: &str = "If-None-Match";

pub const PARAM_COLLECTION: &str = "collection";
pub const PARAM_DETAILS: &str = "details";
pub const PARAM_EXCLUDE_SYSTEM: &str = "excludeSystem";
pub const PARAM_IGNORE_REVISIONS: &str = "ignoreRevs";
pub const PARAM_KEEP_NULL: &str = "keepNull";
pub const PARAM_MERGE_OBJECTS: &str = "mergeObjects";
pub const PARAM_RETURN_NEW: &str = "returnNew";
pub const PARAM_RETURN_OLD: &str = "returnOld";
pub const PARAM_WAIT_FOR_SYNC: &str = "waitForSync";
#[cfg(feature = "cluster")]
pub const PARAM_WAIT_FOR_SYNC_REPLICATION: &str = "waitForSyncReplication";

pub const PATH_ADMIN: &str = "/_admin";
pub const PATH_API_COLLECTION: &str = "/_api/collection";
pub const PATH_API_CURSOR: &str = "/_api/cursor";
pub const PATH_API_DATABASE: &str = "/_api/database";
pub const PATH_API_DOCUMENT: &str = "/_api/document";
pub const PATH_API_EXPLAIN: &str = "/_api/explain";
pub const PATH_API_GHARIAL: &str = "/_api/gharial";
pub const PATH_API_INDEX: &str = "/_api/index";
pub const PATH_API_QUERY: &str = "/_api/query";
pub const PATH_API_USER: &str = "/_api/user";
pub const PATH_API_VERSION: &str = "/_api/version";
pub const PATH_OPEN_AUTH: &str = "/_open/auth";

pub const PATH_CURRENT: &str = "current";
pub const PATH_DATABASE: &str = "database";
pub const PATH_DB: &str = "/_db/";
pub const PATH_PROPERTIES: &str = "properties";
pub const PATH_RENAME: &str = "rename";
pub const PATH_TARGET_VERSION: &str = "target-version";
pub const PATH_USER: &str = "user";


const CAPTURE_CONTEXT_NAME: &str = "ctx";
const CAPTURE_ELEMENT_KEY: &str = "key";
const REGEX_ID_CAPTURE: &str = "^((?P<ctx>[^/]+)/)?(?P<key>[^/]+)$";


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HandleOption {
    Qualified(Handle),
    Local(HandleKey),
}

impl HandleOption {
    pub fn from_str(handle_name: &str, value: &str) -> Result<HandleOption, String> {
        let re = Regex::new(REGEX_ID_CAPTURE).unwrap();
        if let Some(caps) = re.captures(value) {
            match (caps.name(CAPTURE_CONTEXT_NAME), caps.name(CAPTURE_ELEMENT_KEY)) {
                (Some(context_name), Some(element_key)) =>
                    Ok(HandleOption::Qualified(Handle {
                        context: context_name.as_str().to_string(),
                        key: element_key.as_str().to_string(),
                    })),
                (None, Some(element_key)) =>
                    Ok(HandleOption::Local(HandleKey(
                        element_key.as_str().to_string()
                    ))),
                (_, None) =>
                    Err(format!("{} does not have a key: {:?}", handle_name, value)),
            }
        } else {
            Err(format!("Invalid {}: {:?}", handle_name, value))
        }
    }
}

impl Serialize for HandleOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            HandleOption::Qualified(ref handle) => handle.serialize(serializer),
            HandleOption::Local(ref handle_key) => handle_key.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for HandleOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        HandleOption::from_str("handle", &value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Handle {
    context: String,
    key: String,
}

impl Handle {
    pub fn from_str(handle_name: &str, value: &str) -> Result<Self, String> {
        let re = Regex::new(REGEX_ID_CAPTURE).unwrap();
        if let Some(caps) = re.captures(value) {
            match (caps.name(CAPTURE_CONTEXT_NAME), caps.name(CAPTURE_ELEMENT_KEY)) {
                (Some(context_name), Some(element_key)) =>
                    Ok(Handle {
                        context: context_name.as_str().to_string(),
                        key: element_key.as_str().to_string(),
                    }),
                (None, _) =>
                    Err(format!("{} does not have a context: {:?}", handle_name, value)),
                (_, None) =>
                    Err(format!("{} does not have a key: {:?}", handle_name, value)),
            }
        } else {
            Err(format!("Invalid {}: {:?}", handle_name, value))
        }
    }

    pub fn deconstruct(self) -> (String, String) {
        (self.context, self.key)
    }

    pub fn to_string(&self) -> String {
        self.context.to_owned() + "/" + &self.key
    }
}

impl From<Handle> for HandleOption {
    fn from(handle: Handle) -> Self {
        HandleOption::Qualified(handle)
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        Handle::from_str("handle", &value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HandleKey(String);

impl HandleKey {
    pub fn deconstruct(self) -> String {
        self.0
    }

    pub fn from_string(handle_name: &str, value: String) -> Result<Self, String> {
        if value.contains('/') {
            Err(format!("A {} key must not contain any '/' character, but got: {:?}",
                handle_name, value))
        } else {
            Ok(HandleKey(value))
        }
    }
}

impl From<HandleKey> for HandleOption {
    fn from(handle_key: HandleKey) -> Self {
        HandleOption::Local(handle_key)
    }
}

impl Serialize for HandleKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for HandleKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        let value = String::deserialize(deserializer)?;
        HandleKey::from_string("handle", value).map_err(D::Error::custom)
    }
}

impl<'de, T> Deserialize<'de> for method::Result<T>
    where T: DeserializeOwned
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use serde_json::{Map, Value, from_value};

        #[derive(Debug)]
        enum Field {
            Code,
            Error,
            ErrorNumber,
            ErrorMessage,
            Other(String),
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a valid object or an error object with fields 'code', 'errorNum' and 'errorMessage'")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where E: Error
                    {
                        Ok(match value {
                            FIELD_CODE => Field::Code,
                            FIELD_ERROR => Field::Error,
                            FIELD_ERROR_NUMBER => Field::ErrorNumber,
                            FIELD_ERROR_MESSAGE => Field::ErrorMessage,
                            _ => Field::Other(value.to_owned()),
                        })
                    }
                }

                deserializer.deserialize_str(FieldVisitor)
            }
        }

        struct ResultVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for ResultVisitor<T>
            where T: DeserializeOwned
        {
            type Value = method::Result<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum method::Result")
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                where A: MapAccess<'de>,
            {
                let mut error_code: Option<api::ErrorCode> = None;
                let mut message: Option<String> = None;
                let mut other = Map::new();
                let mut fields = map;
                while let Some(name) = fields.next_key()? {
                    match name {
                        Field::Code => {
                            let _: i32 = fields.next_value()?;
                        },
                        Field::Error => {
                            let _: bool = fields.next_value()?;
                        },
                        Field::ErrorNumber => {
                            error_code = fields.next_value()?;
                        },
                        Field::ErrorMessage => {
                            message = fields.next_value()?;
                        },
                        Field::Other(name) => {
                            other.insert(name, fields.next_value()?);
                        },
                    }
                }
                match (error_code, message) {
                    (Some(error_code), Some(message)) => {
                        let error = method::Error::new(error_code, message);
                        Ok(method::Result::Failed(error))
                    },
                    (Some(_), None) => Err(A::Error::missing_field(FIELD_ERROR_MESSAGE)),
                    (None, _) => {
                        let result_value = from_value(Value::Object(other)).map_err(A::Error::custom)?;
                        Ok(method::Result::Success(result_value))
                    },
                }
            }
        }

        deserializer.deserialize_map(ResultVisitor(PhantomData))
    }
}
