//! Method Calls as Data
//!
//! The REST API of ArangoDB defines operations to be invoked by a client
//! application. In this driver we speak of a method call for the invocation
//! of a REST operation. Each operation or method is defined by its input
//! parameters and its return type. The input parameters may be mandatory or
//! optional.
//!
//! In rincon method calls are represented as structs that implement the
//! `Method` and the `Prepare` traits. Hence method calls are in fact structs
//! that hold the data necessary to invoke an operation.
//!
//! The big advantage of defining method calls as data types is that instances
//! of concrete method calls can be easily queued, distributed, repeated, cached
//! or processed in batches. Further it is very easy to extend the driver with
//! new operations by simple defining a new struct for each new operation, done.

use std;
use std::fmt::{self, Debug, Display};
use std::iter::{ExactSizeIterator, FromIterator, Iterator};
use std::slice::Iter;
use std::vec::IntoIter;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::types::Value;
use arango::ErrorCode;

/// A `Method` type can be used to represent method calls.
///
/// This trait defines the result type of a method. As the rincon driver uses
/// the `serde` crate as its serialization framework the return type must
/// implement the `DeserializeOwned` trait of `serde`.
pub trait Method {
    /// The type of the result of a method call.
    type Result: DeserializeOwned;

    /// Specification of the fields of RPC-like return type.
    ///
    /// The design of the ArangoDB REST API is not very type safe friendly as
    /// different operations use different fields in the top level result type.
    /// Mainly it does not stick to one RPC-like style of fields returned as
    /// operation results. Hence this hack is used to overcome this issue and
    /// allows the driver to always return specific types for each operation.
    const RETURN_TYPE: RpcReturnType;

    /// Returns the specification of the RPC-like return type.
    fn return_type(&self) -> RpcReturnType {
        Self::RETURN_TYPE
    }
}

/// A `Prepare` type of a method call is used to convert the method call into
/// a concrete request that is specific to the protocol used by a `Connector`.
///
/// For example the `JsonHttpConnector` converts the `Prepare` type into a
/// HTTP-request and serializes the content as JSON into the body of the
/// request.
///
/// As the rincon driver uses the `serde` crate as its serialization framework
/// the content type must implement the `Serialize` trait of `serde`.
pub trait Prepare {
    /// The type of the content of a method call.
    ///
    /// The content is the part of a request that is sent within the body of a
    /// REST call.
    type Content: Serialize;

    /// Returns the type of operation this method is executing.
    fn operation(&self) -> Operation;

    /// Returns the resource path of a REST operation.
    fn path(&self) -> String;

    /// Returns the query parameters of this method.
    ///
    /// The query parameters are usually part of the URL of a REST call.
    fn parameters(&self) -> Parameters;

    /// Returns the header parameters of this method.
    ///
    /// The header parameters are usually sent with the HTTP header of a REST
    /// call.
    fn header(&self) -> Parameters;

    /// Returns the content of this method if any.
    ///
    /// The content of a method is usually sent within the body of a REST call.
    fn content(&self) -> Option<&Self::Content>;
}

/// Enumeration of the used operation of a REST API.
///
/// The operations are defined in a logical sense thus being abstract over the
/// HTTP operations like POST, GET, PUT, PATCH, etc.
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    /// Create a new entity
    Create,
    /// Get an entity or resource
    Read,
    /// Modify an existing entity
    Modify,
    /// Replace an existing entity
    Replace,
    /// Delete an entity
    Delete,
    /// Get the header data or short info about an entity
    ReadHeader,
}

/// A new type for a set of parameters or name/value pairs.
///
/// Each parameter consists of the name of the parameter and its value.
#[derive(Clone, PartialEq)]
pub struct Parameters {
    list: Vec<(String, Value)>,
}

impl Parameters {
    /// Creates and empty set of parameters.
    pub fn empty() -> Self {
        Parameters { list: Vec::new() }
    }

    /// Creates a new set of parameters, which is empty.
    pub fn new() -> Self {
        Parameters { list: Vec::new() }
    }

    /// Creates a new set of parameters with the given capacity.
    ///
    /// When the number of parameters to be inserted in the new parameter set
    /// is known beforehand using this function can speed up memory allocation.
    pub fn with_capacity(capacity: usize) -> Self {
        Parameters {
            list: Vec::with_capacity(capacity),
        }
    }

    /// Returns whether this parameter set is empty.
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Returns an `Iterator` over the parameters in this set.
    pub fn iter(&self) -> ParameterIter {
        ParameterIter {
            inner: self.list.iter(),
        }
    }

    /// Inserts a name/value pair as a new parameter into this set.
    pub fn insert<K, V>(&mut self, name: K, value: V)
    where
        K: Into<String>,
        V: Into<Value>,
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
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(list: Vec<(K, V)>) -> Self {
        Parameters::from_iter(list)
    }
}

impl<K, V> FromIterator<(K, V)> for Parameters
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(iter.into_iter().map(|(k, v)| (k.into(), v.into()))),
        }
    }
}

