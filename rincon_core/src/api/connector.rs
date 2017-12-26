
use futures::Future;

use api;
use api::method::{Method, Prepare};

pub trait Execute {
    fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare + 'static;
}

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=Error>>;

#[derive(Clone, Debug, PartialEq, Eq, Fail)]
pub enum Error {
    #[fail(display = "Communication failed: {}", _0)]
    Communication(String),
    #[fail(display = "Deserialization failed: {}", _0)]
    Deserialization(String),
    #[fail(display = "Execution of method failed: {}", _0)]
    Method(api::Error),
    #[fail(display = "Not authenticated to datasource: {}", _0)]
    NotAuthenticated(String),
    #[fail(display = "Serialization failed: {}", _0)]
    Serialization(String),
    #[fail(display = "Timeout on request: {}", _0)]
    Timeout(String),
}
