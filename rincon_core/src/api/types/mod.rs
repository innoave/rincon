//! Miscellaneous types defined for the rincon core API.

#[cfg(test)]
mod tests;

use std::fmt;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeSeq, Serializer};
use serde_json;

/// The `Url` type used by this crate.
pub type Url = ::url::Url;

/// An entity represented either by its name only or the whole object.
///
/// This type is used mainly in the session API to represent an entity that is
/// either known by its name or the whole object is loaded from the database
/// into the session.
#[derive(Debug)]
pub enum Entity<T> {
    /// An entity where currently only the name is known
    Name(String),
    /// An entity with the whole content loaded
    Object(T),
}

/// Constant instance of the `Empty` struct.
pub const EMPTY: Empty = Empty {};

/// Represents an 'empty' payload of a method where no type information is
/// available.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}

/// The type for JSON values used by this crate.
pub type JsonValue = serde_json::Value;

/// A new type for strings that contain JSON.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonString(String);

impl JsonString {
    /// Creates a new `JsonString` from the given string.
    ///
    /// The given value should convert into a valid JSON string.
    pub fn new<J>(value: J) -> Self
    where
        J: Into<String>,
    {
        JsonString(value.into())
    }

    /// Creates a new `JsonString` from the given string value.
    ///
    /// The given value should convert into a valid JSON string.
    ///
    /// It is not checked whether the given string is a valid JSON string.
    pub fn from_string_unchecked(value: String) -> Self {
        JsonString(value)
    }

    /// Creates a new `JsonString` from the given string slice.
    ///
    /// The given value should convert into a valid JSON string.
    ///
    /// It is not checked whether the given string slice is a valid JSON string.
    pub fn from_str_unchecked(value: &str) -> Self {
        JsonString(value.to_owned())
    }

    /// Converts this `JsonString` into a std string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns this `JsonString` as a reference to a std str.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for JsonString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Serialize for JsonString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let json_value: JsonValue = serde_json::from_str(&self.0).map_err(S::Error::custom)?;
        json_value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for JsonString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let json_value = JsonValue::deserialize(deserializer).map_err(D::Error::custom)?;
        let json_string = serde_json::to_string(&json_value).map_err(D::Error::custom)?;
        Ok(JsonString(json_string))
    }
}

/// Defines the type of the value of a parameter for methods and queries.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A string value
    String(String),

    /// A bool value
    Bool(bool),

    /// A 64bit float value
    F64(f64),

    /// A 32bit float value
    F32(f32),

    /// A isize signed integer value
    ISize(isize),

    /// A 64bit signed integer value
    I64(i64),

    /// A 32bit signed integer value
    I32(i32),

    /// A 16bit signed integer value
    I16(i16),

    /// A 8bit signed integer value
    I8(i8),

    /// A usize unsigned integer value
    USize(usize),

    /// A 64bit unsigned integer value
    U64(u64),

    /// A 32bit unsigned integer value
    U32(u32),

    /// A 16bit unsigned integer value
    U16(u16),

    /// A 8bit unsigned integer value
    U8(u8),

    /// A vec of string values
    VecString(Vec<String>),

    /// A vec of bool values
    VecBool(Vec<bool>),

    /// A vec of 64bit float values
    VecF64(Vec<f64>),

    /// A vec of 32bit float values
    VecF32(Vec<f32>),

    /// A vec of isize signed integer values
    VecISize(Vec<isize>),

    /// A vec of 64bit signed integer values
    VecI64(Vec<i64>),

    /// A vec of 32bit signed integer values
    VecI32(Vec<i32>),

    /// A vec of 16bit signed integer values
    VecI16(Vec<i16>),

    /// A vec of 8bit signed integer values
    VecI8(Vec<i8>),

    /// A vec of usize unsigned integer values
    VecUSize(Vec<usize>),

    /// A vec of 64bit unsigned integer values
    VecU64(Vec<u64>),

    /// A vec of 32bit unsigned integer values
    VecU32(Vec<u32>),

    /// A vec of 16bit unsigned integer values
    VecU16(Vec<u16>),

    /// A vec of 8bit unsigned integer values
    VecU8(Vec<u8>),
}

impl Value {
    /// Unwraps the value of the underlying type out of this `Value`.
    pub fn unwrap<T>(&self) -> &T
    where
        T: UnwrapValue,
    {
        UnwrapValue::unwrap(self)
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        match *self {
            Value::String(ref value) => value,
            _ => unreachable!(),
        }
    }
}

/// Defines how to unwrap the value out of the `Value` enum.
///
/// This trait should be implemented for all types that can be wrapped inside
/// the `Value` enum.
pub trait UnwrapValue {
    /// Returns the value of the underlying type of this `Value`.
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

impl<'a> From<&'a str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
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

impl<'a> From<Vec<&'a str>> for Value {
    fn from(value: Vec<&str>) -> Self {
        Value::VecString(value.iter().map(|v| v.to_string()).collect())
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match *self {
            String(ref value) => format_value(value, f),
            Bool(ref value) => format_value(value, f),
            F64(ref value) => format_value(value, f),
            F32(ref value) => format_value(value, f),
            ISize(ref value) => format_value(value, f),
            I64(ref value) => format_value(value, f),
            I32(ref value) => format_value(value, f),
            I16(ref value) => format_value(value, f),
            I8(ref value) => format_value(value, f),
            USize(ref value) => format_value(value, f),
            U64(ref value) => format_value(value, f),
            U32(ref value) => format_value(value, f),
            U16(ref value) => format_value(value, f),
            U8(ref value) => format_value(value, f),
            VecString(ref value) => format_value_list(value, f),
            VecBool(ref value) => format_value_list(value, f),
            VecF64(ref value) => format_value_list(value, f),
            VecF32(ref value) => format_value_list(value, f),
            VecISize(ref value) => format_value_list(value, f),
            VecI64(ref value) => format_value_list(value, f),
            VecI32(ref value) => format_value_list(value, f),
            VecI16(ref value) => format_value_list(value, f),
            VecI8(ref value) => format_value_list(value, f),
            VecUSize(ref value) => format_value_list(value, f),
            VecU64(ref value) => format_value_list(value, f),
            VecU32(ref value) => format_value_list(value, f),
            VecU16(ref value) => format_value_list(value, f),
            VecU8(ref value) => format_value_list(value, f),
        }
    }
}

fn format_value<T>(value: &T, f: &mut fmt::Formatter) -> fmt::Result
where
    T: ToString,
{
    f.write_str(&value.to_string())
}

fn format_value_list<T>(values: &[T], f: &mut fmt::Formatter) -> fmt::Result
where
    T: ToString,
{
    let mut iter = values.iter();
    f.write_str("[")?;
    if let Some(first) = iter.next() {
        f.write_str(&first.to_string())?;
    }
    #[cfg_attr(feature = "cargo-clippy", allow(while_let_on_iterator))]
    while let Some(value) = iter.next() {
        f.write_str(",")?;
        format_value(value, f)?;
    }
    f.write_str("]")
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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
where
    T: Serialize,
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for element in value {
        seq.serialize_element(element)?;
    }
    seq.end()
}
