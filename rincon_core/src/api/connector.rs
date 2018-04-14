//! Definition of Connector and Connection.
//!
//! The rincon driver uses a connector that implements the communication layer
//! to the ArangoDB server. It defines the transport protocol like HTTP or HTTPS
//! and the format for serializing the payload of requests and responses, like
//! JSON for example.
//!
//! A connector establishes connections to a server. A connection is used to
//! actually execute a method call.
//!
//! A connector is defined to implement the `Connector` trait. A connection as
//! established and provided by a `Connector` is of type `Connector::Connection`
//! The `Connection` type is bound to implement the `Execute` trait.
//!
//! Depending on the implementation of a connector it can be simply a factory
//! for connections or be more advanced by maintaining a pool of connections.
//!
//! Implementations of a connection know how to serialize and send method calls
//! to the server and deserialize the responses into the defined result type.
use std::io;

use futures::Future;

use api;
use api::auth::Jwt;
use api::method::{Method, Prepare};

/// A connector establishes and provides connections to a server.
pub trait Connector {
    /// The type of connections this connector provides.
    type Connection: 'static + Execute;

    /// Establishes a connection to the database with the given name and returns
    /// the connection.
    ///
    /// More sophisticated implementations may also maintain a pool of
    /// connections and return available connections from that pool.
    fn connection(&self, database_name: &str) -> Self::Connection;

    /// Establishes a connection to the system database as defined by ArangoDB
    /// and returns the connection.
    fn system_connection(&self) -> Self::Connection;

    /// Tells this connector to use the given token for authentication.
    fn use_auth_token(&mut self, token: Jwt);

    /// Tells this connector to no longer use the configured token for
    /// authentication.
    fn invalidate_auth_token(&mut self);
}

/// A type that can execute method calls.
///
/// Any type that implements this `Execute` trait can be returned by the
/// `Connector` as `Connector::Connection`.
pub trait Execute {
    /// Executes the given method asynchronously and returns a future result.
    fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: 'static + Method + Prepare;
}

/// The result of any asynchronous method call
pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=Error>>;

/// The type of error that can occur during communication with the server.
#[derive(Debug, Clone, PartialEq, Eq, Fail)]
pub enum Error {
    /// A communication error like server not responding or IO error.
    #[fail(display = "Communication failed: {}", _0)]
    Communication(String),
    /// An error that occurs during deserialization of a response.
    #[fail(display = "Deserialization failed: {}", _0)]
    Deserialization(String),
    /// An error that occurs during method execution.
    ///
    /// This error signals ArangoDB specific errors like AQL syntax error or
    /// document not found.
    #[fail(display = "Execution of method failed: {}", _0)]
    Method(api::Error),
    /// The caller is not authenticated to the server.
    #[fail(display = "Not authenticated to datasource: {}", _0)]
    NotAuthenticated(String),
    /// An error that occurs during serialization of a request.
    #[fail(display = "Serialization failed: {}", _0)]
    Serialization(String),
    /// A timeout occurred during method call execution.
    #[fail(display = "Timeout on request: {}", _0)]
    Timeout(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Communication(err.to_string())
    }
}
