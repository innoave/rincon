
pub use arango::error_code::ErrorCode;

use std::fmt::{self, Debug, Display};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::types::Value;

pub trait Method {
    type Result: DeserializeOwned;
    const RETURN_TYPE: RpcReturnType;

    fn return_type(&self) -> RpcReturnType {
        Self::RETURN_TYPE
    }
}

pub trait Prepare {
    type Content: Serialize;

    fn operation(&self) -> Operation;

    fn path(&self) -> String;

    fn parameters(&self) -> Parameters;

    fn header(&self) -> Parameters;

    fn content(&self) -> Option<&Self::Content>;
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Create,
    Read,
    Modify,
    Replace,
    Delete,
    ReadHeader,
}

#[derive(Clone, PartialEq)]
pub struct Parameters {
    list: Vec<(String, Value)>,
}

impl Parameters {
    pub fn empty() -> Self {
        Parameters {
            list: Vec::new(),
        }
    }

    pub fn new() -> Self {
        Parameters {
            list: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Parameters {
            list: Vec::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn iter(&self) -> ParameterIter {
        ParameterIter {
            inner: self.list.iter(),
        }
    }

    pub fn insert<K, V>(&mut self, name: K, value: V)
        where K: Into<String>, V: Into<Value>
    {
        self.list.push((name.into(), value.into()));
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            list: Vec::default(),
        }
    }
}

impl Debug for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Parameters")
            .and_then(|_| f.debug_list().entries(self.iter()).finish())
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        display.push_str("Parameters[");
        if !self.list.is_empty() {
            for &(ref key, ref value) in self.list.iter() {
                display.push_str(key);
                display.push('=');
                display.push_str(&value.to_string());
                display.push(',');
                display.push(' ');
            }
            display.pop();
            display.pop();
        }
        display.push(']');
        f.write_str(&display)
    }
}

impl<K, V> From<Vec<(K, V)>> for Parameters
    where K: Into<String>, V: Into<Value>
{
    fn from(list: Vec<(K, V)>) -> Self {
        Parameters::from_iter(list)
    }
}

impl<K, V> FromIterator<(K, V)> for Parameters
    where K: Into<String>, V: Into<Value>
{
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(iter.into_iter().map(|(k, v)| (k.into(), v.into()))),
        }
    }
}

impl<'i, K, V> FromIterator<&'i (K, V)> for Parameters
    where K: Into<String> + Clone, V: Into<Value> + Clone
{
    fn from_iter<T: IntoIterator<Item=&'i (K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(iter.into_iter().map(|&(ref k, ref v)|
                (k.clone().into(), v.clone().into()))),
        }
    }
}

impl<K, V> Extend<(K, V)> for Parameters
    where K: Into<String>, V: Into<Value>
{
    fn extend<T: IntoIterator<Item=(K, V)>>(&mut self, iter: T) {
        self.list.extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    }
}

#[derive(Debug)]
pub struct ParameterIter<'i> {
    inner: Iter<'i, (String, Value)>,
}

impl<'i> Iterator for ParameterIter<'i> {
    type Item = &'i (String, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'i> ExactSizeIterator for ParameterIter<'i> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct RpcReturnType {
    pub result_field: Option<&'static str>,
    pub code_field: Option<&'static str>,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Error {
    #[serde(rename = "code")]
    status_code: u16,
    #[serde(rename = "errorNum")]
    error_code: ErrorCode,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    pub fn new<M>(status_code: u16, error_code: ErrorCode, message: M) -> Self
        where M: Into<String>
    {
        Error {
            status_code,
            error_code,
            message: message.into(),
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn error_code(&self) -> ErrorCode {
        self.error_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {} (Status: {})",
            &self.error_code.as_u16(), &self.message, &self.status_code))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {} (Status: {})",
            &self.error_code.as_u16(), &self.message, &self.status_code))
    }
}
