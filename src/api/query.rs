
use std::collections::HashMap;
use std::mem;

use serde::ser::{Serialize, Serializer, SerializeSeq};

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
    pub fn set_parameter<T>(&mut self, name: String, value: T)
    where T: Into<Value>
    {
        self.params.insert(name, value.into());
    }

    /// Returns the value of a named parameter.
    pub fn parameter<T>(&self, name: &str) -> Option<&T>
    where T: UnwrapValue
    {
        self.params.get(name).map(UnwrapValue::unwrap)
    }
}

/// Defines the type of a query parameter.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    F64(f64),
    F32(f32),
    ISize(isize),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    USize(usize),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    VecString(Vec<String>),
    VecBool(Vec<bool>),
    VecF64(Vec<f64>),
    VecF32(Vec<f32>),
    VecISize(Vec<isize>),
    VecI64(Vec<i64>),
    VecI32(Vec<i32>),
    VecI16(Vec<i16>),
    VecI8(Vec<i8>),
    VecUSize(Vec<usize>),
    VecU64(Vec<u64>),
    VecU32(Vec<u32>),
    VecU16(Vec<u16>),
    VecU8(Vec<u8>),
}

impl Value {
    pub fn unwrap<T>(&self) -> &T
        where T: UnwrapValue
    {
        UnwrapValue::unwrap(self)
    }
}

/// Defines how to unwrap the value out of the `Value` enum.
///
/// This trait should be implemented for all types that can be wrapped inside
/// the `Value` enum.
pub trait UnwrapValue {
    fn unwrap(value: &Value) -> &Self;
}

impl UnwrapValue for String {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::String(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for bool {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::Bool(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for f64 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::F64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for f32 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::F32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for isize {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::ISize(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for i64 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::I64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for i32 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::I32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for i16 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::I16(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for i8 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::I8(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for usize {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::USize(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for u64 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::U64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for u32 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::U32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for u16 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::U16(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for u8 {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::U8(ref value) => value,
            _ => unreachable!(),
        }
    }
}
impl UnwrapValue for Vec<String> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecString(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<bool> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecBool(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<f64> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecF64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<f32> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecF32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<isize> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecISize(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<i64> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecI64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<i32> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecI32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<i16> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecI16(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<i8> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecI8(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<usize> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecUSize(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<u64> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecU64(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<u32> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecU32(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<u16> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecU16(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl UnwrapValue for Vec<u8> {
    fn unwrap(value: &Value) -> &Self {
        match *value {
            Value::VecU8(ref value) => value,
            _ => unreachable!(),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::I16(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::I8(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::U32(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::U16(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::U8(value)
    }
}

impl From<Vec<String>> for Value {
    fn from(value: Vec<String>) -> Self {
        Value::VecString(value)
    }
}

impl From<Vec<bool>> for Value {
    fn from(value: Vec<bool>) -> Self {
        Value::VecBool(value)
    }
}

impl From<Vec<f64>> for Value {
    fn from(value: Vec<f64>) -> Self {
        Value::VecF64(value)
    }
}

impl From<Vec<f32>> for Value {
    fn from(value: Vec<f32>) -> Self {
        Value::VecF32(value)
    }
}

impl From<Vec<isize>> for Value {
    fn from(value: Vec<isize>) -> Self {
        Value::VecISize(value)
    }
}

impl From<Vec<i64>> for Value {
    fn from(value: Vec<i64>) -> Self {
        Value::VecI64(value)
    }
}

impl From<Vec<i32>> for Value {
    fn from(value: Vec<i32>) -> Self {
        Value::VecI32(value)
    }
}

impl From<Vec<i16>> for Value {
    fn from(value: Vec<i16>) -> Self {
        Value::VecI16(value)
    }
}

impl From<Vec<i8>> for Value {
    fn from(value: Vec<i8>) -> Self {
        Value::VecI8(value)
    }
}

impl From<Vec<usize>> for Value {
    fn from(value: Vec<usize>) -> Self {
        Value::VecUSize(value)
    }
}

impl From<Vec<u64>> for Value {
    fn from(value: Vec<u64>) -> Self {
        Value::VecU64(value)
    }
}

impl From<Vec<u32>> for Value {
    fn from(value: Vec<u32>) -> Self {
        Value::VecU32(value)
    }
}

impl From<Vec<u16>> for Value {
    fn from(value: Vec<u16>) -> Self {
        Value::VecU16(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::VecU8(value)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::Value::*;
        match *self {
            String(ref value) => serializer.serialize_str(value),
            Bool(value) => serializer.serialize_bool(value),
            F64(value) => serializer.serialize_f64(value),
            F32(value) => serializer.serialize_f32(value),
            ISize(value) => serializer.serialize_i64(value as i64),
            I64(value) => serializer.serialize_i64(value),
            I32(value) => serializer.serialize_i32(value),
            I16(value) => serializer.serialize_i16(value),
            I8(value) => serializer.serialize_i8(value),
            USize(value) => serializer.serialize_u64(value as u64),
            U64(value) => serializer.serialize_u64(value),
            U32(value) => serializer.serialize_u32(value),
            U16(value) => serializer.serialize_u16(value),
            U8(value) => serializer.serialize_u8(value),
            VecString(ref value) => serialize_slice(value, serializer),
            VecBool(ref value) => serialize_slice(value, serializer),
            VecF64(ref value) => serialize_slice(value, serializer),
            VecF32(ref value) => serialize_slice(value, serializer),
            VecISize(ref value) => serialize_slice(value, serializer),
            VecI64(ref value) => serialize_slice(value, serializer),
            VecI32(ref value) => serialize_slice(value, serializer),
            VecI16(ref value) => serialize_slice(value, serializer),
            VecI8(ref value) => serialize_slice(value, serializer),
            VecUSize(ref value) => serialize_slice(value, serializer),
            VecU64(ref value) => serialize_slice(value, serializer),
            VecU32(ref value) => serialize_slice(value, serializer),
            VecU16(ref value) => serialize_slice(value, serializer),
            VecU8(ref value) => serialize_slice(value, serializer),
        }
    }
}

fn serialize_slice<T, S>(value: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where T: Serialize, S: Serializer
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for element in value {
        seq.serialize_element(element)?;
    }
    seq.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_set_string_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());

        assert_eq!(Some(&Value::String("simone".to_owned())), query.params.get("name"));
    }

    #[test]
    fn query_set_bool_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.active = @active RETURN u.name");
        query.set_parameter("active".to_owned(), true);

        assert_eq!(Some(&Value::Bool(true)), query.params.get("active"));
    }

    #[test]
    fn query_set_i64_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id = @id RETURN u.name");
        query.set_parameter("id".to_owned(), -1828359i64);

        assert_eq!(Some(&Value::I64(-1828359)), query.params.get("id"));
    }

    #[test]
    fn query_set_vec_of_u64_parameter() {
        let mut query = Query::new("FOR u IN users FILTER u.id in @ids RETURN u.name");
        let ids: Vec<u64> = vec![1, 2, 3, 4, 5];
        query.set_parameter("ids".to_owned(), ids);

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
        query.set_parameter("ids".to_owned(), Value::VecF32(ids));

        assert_eq!(Some(&vec![1.1f32, 2.2, 3.3, 4.4, 5.5]), query.parameter("ids"));
    }
}
