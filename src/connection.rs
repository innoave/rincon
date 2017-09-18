
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;

use futures::{Future, Stream};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic, Bearer, UserAgent};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use native_tls;
use serde_json::{self, Value};
use tokio_core::reactor;

use api::{self, Method, Operation, Prepare, RpcErrorType};
use datasource::{Authentication, DataSource};

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; ArangoDB-RustDriver/1.1)";

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=self::Error>>;

#[derive(Debug)]
pub enum Error {
    ApiError(api::ErrorCode),
    CommunicationFailed(hyper::Error),
    JsonError(serde_json::Error),
    HttpError(hyper::StatusCode),
    IoError(io::Error),
    NativeTlsError(native_tls::Error),
    NotUtf8Content(FromUtf8Error),
}

impl From<api::ErrorCode> for Error {
    fn from(code: api::ErrorCode) -> Self {
        Error::ApiError(code)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::NotUtf8Content(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
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

impl From<StatusCode> for Error {
    fn from(status_code: StatusCode) -> Self {
        Error::HttpError(status_code)
    }
}

#[derive(Debug)]
pub struct Connection {
    datasource: DataSource,
    client: Client<TimeoutConnector<HttpsConnector<HttpConnector>>>,
    user_agent: String,
}

impl Connection {
    pub fn establish(datasource: DataSource, reactor: &reactor::Handle)
        -> Result<Self, self::Error>
    {
        let https_connector = HttpsConnector::new(4, &reactor)?;
        let mut timeout_connector = TimeoutConnector::new(https_connector, &reactor);
        timeout_connector.set_connect_timeout(Some(*datasource.timeout()));
        let client = Client::configure()
            .connector(timeout_connector)
            .build(reactor);
        debug!("Created connection for {:?}", &datasource);
        Ok(Connection {
            datasource,
            client,
            user_agent: DEFAULT_USER_AGENT.to_owned(),
        })
    }

    pub fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare + 'static
    {
        let request = self.prepare_request(&method);
        debug!("Sending {:?}", &request);
        Box::new(self.client.request(request).from_err()
            .and_then(move |response| {
                let status_code = response.status();
                response.body().concat2().from_err()
                    .and_then(move |buffer|
                        parse_return_type::<M>(&method.error_type(), status_code, &buffer))
            })
        )
    }

    pub fn prepare_request<P>(&self, prepare: &P) -> Request
        where P: Prepare
    {
        let operation = prepare.operation();
        let http_method = http_method_for_operation(&operation);
        let uri = build_request_uri(&self.datasource, prepare);
        let mut request = Request::new(http_method, uri);
        request.set_version(HttpVersion::Http11);
        {
            let headers = request.headers_mut();
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
    if !prepare.parameters().is_empty() {
        request_uri.push('?');
        for &(ref key, ref value) in prepare.parameters().iter() {
            request_uri.push_str(key);
            request_uri.push('=');
            request_uri.push_str(value);
            request_uri.push('&');
        }
        request_uri.pop();
    }
    Uri::from_str(&request_uri).unwrap()
}

fn parse_return_type<M>(error_type: &RpcErrorType, status_code: StatusCode, payload: &[u8])
    -> Result<<M as Method>::Result, Error>
    where M: Method
{
    let mut payload_value = serde_json::from_slice(payload)?;
    if status_code.is_success() {
        let result_field = match payload_value {
            Value::Object(ref mut obj) =>
                error_type.result_field.and_then(|result| obj.remove(result)),
            _ => None,
        };
        let result_value = result_field.unwrap_or(payload_value);
        serde_json::from_value(result_value).map_err(Error::from)
    } else {
        let code_field = match payload_value {
            Value::Object(ref mut obj) =>
                error_type.code_field.and_then(|code| obj.remove(code)),
            _ => None,
        };
        if let Some(code_value) = code_field {
            serde_json::from_value(code_value).map_err(Error::from)
                .and_then(|code| Err(Error::from(api::ErrorCode::from_u16(code))))
        } else {
            Err(Error::from(status_code))
        }
    }
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
