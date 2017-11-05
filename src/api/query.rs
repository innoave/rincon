
use std::collections::HashMap;
use std::mem;

use api::types::{Value, UnwrapValue};

#[derive(Clone, Debug, PartialEq)]
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
        let mut query = self;
        (
            mem::replace(&mut query.string, String::with_capacity(0)),
            mem::replace(&mut query.params, HashMap::with_capacity(0)),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_set_string_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name", "simone");

        assert_eq!(Some(&Value::String("simone".to_owned())), query.params.get("name"));
    }

    #[test]
    fn query_set_bool_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.active = @active RETURN u.name");
        query.set_parameter("active", true);

        assert_eq!(Some(&Value::Bool(true)), query.params.get("active"));
    }

    #[test]
    fn query_set_i64_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id = @id RETURN u.name");
        query.set_parameter("id", -1828359i64);

        assert_eq!(Some(&Value::I64(-1828359)), query.params.get("id"));
    }

    #[test]
    fn query_set_vec_of_u64_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id in @ids RETURN u.name");
        let ids: Vec<u64> = vec![1, 2, 3, 4, 5];
        query.set_parameter("ids", ids);

        assert_eq!(Some(&Value::VecU64(vec![1, 2, 3, 4, 5])), query.params.get("ids"));
    }

    #[test]
    fn query_get_string_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.params.insert("name".to_owned(), Value::String("appolonia".to_owned()));

        assert_eq!(Some(&"appolonia".to_owned()), query.parameter("name"));

    }

    #[test]
    fn query_get_bool_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.active = @active RETURN u.name");
        query.params.insert("active".to_owned(), Value::Bool(false));

        assert_eq!(Some(&false), query.parameter("active"));
    }

    #[test]
    fn query_get_i8_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id = @id RETURN u.name");
        query.set_parameter("id".to_owned(), Value::I8(-1));

        assert_eq!(Some(&-1i8), query.parameter("id"));
    }

    #[test]
    fn query_get_vec_of_f32_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id in @ids RETURN u.name");
        let ids = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        query.set_parameter("ids", Value::VecF32(ids));

        assert_eq!(Some(&vec![1.1f32, 2.2, 3.3, 4.4, 5.5]), query.parameter("ids"));
    }
}
