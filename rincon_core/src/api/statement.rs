
use futures::{Future, Poll};

use api::method::{Method, Operation, Parameters};
use api::types::Document;

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
            parameters: Parameters::empty(),
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

#[derive(Debug, Clone)]
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
pub struct FutureResult<M>(Box<Future<Item=<M as Method>::Result, Error=Error>>)
    where M: Method;

impl<M> Future for FutureResult<M>
    where M: Method
{
    type Item = <M as Method>::Result;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Debug)]
pub enum Error {

}
