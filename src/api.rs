
use std::fmt::{self, Debug};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;

use serde::de::DeserializeOwned;

pub trait Method {
    type Result: DeserializeOwned + 'static;
}

pub trait Prepare {
    fn operation(&self) -> Operation;
    fn path(&self) -> &str;
    fn parameters(&self) -> Parameters;
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Result<T> {
    error: bool,
    code: ResultCode,
    result: T,
}

impl<T> Result<T> {
    pub fn is_error(&self) -> bool {
        self.error
    }

    pub fn code(&self) -> ResultCode {
        self.code
    }

    pub fn result(&self) -> &T {
        &self.result
    }
}

pub type ResultCode = i32;

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

#[derive(Clone, Debug)]
pub struct Document {
    text: String,
}

impl Document {
    pub fn from_str(text: &str) -> Self {
        Document {
            text: text.to_owned(),
        }
    }

    pub fn from_string(text: String) -> Self {
        Document {
            text,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
