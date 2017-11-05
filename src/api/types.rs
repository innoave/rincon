
use serde_json;

pub const EMPTY: Empty = Empty {};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}

pub type JsonValue = serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct JsonString(String);

impl JsonString {
    pub fn new<J>(value: J) -> Self
        where J: Into<String>
    {
        JsonString(value.into())
    }

    pub fn from_string(value: String) -> Self {
        JsonString(value)
    }

    pub fn from_str(value: &str) -> Self {
        JsonString(value.to_owned())
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
