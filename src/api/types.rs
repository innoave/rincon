
use std::iter::FromIterator;

use serde_json;

pub type JsonValue = serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    bytes: Vec<u8>,
}

impl Document {
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<Vec<u8>> for Document {
    fn from(bytes: Vec<u8>) -> Self {
        Document {
            bytes
        }
    }
}

impl FromIterator<u8> for Document {
    fn from_iter<T: IntoIterator<Item=u8>>(iter: T) -> Self {
        Document {
            bytes: Vec::from_iter(iter),
        }
    }
}

pub const EMPTY: Empty = Empty {};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}
