
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;

use futures::{future, Future, Stream};
use hyper::{self, Client, Request, Uri};
use hyper::client::HttpConnector;
use serde_json;
use tokio_core::reactor::Core;

use statement::{Method, Operation, Prepare, PreparedStatement};
use datasource::DataSource;

#[derive(Debug)]
pub struct Connection {
    datasource: DataSource,
    client: Client<HttpConnector>,
}

impl Connection {
    pub fn establish(datasource: DataSource) -> Result<Self, io::Error> {
        let core = Core::new()?;
        let client = Client::new(&core.handle());
        Ok(Connection {
            datasource,
            client,
        })
    }

    pub fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare<M>
    {
        let stmt = method.prepare();
        let op = http_method_for_operation(stmt.operation());
        let uri = self.build_request_uri(stmt);
        let request = Request::new(op, uri);
        Box::new(self.client.request(request)
            .from_err()
            .and_then(|res|
                res.body().concat2().from_err().and_then(|buffer| {
                    match serde_json::from_slice(&buffer) {
                        Ok(value) =>
                            future::ok(value),
                        Err(err) =>
                            future::err(Error::DeserializationFailed(err)),
                    }
                })
            )
        )
    }

    fn build_request_uri<M>(&self, stmt: PreparedStatement<M>) -> Uri
        where M: Method
    {
        Uri::from_str(&format!("{}://{}:{}/{}",
            self.datasource.protocol(),
            self.datasource.host(),
            self.datasource.port(),
            stmt.path(),
        )).unwrap()
    }
}

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=self::Error>>;

#[derive(Debug)]
pub enum Error {
    CommunicationFailed(hyper::Error),
    DeserializationFailed(serde_json::Error),
    NotUtf8Content(FromUtf8Error),
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::NotUtf8Content(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::CommunicationFailed(err)
    }
}

pub fn http_method_for_operation(operation: &Operation) -> hyper::Method {
    use self::hyper::Method;
    match *operation {
        Operation::Create => Method::Post,
        Operation::Read => Method::Get,
        Operation::Update => Method::Patch,
        Operation::Delete => Method::Delete,
    }
}
