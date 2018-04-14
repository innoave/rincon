
use std::io;

use futures::Future;

use api;
use api::auth::Jwt;
use api::method::{Method, Prepare};

pub trait Connector {
    type Connection: 'static + Execute;

    fn connection(&self, database_name: &str) -> Self::Connection;

    fn system_connection(&self) -> Self::Connection;

    fn accept_auth_token(&mut self, token: Jwt);

    fn invalidate_auth_token(&mut self);
}

pub trait Execute {
    fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: 'static + Method + Prepare;
}

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=Error>>;

#[derive(Debug, Clone, PartialEq, Eq, Fail)]
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Communication(err.to_string())
    }
}
