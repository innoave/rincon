
mod error_code;
pub use self::error_code::ErrorCode;

use std::fmt::{self, Debug};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;

use serde::de::DeserializeOwned;

pub trait Method {
    type Result: DeserializeOwned + 'static;
    const ERROR_TYPE: RpcErrorType;

    fn error_type(&self) -> RpcErrorType {
        Self::ERROR_TYPE
    }
}

pub trait Prepare {
    fn operation(&self) -> Operation;
    fn path(&self) -> &str;
    fn parameters(&self) -> Parameters;
}

#[derive(Clone, Debug)]
pub enum Operation {
    Create,
    Read,
    Modify,
    Replace,
    Delete,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Parameters {
    list: Vec<(String, String)>,
}

impl Parameters {
    pub fn empty() -> Self {
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
        ParameterIter(self.list.iter())
    }

    pub fn push<K, V>(&mut self, name: K, value: V)
        where K: Into<String>, V: Into<String>
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
            .and_then(|_| f.debug_list().entries(self.list.iter()).finish())
    }
}

impl<K, V> From<Vec<(K, V)>> for Parameters
    where K: Into<String>, V: Into<String>
{
    fn from(list: Vec<(K, V)>) -> Self {
        Parameters::from_iter(list)
    }
}

impl<K, V> FromIterator<(K, V)> for Parameters
    where K: Into<String>, V: Into<String>
{
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(iter.into_iter().map(|(k, v)| (k.into(), v.into()))),
        }
    }
}

impl<'a, K, V> FromIterator<&'a (K, V)> for Parameters
    where K: Into<String> + Clone, V: Into<String> + Clone
{
    fn from_iter<T: IntoIterator<Item=&'a (K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(iter.into_iter().map(|&(ref k, ref v)|
                (k.clone().into(), v.clone().into()))),
        }
    }
}

impl<K, V> Extend<(K, V)> for Parameters
    where K: Into<String>, V: Into<String>
{
    fn extend<T: IntoIterator<Item=(K, V)>>(&mut self, iter: T) {
        self.list.extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    }
}

#[derive(Debug)]
pub struct ParameterIter<'a>(Iter<'a, (String, String)>);

impl<'a> Iterator for ParameterIter<'a> {
    type Item = &'a (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for ParameterIter<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RpcErrorType {
    pub result_field: Option<&'static str>,
    pub code_field: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    code: ErrorCode,
    message: String,
}
