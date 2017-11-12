
use std::mem;

use regex::Regex;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

pub const FIELD_CODE: &str = "code";
pub const FIELD_ENTITY_ID: &str = "_id";
pub const FIELD_ENTITY_KEY: &str = "_key";
pub const FIELD_ENTITY_REVISION: &str = "_rev";
pub const FIELD_ENTITY_NEW: &str = "new";
pub const FIELD_ENTITY_OLD: &str = "old";
pub const FIELD_ENTITY_OLD_REVISION: &str = "_oldRev";
pub const FIELD_ID: &str = "id";
pub const FIELD_RESULT: &str = "result";

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
        let mut handle = self;
        (
            mem::replace(&mut handle.context, String::new()),
            mem::replace(&mut handle.key, String::new()),
        )
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
        let mut handle_key = self;
        mem::replace(&mut handle_key.0, String::new())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_handle_key_from_str() {
        let handle_key = HandleKey::from_string("index id", "12341".to_owned()).unwrap();
        assert_eq!("12341", &handle_key.0);
    }

    #[test]
    fn get_handle_key_from_str_with_slash_character_in_the_middle() {
        let result = HandleKey::from_string("index id", "mine/12341".to_owned());
        assert_eq!(Err("A index id key must not contain any '/' character, but got: \"mine/12341\"".to_owned()), result);
    }

    #[test]
    fn get_handle_key_from_str_with_slash_character_at_the_beginning() {
        let result = HandleKey::from_string("index id", "/12341".to_owned());
        assert_eq!(Err("A index id key must not contain any '/' character, but got: \"/12341\"".to_owned()), result);
    }

    #[test]
    fn get_handle_key_from_str_with_slash_character_at_the_end() {
        let result = HandleKey::from_string("index id", "12341/".to_owned());
        assert_eq!(Err("A index id key must not contain any '/' character, but got: \"12341/\"".to_owned()), result);
    }

    #[test]
    fn get_handle_from_str() {
        let handle = Handle::from_str("index id", "mine/12341").unwrap();
        assert_eq!("mine", &handle.context);
        assert_eq!("12341", &handle.key);
        assert_eq!("mine/12341", &handle.to_string());
    }

    #[test]
    fn get_handle_from_str_without_context() {
        let result = Handle::from_str("index id", "12341");
        assert_eq!(Err("index id does not have a context: \"12341\"".to_owned()), result);
    }

    #[test]
    fn get_handle_from_str_with_empty_context() {
        let result = Handle::from_str("index id", "/12341");
        assert_eq!(Err("Invalid index id: \"/12341\"".to_owned()), result);
    }

    #[test]
    fn get_handle_from_str_with_empty_key() {
        let result = Handle::from_str("index id", "mine/");
        assert_eq!(Err("Invalid index id: \"mine/\"".to_owned()), result);
    }

    #[test]
    fn get_handle_option_from_str_with_context_and_key() {
        let handle_option = HandleOption::from_str("index id", "mine/12341").unwrap();
        assert_eq!(HandleOption::Qualified(Handle { context: "mine".to_owned(), key: "12341".to_owned() }), handle_option);
    }

    #[test]
    fn get_handle_option_from_str_with_key_only() {
        let handle_option = HandleOption::from_str("index id", "12341").unwrap();
        assert_eq!(HandleOption::Local(HandleKey("12341".to_owned())), handle_option);
    }
}
