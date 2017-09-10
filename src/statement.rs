
use std::collections::HashMap;

use futures::{Future, Poll};
use serde::de::DeserializeOwned;

pub trait Method {
    type Result: DeserializeOwned + 'static;
}

pub trait Prepare<M>
    where M: Method
{
    fn prepare(self) -> PreparedStatement<M>;
}

#[derive(Debug)]
pub struct PreparedStatement<M>
    where M: Method
{
    method: M,
    operation: Operation,
    path: String,
    parameters: Parameters,
    document: Option<Document>,
}

impl<M> PreparedStatement<M>
    where M: Method
{
    pub fn new(method: M, operation: Operation, path: &str) -> Self {
        PreparedStatement {
            method,
            operation,
            path: path.to_owned(),
            parameters: Parameters::default(),
            document: None,
        }
    }

    pub fn with_parameters(method: M, operation: Operation, path: &str, parameters: Parameters)
        -> Self
    {
        PreparedStatement {
            method,
            operation,
            path: path.to_owned(),
            parameters,
            document: None,
        }
    }

    pub fn with_document(mut self, document: Document) -> Self {
        self.document = Some(document);
        self
    }

    pub fn method(&self) -> &M {
        &self.method
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut Parameters {
        &mut self.parameters
    }

    pub fn document(&self) -> Option<&Document> {
        self.document.as_ref()
    }
}

#[derive(Debug)]
pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Clone, Debug)]
pub struct Parameters {
    map: HashMap<String, String>,
}

impl Parameters {
    pub fn set_str(&mut self, name: &str, value: &str) {
        self.map.insert(name.to_owned(), value.to_owned());
    }

    pub fn set_string(&mut self, name: String, value: String) {
        self.map.insert(name, value);
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            map: HashMap::new(),
        }
    }
}

impl From<HashMap<String, String>> for Parameters {
    fn from(map: HashMap<String, String>) -> Self {
        Parameters {
            map,
        }
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

#[derive(Clone, Debug)]
pub struct StatementResult {
    document: Option<Document>,
}

impl StatementResult {
    pub fn new(document: Option<Document>) -> Self {
        StatementResult {
            document,
        }
    }

    pub fn document(&self) -> Option<&Document> {
        self.document.as_ref()
    }
}

/// A `Future` that will resolve to a result of a `Method` execution.
pub struct FutureResult<M>(Box<Future<Item=<M as Method>::Result, Error=self::Error>>)
    where M: Method;

impl<M> Future for FutureResult<M>
    where M: Method
{
    type Item = <M as Method>::Result;
    type Error = self::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Debug)]
pub enum Error {

}
