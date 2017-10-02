
mod error_code;
pub use self::error_code::ErrorCode;

use std::fmt::{self, Debug, Display};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;

use serde::de::{self, Deserialize, Deserializer, DeserializeOwned, Visitor};
use serde::ser::Serialize;

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
    fn content(&self) -> Option<&Self::Content>;
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Create,
    Read,
    Modify,
    Replace,
    Delete,
}

#[derive(Clone, PartialEq)]
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

impl Display for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        display.push_str("Parameters[");
        if !self.list.is_empty() {
            for &(ref key, ref value) in self.list.iter() {
                display.push_str(key);
                display.push('=');
                display.push_str(value);
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

#[derive(Clone, Debug, PartialEq)]
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

pub const EMPTY: Empty = Empty {};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Empty {}

impl Default for Empty {
    fn default() -> Self {
        Empty {}
    }
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq)]
pub struct RpcReturnType {
    pub result_field: Option<&'static str>,
    pub code_field: Option<&'static str>,
}

/// This enum defines the supported authentication methods.
#[derive(Clone, Debug)]
pub enum Authentication {
    /// Basic authentication.
    Basic(Credentials),
    /// Authentication via JSON Web Token (JWT).
    Jwt(Credentials),
    None,
}

/// This struct holds the credentials needed to authenticate a user.
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Credentials {
    /// The username registered for a user.
    username: String,
    /// The password registered with a users username.
    password: String,
}

impl Credentials {
    /// Constructs new `Credentials` with the given username and password.
    pub fn new<S>(username: S, password: S) -> Self
        where S: Into<String>
    {
        Credentials {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns the username of this `Credentials`.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the password of this `Credentials`.
    pub fn password(&self) -> &str {
        &self.password
    }
}

/// Type definition for a JSON Web Token (JWT).
pub type Jwt = String;

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

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u64(ErrorCodeVisitor)
    }
}

struct ErrorCodeVisitor;

impl<'de> Visitor<'de> for ErrorCodeVisitor {
    type Value = ErrorCode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an u16 integer")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where E: de::Error
    {
        Ok(ErrorCode::from_u16(value as u16))
    }
}
