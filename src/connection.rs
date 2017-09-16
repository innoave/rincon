
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::time::Duration;

use futures::{future, Future, Stream};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic, Bearer, UserAgent};
use hyper_tls::HttpsConnector;
use native_tls;
use serde_json;
use tokio_core::reactor;
use tokio_timer::{Timer, TimeoutError, TimerError};

use api::{Method, Operation, Prepare};
use datasource::{Authentication, DataSource};

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; ArangoDB-RustDriver/1.1)";

#[derive(Debug)]
pub struct Connection {
    datasource: DataSource,
    client: Client<HttpsConnector<HttpConnector>>,
    user_agent: String,
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
            user_agent: DEFAULT_USER_AGENT.to_owned(),
        })
    }

    pub fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare
    {
        let timeout = Timer::default().sleep(self.datasource.timeout().clone());
        let request = self.prepare_request(method);
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

    pub fn prepare_request<M>(&self, method: M) -> Request
        where M: Method + Prepare
    {
        let operation = method.operation();
        let http_method = http_method_for_operation(&operation);
        let uri = build_request_uri(&self.datasource, &method);
        let mut request = Request::new(http_method, uri);
        request.set_version(HttpVersion::Http11);
        {
            let mut headers = request.headers_mut();
            headers.set(UserAgent::new(self.user_agent.clone()));
            match *self.datasource.authentication() {
                Authentication::Basic(ref credentials) => {
                    headers.set(Authorization(Basic {
                        username: credentials.username().to_owned(),
                        password: Some(credentials.password().to_owned()),
                    }))
                },
                Authentication::None => {},
            }
        }
        request
    }
}

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=self::Error>>;

pub trait PreparedRequest<M> {}

impl<M> PreparedRequest<M> for Request {}

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

fn http_method_for_operation(operation: &Operation) -> hyper::Method {
    use self::hyper::Method;
    match *operation {
        Operation::Create => Method::Post,
        Operation::Read => Method::Get,
        Operation::Modify => Method::Patch,
        Operation::Replace => Method::Put,
        Operation::Delete => Method::Delete,
    }
}

fn build_request_uri<P>(datasource: &DataSource, prepare: &P) -> Uri
    where P: Prepare
{
    let mut request_uri = String::new();
    request_uri.push_str(datasource.protocol());
    request_uri.push_str("://");
    request_uri.push_str(datasource.host());
    request_uri.push_str(":");
    request_uri.push_str(&datasource.port().to_string());
    if let Some(database_name) = datasource.database_name() {
        request_uri.push_str("/_db/");
        request_uri.push_str(database_name);
    }
    request_uri.push_str(prepare.path());
    request_uri.push('?');
    for &(ref key, ref value) in prepare.parameters().iter() {
        request_uri.push_str(key);
        request_uri.push('=');
        request_uri.push_str(value);
        request_uri.push('&');
    }
    request_uri.pop();
    Uri::from_str(&request_uri).unwrap()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use api::{Parameters, Prepare};
    use datasource::{Authentication, Credentials};
    use super::*;

    struct Prepared<'a> {
        operation: Operation,
        path: &'a str,
        params: Vec<(&'a str, &'a str)>,
    }

    impl<'a> Prepare for Prepared<'a> {
        fn operation(&self) -> Operation {
            self.operation.clone()
        }

        fn path(&self) -> &str {
            &self.path
        }

        fn parameters(&self) -> Parameters {
            Parameters::from_iter(self.params.iter())
        }
    }

    #[test]
    fn build_request_uri_for_http() {
        let datasource = DataSource::from_url("http://localhost:8529").unwrap();
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/user",
            params: vec![],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("http://localhost:8529/_api/user", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_https_with_authentication() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .with_authentication(Authentication::Basic(
                Credentials::new("micky".to_owned(), "pass".to_owned())));
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/user",
            params: vec![],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_api/user", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("urltest");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/collection",
            params: vec![],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/urltest/_api/collection", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_one_param() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("thebigdata");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25")],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/thebigdata/_api/document\
                ?id=25", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_two_params() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("thebigdata");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25"), ("name", "JuneReport")],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/thebigdata/_api/document\
                ?id=25&name=JuneReport", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_three_params() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("thebigdata");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25"), ("name", "JuneReport"), ("max", "42")],
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/thebigdata/_api/document\
                ?id=25&name=JuneReport&max=42", uri.to_string());
    }

}
