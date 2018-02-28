
#[cfg(test)]
mod tests;

use std::collections::HashMap;

use api::types::{Value, UnwrapValue};

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    string: String,
    params: HashMap<String, Value>,
}

impl Query {
    /// Constructs a new `Query` with the given query string.
    pub fn new<Q>(query_string: Q) -> Self
        where Q: Into<String>
    {
        Query {
            string: query_string.into(),
            params: HashMap::new(),
        }
    }

    /// Moves the fields out of the `Query` struct to reuse their values
    /// without cloning them.
    ///
    /// After calling this function this `Query` instance is invalid.
    pub fn deconstruct(self) -> (String, HashMap<String, Value>) {
        (self.string, self.params)
    }

    /// Returns the query string as a `&str`.
    pub fn str(&self) -> &str {
        &self.string
    }

    /// Sets the value of a named parameter.
    pub fn set_parameter<N, T>(&mut self, name: N, value: T)
        where N: Into<String>, T: Into<Value>
    {
        self.params.insert(name.into(), value.into());
    }

    /// Returns the value of a named parameter.
    pub fn parameter<T>(&self, name: &str) -> Option<&T>
        where T: UnwrapValue
    {
        self.params.get(name).map(UnwrapValue::unwrap)
    }
}
