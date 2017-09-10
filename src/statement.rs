
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
    parameter: HashMap<String, String>,
    document: Option<String>,
}

impl<M> PreparedStatement<M>
    where M: Method
{
    pub fn new(method: M, operation: Operation, path: String) -> Self {
        PreparedStatement {
            method,
            operation,
            path,
            parameter: HashMap::new(),
            document: None,
        }
    }

    pub fn set_document(&mut self, document: String) {
        self.document = Some(document);
    }

    pub fn set_string(&mut self, name: String, value: String) {
        self.parameter.insert(name, value);
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

    pub fn parameter(&self) -> &HashMap<String, String> {
        &self.parameter
    }

    pub fn document(&self) -> &Option<String> {
        &self.document
    }
}

#[derive(Debug)]
pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Debug)]
pub struct StatementResult {
    document: Option<String>,
}

impl StatementResult {
    pub fn new(document: Option<String>) -> Self {
        StatementResult {
            document,
        }
    }

    pub fn document(&self) -> Option<&String> {
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