impl<'i, K, V> FromIterator<&'i (K, V)> for Parameters
where
    K: Into<String> + Clone,
    V: Into<Value> + Clone,
{
    fn from_iter<T: IntoIterator<Item = &'i (K, V)>>(iter: T) -> Parameters {
        Parameters {
            list: Vec::from_iter(
                iter.into_iter()
                    .map(|&(ref k, ref v)| (k.clone().into(), v.clone().into())),
            ),
        }
    }
}

impl<K, V> Extend<(K, V)> for Parameters
where
    K: Into<String>,
    V: Into<Value>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.list
            .extend(iter.into_iter().map(|(k, v)| (k.into(), v.into())));
    }
}

/// An `Iterator` over the references to name/value pairs of a parameter set.
///
/// This iterator is created by calling the `Parameters.iter()` function.
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

/// Specification of the fields of RPC-like return type.
///
/// The design of the ArangoDB REST API is not very type safe friendly as
/// different operations use different fields in the top level result type.
/// Mainly it does not stick to one RPC-like style of fields returned as
/// operation results. Hence this hack is used to overcome this issue and
/// allows the driver to always return specific types for each operation.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub struct RpcReturnType {
    /// The name of the result field if any or none if the fields of the result
    /// type are at the top level returned object.
    pub result_field: Option<&'static str>,

    /// The name of the code field that contains the error code if there is any
    /// or none if the result will never contain an error code.
    pub code_field: Option<&'static str>,
}

/// A container for entities in the result of a method call, where the result
/// contains a list of entities and there might be error information for single
/// results only, not the whole operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Result<T> {
    /// The successful result for one single entity of an operation that has
    /// been executed for multiple entities.
    Success(T),
    /// The error for one single result that occurred during execution of an
    /// operation for multiple entities.
    Failed(Error),
}

impl<T> Result<T> {
    /// Converts this `Result` into a `::std::result::Result`.
    pub fn into_std_result(self) -> std::result::Result<T, Error> {
        use self::Result::*;
        match self {
            Success(value) => Ok(value),
            Failed(error) => Err(error),
        }
    }

    /// Returns this `Result` as a `::std::result::Result` to the references
    /// of the success or error value.
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

/// A container for a list of `Result`s. This type is used as return type for
/// methods that operate on a list of entities where the result can contain
/// error information for single entities only, not the whole operation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ResultList<T>(#[serde(bound(deserialize = "T: DeserializeOwned"))] Vec<Result<T>>);

impl<T> ResultList<T> {
    /// Returns the result at the given index if available.
    pub fn get(&self, index: usize) -> Option<std::result::Result<&T, &Error>> {
        self.0.get(index).map(|x| x.as_std_result())
    }

    /// Returns an `Iterator` over the result items in this result list.
    pub fn iter(&self) -> ResultListIter<T> {
        ResultListIter(self.0.iter())
    }
}

impl<T> FromIterator<std::result::Result<T, Error>> for ResultList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = std::result::Result<T, Error>>,
    {
        ResultList(Vec::from_iter(iter.into_iter().map(From::from)))
    }
}

impl<T> FromIterator<Result<T>> for ResultList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Result<T>>,
    {
        ResultList(Vec::from_iter(iter.into_iter()))
    }
}

impl<T> IntoIterator for ResultList<T>
where
    T: DeserializeOwned,
{
    type Item = std::result::Result<T, Error>;
    type IntoIter = ResultListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ResultListIntoIter(self.0.into_iter())
    }
}

/// An `Iterator` over the result items of a `ResultList`.
///
/// This iterator is returned by the `ResultList.into_iter()` function.
#[derive(Debug)]
pub struct ResultListIntoIter<T>(IntoIter<Result<T>>);

impl<T> Iterator for ResultListIntoIter<T> {
    type Item = std::result::Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.into_std_result())
    }
}

impl<T> ExactSizeIterator for ResultListIntoIter<T> {}

/// An `Iterator` over references to the result items of a `ResultList`.
///
/// This iterator is returned by the `ResulList.iter()` function.
#[derive(Debug)]
pub struct ResultListIter<'a, T: 'a>(Iter<'a, Result<T>>);

impl<'a, T> Iterator for ResultListIter<'a, T> {
    type Item = std::result::Result<&'a T, &'a Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.as_std_result())
    }
}

impl<'a, T> ExactSizeIterator for ResultListIter<'a, T> {}

/// Represents an error that occurs during method execution.
///
/// A method `Error` is returned by the ArangoDB server when the execution of
/// an operation fails.
#[derive(Clone, PartialEq, Deserialize)]
pub struct Error {
    #[serde(rename = "errorNum")]
    code: ErrorCode,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    /// Contructs a new method `Error` with given code and message.
    pub fn new<M>(code: ErrorCode, message: M) -> Self
    where
        M: Into<String>,
    {
        Error {
            code,
            message: message.into(),
        }
    }

    /// Returns the code of this error.
    ///
    /// The possible error codes are defined by ArangoDB.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Returns the message of this error.
    ///
    /// The message coming with an error is received from the ArangoDB server.
    /// The driver transparently copies the error message into this property of
    /// the `Error` struct.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {}", &self.code.as_u16(), &self.message))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {}", &self.code.as_u16(), &self.message))
    }
}
