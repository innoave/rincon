
use std;
use std::fmt::{self, Debug, Display};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;
use std::vec::IntoIter;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::types::Value;
use arango::ErrorCode;

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
            for &(ref key, ref value) in &self.list {
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
#[derive(Debug, Clone, PartialEq)]
pub struct RpcReturnType {
    pub result_field: Option<&'static str>,
    pub code_field: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Result<T> {
    Success(T),
    Failed(Error),
}

impl<T> Result<T> {
    pub fn into_std_result(self) -> std::result::Result<T, Error> {
        use self::Result::*;
        match self {
            Success(value) => Ok(value),
            Failed(error) => Err(error),
        }
    }

    pub fn as_std_result(&self) -> std::result::Result<&T, &Error> {
        use self::Result::*;
        match *self {
            Success(ref value) => Ok(value),
            Failed(ref error) => Err(error),
        }
    }
}

impl<T> From<std::result::Result<T, Error>> for Result<T> {
    fn from(value: std::result::Result<T, Error>) -> Self {
        use self::Result::*;
        match value {
            Ok(value) => Success(value),
            Err(error) => Failed(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ResultList<T>(#[serde(bound(deserialize = "T: DeserializeOwned"))] Vec<Result<T>>);

impl<T> ResultList<T> {
    pub fn get(&self, index: usize) -> Option<std::result::Result<&T, &Error>> {
        self.0.get(index).map(|x| x.as_std_result())
    }

    pub fn iter(&self) -> ResultListIter<T> {
        ResultListIter(self.0.iter())
    }
}

impl<T> FromIterator<std::result::Result<T, Error>> for ResultList<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=std::result::Result<T, Error>>
    {
        ResultList(Vec::from_iter(iter.into_iter().map(From::from)))
    }
}

impl<T> FromIterator<Result<T>> for ResultList<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=Result<T>>
    {
        ResultList(Vec::from_iter(iter.into_iter()))
    }
}

impl<T> IntoIterator for ResultList<T>
    where T: DeserializeOwned
{
    type Item = std::result::Result<T, Error>;
    type IntoIter = ResultListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ResultListIntoIter(self.0.into_iter())
    }
}

#[derive(Debug)]
pub struct ResultListIntoIter<T>(IntoIter<Result<T>>);

impl<T> Iterator for ResultListIntoIter<T> {
    type Item = std::result::Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.into_std_result())
    }
}

impl<T> ExactSizeIterator for ResultListIntoIter<T> {}

#[derive(Debug)]
pub struct ResultListIter<'a, T: 'a>(Iter<'a, Result<T>>);

impl<'a, T> Iterator for ResultListIter<'a, T> {
    type Item = std::result::Result<&'a T, &'a Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.as_std_result())
    }
}

impl<'a, T> ExactSizeIterator for ResultListIter<'a, T> {}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Error {
    #[serde(rename = "errorNum")]
    code: ErrorCode,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    pub fn new<M>(code: ErrorCode, message: M) -> Self
        where M: Into<String>
    {
        Error {
            code,
            message: message.into(),
        }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {}",
            &self.code.as_u16(), &self.message))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {}",
            &self.code.as_u16(), &self.message))
    }
}
