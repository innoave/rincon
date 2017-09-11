
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::time::Duration;

use futures::{future, Future, Stream};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic, Bearer};
use hyper_tls::HttpsConnector;
use native_tls;
use serde_json;
use tokio_core::reactor;
use tokio_timer::{Timer, TimeoutError, TimerError};

use statement::{Method, Operation, Prepare, PreparedStatement};
use datasource::{Authentication, DataSource};

#[derive(Debug)]
pub struct Connection {
    datasource: DataSource,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Connection {
    pub fn establish(datasource: DataSource, reactor: &reactor::Handle)
        -> Result<Self, self::Error>
    {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &reactor)?)
            .build(reactor);
        debug!("Created connection for {:?}", &datasource);
        Ok(Connection {
            datasource,
            client,
        })
    }

    pub fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare<M>
    {
        let timeout = Timer::default().sleep(self.datasource.timeout().clone());
        let stmt = method.prepare();
        let operation = stmt.operation();
        let http_method = http_method_for_operation(operation);
        let uri = self.build_request_uri(&stmt);
        let mut request = Request::new(http_method, uri);
        {
            request.set_version(HttpVersion::Http11);
            let mut headers = request.headers_mut();
            match *operation {
                Operation::Create => {},
                Operation::Read => {},
                Operation::Update => {},
                Operation::Delete => {},
            }
            match *self.datasource.authentication() {
                Authentication::Basic(ref credentials) =>
                    headers.set(Authorization(Basic {
                        username: credentials.username().to_owned(),
                        password: Some(credentials.password().to_owned()),
                    })),
                Authentication::None => (),
            }
        }
        debug!("Sending {:?}", &request);
        Box::new(self.client.request(request).from_err()
            .and_then(|res| {
                let status_code = res.status();
                match status_code {
                    StatusCode::Ok
                        | StatusCode::Accepted
                        | StatusCode::Created
                        => future::ok(res),
                    StatusCode::BadRequest
                        | StatusCode::Unauthorized
                        | StatusCode::PaymentRequired
                        | StatusCode::Forbidden
                        | StatusCode::NotFound
                        | StatusCode::MethodNotAllowed
                        | StatusCode::NotAcceptable
                        | StatusCode::ProxyAuthenticationRequired
                        | StatusCode::RequestTimeout
                        | StatusCode::Conflict
                        | StatusCode::Gone
                        | StatusCode::LengthRequired
                        | StatusCode::PreconditionFailed
                        | StatusCode::PayloadTooLarge
                        | StatusCode::UriTooLong
                        | StatusCode::UnsupportedMediaType
                        | StatusCode::RangeNotSatisfiable
                        | StatusCode::ExpectationFailed
                        | StatusCode::ImATeapot
                        | StatusCode::MisdirectedRequest
                        | StatusCode::UnprocessableEntity
                        | StatusCode::Locked
                        | StatusCode::FailedDependency
                        | StatusCode::UpgradeRequired
                        | StatusCode::PreconditionRequired
                        | StatusCode::TooManyRequests
                        | StatusCode::RequestHeaderFieldsTooLarge
                        | StatusCode::UnavailableForLegalReasons
                        | StatusCode::InternalServerError
                        | StatusCode::NotImplemented
                        | StatusCode::BadGateway
                        | StatusCode::ServiceUnavailable
                        | StatusCode::GatewayTimeout
                        | StatusCode::HttpVersionNotSupported
                        | StatusCode::VariantAlsoNegotiates
                        | StatusCode::InsufficientStorage
                        | StatusCode::LoopDetected
                        | StatusCode::NotExtended
                        | StatusCode::NetworkAuthenticationRequired
                        | StatusCode::Unregistered(_)
                        => future::err(Error::from(status_code)),
                    _ => future::err(Error::from(status_code)),
                }
            })
            .and_then(|res| {
                res.body().concat2().from_err().and_then(|buffer| {
                    match serde_json::from_slice(&buffer) {
                        Ok(value) =>
                            future::ok(value),
                        Err(err) =>
                            future::err(Error::DeserializationFailed(err)),
                    }
                })
            })
        )
    }

    fn build_request_uri<M>(&self, stmt: &PreparedStatement<M>) -> Uri
        where M: Method
    {
        Uri::from_str(&format!("{}://{}:{}{}",
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
    HttpError(hyper::StatusCode),
    IoError(io::Error),
    NativeTlsError(native_tls::Error),
    NotUtf8Content(FromUtf8Error),
    Timeout(TimeoutError<hyper::Error>),
    TimerError(TimerError),
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::NotUtf8Content(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::CommunicationFailed(err)
    }
}

impl From<native_tls::Error> for Error {
    fn from(err: native_tls::Error) -> Self {
        Error::NativeTlsError(err)
    }
}

impl From<TimeoutError<hyper::Error>> for Error {
    fn from(err: TimeoutError<hyper::Error>) -> Self {
        Error::Timeout(err)
    }
}

impl From<TimerError> for Error {
    fn from(err: TimerError) -> Self {
        Error::TimerError(err)
    }
}

impl From<StatusCode> for Error {
    fn from(status_code: StatusCode) -> Self {
        Error::HttpError(status_code)
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
