//! Representation of database queries in the rincon driver.
//!
//! This module defines a driver specific `Query` struct that holds the
//! necessary data to execute a query.

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::fmt;

use api::types::{UnwrapValue, Value};

/// Represents a database query within the rincon driver.
///
/// This struct holds the AQL query string and the query parameters needed to
/// call the ArangoDB server to execute the database query.
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    string: String,
    params: HashMap<String, Value>,
}

impl Query {
    /// Constructs a new `Query` with the given query string.
    pub fn new<Q>(query_string: Q) -> Self
    where
        Q: Into<String>,
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
    pub fn unwrap(self) -> (String, HashMap<String, Value>) {
        (self.string, self.params)
    }

    /// Returns the query string as a `&str`.
    pub fn str(&self) -> &str {
        &self.string
    }

    /// Sets the value of a named parameter.
    pub fn set_parameter<N, T>(&mut self, name: N, value: T)
    where
        N: Into<String>,
        T: Into<Value>,
    {
        self.params.insert(name.into(), value.into());
    }

    /// Returns the value of a named parameter.
    pub fn parameter<T>(&self, name: &str) -> Option<&T>
    where
        T: UnwrapValue,
    {
        self.params.get(name).map(UnwrapValue::unwrap)
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::fmt::Write;
        f.write_str(&self.string)?;
        if !self.params.is_empty() {
            f.write_str("\n with: ")?;
            let mut first = true;
            for (name, value) in &self.params {
                if first {
                    first = false;
                } else {
                    f.write_str(", ")?;
                }
                f.write_str(name)?;
                f.write_char('=')?;
                f.write_str(&value.to_string())?;
            }
        }
        Ok(())
    }
}
