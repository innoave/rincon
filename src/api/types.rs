
use serde_json;

pub type JsonValue = serde_json::Value;

pub const EMPTY: Empty = Empty {};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}
