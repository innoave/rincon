
use std::io;
use std::str::FromStr;
use std::string::FromUtf8Error;

use futures::{future, Future, Stream};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper::header::{Authorization, Basic, Bearer, ContentLength, ContentType, UserAgent};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use native_tls;
use serde::ser::Serialize;
use serde_json::{self, Value};
use tokio_core::reactor;
use url;
use url::percent_encoding::DEFAULT_ENCODE_SET;

use api::auth::{Authentication, Credentials, Jwt};
use api::method::{Method, Operation, Prepare, RpcReturnType};
use api::method as api;
use arango::protocol::PATH_DB;
use datasource::DataSource;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (compatible; ArangoDB-RustDriver/1.1)";

pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=Error>>;

#[derive(Debug)]
pub enum Error {
    ApiError(api::Error),
    CommunicationFailed(hyper::Error),
    JsonError(serde_json::Error),
    HttpError(StatusCode),
    IoError(io::Error),
    NativeTlsError(native_tls::Error),
    NotAuthenticated(String),
    NotUtf8Content(FromUtf8Error),
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

impl From<StatusCode> for api::ErrorCode {
    fn from(status_code: StatusCode) -> Self {
        api::ErrorCode::from_u16(status_code.as_u16())
    }
}

#[derive(Debug)]
pub struct Connection {
    datasource: DataSource,
    client: Client<TimeoutConnector<HttpsConnector<HttpConnector>>>,
    user_agent: String,
    token: Option<Jwt>,
}

impl Connection {
    pub fn establish(datasource: DataSource, reactor: &reactor::Handle)
        -> Result<Self, Error>
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
            token: None,
        })
    }

    fn authenticate(&mut self, credentials: &Credentials) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare + 'static
    {
        match self.prepare_request(&method) {
            Ok(request) => {
                debug!("Sending {:?}", &request);
                Box::new(self.client.request(request).from_err()
                    .and_then(move |response| {
                        let status_code = response.status();
                        response.body().concat2().from_err()
                            .and_then(move |buffer|
                                parse_return_type::<M>(&method.return_type(), status_code, &buffer))
                    })
                )
            },
            Err(error) =>
                Box::new(future::err(error)),
        }
    }

    pub fn prepare_request<P>(&self, prepare: &P) -> Result<Request, Error>
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
                Authentication::Jwt(_) => {
                    match self.token.as_ref() {
                        Some(token) => {
                            headers.set(Authorization(Bearer {
                                token: token.to_owned(),
                            }))
                        },
                        None => {
                            return Err(Error::NotAuthenticated(
                                "the client must be authenticated first,\
                                 when using JWT authentication".into()));
                        },
                    }
                },
                Authentication::None => {},
            }
        }
        if let Some(content) = prepare.content() {
            let json = serialize_payload(content)?;
//            trace!("Payload: {:?}", String::from_utf8(json.clone()));
            request.headers_mut().set(ContentType::json());
            request.headers_mut().set(ContentLength(json.len() as u64));
            request.set_body(json);
        }
        Ok(request)
    }

    pub fn datasource(&self) -> &DataSource {
        &self.datasource
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}

fn parse_return_type<M>(return_type: &RpcReturnType, status_code: StatusCode, payload: &[u8])
    -> Result<<M as Method>::Result, Error>
    where M: Method
{
    debug!("Received response with code {:?}", status_code);
    if status_code.is_success() {
        let parse_result = match return_type.result_field {
            Some(result_field) => match serde_json::from_slice(payload) {
                Ok(Value::Object(ref mut obj)) => match obj.remove(result_field) {
                    Some(result_value) =>
                        serde_json::from_value(result_value),
                    None =>
                        serde_json::from_slice(payload),
                },
                _ =>
                    serde_json::from_slice(payload),
            },
            None => serde_json::from_slice(payload),
        };
        if parse_result.is_err() {
            debug!("| response body: {}", String::from_utf8_lossy(payload));
        }
        parse_result.map_err(Error::from)
    } else {
        debug!("| response body: {}", String::from_utf8_lossy(payload));
        let api_error = serde_json::from_slice(payload).unwrap_or_else(|_| {
            let error_code = api::ErrorCode::from(status_code);
            let message = if payload.is_empty() {
                error_code.description().to_owned()
            } else {
                String::from_utf8_lossy(payload).to_string()
            };
            api::Error::new(status_code.as_u16(), error_code, message)
        });
        Err(Error::ApiError(api_error))
    }
}

fn serialize_payload<T>(content: &T) -> Result<Vec<u8>, Error>
    where T: Serialize
{
    serde_json::to_vec(content).map_err(Error::from)
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
    request_uri.push(':');
    request_uri.push_str(&datasource.port().to_string());
    if let Some(database_name) = datasource.database_name() {
        request_uri.push_str(PATH_DB);
        request_uri.push_str(&percent_encode(database_name));
    }
    request_uri.push_str(&percent_encode(&prepare.path()));
    if !prepare.parameters().is_empty() {
        request_uri.push('?');
        for &(ref key, ref value) in prepare.parameters().iter() {
            request_uri.push_str(&percent_encode(key));
            request_uri.push('=');
            request_uri.push_str(&percent_encode(value));
            request_uri.push('&');
        }
        request_uri.pop();
    }
    Uri::from_str(&request_uri).unwrap()
}

fn percent_encode(value: &str) -> String {
    url::percent_encoding::percent_encode(value.as_bytes(), DEFAULT_ENCODE_SET).to_string()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use api::auth::{Authentication, Credentials};
    use api::method::{Parameters, Prepare};
    use super::*;

    struct Prepared<'a> {
        operation: Operation,
        path: &'a str,
        params: Vec<(&'a str, &'a str)>,
        content: Option<Value>
    }

    impl<'a> Prepare for Prepared<'a> {
        type Content = Value;

        fn operation(&self) -> Operation {
            self.operation.clone()
        }

        fn path(&self) -> String {
            String::from(self.path)
        }

        fn parameters(&self) -> Parameters {
            Parameters::from_iter(self.params.iter())
        }

        fn content(&self) -> Option<&Self::Content> {
            self.content.as_ref()
        }
    }

    #[test]
    fn build_request_uri_for_http() {
        let datasource = DataSource::from_url("http://localhost:8529").unwrap();
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/user",
            params: vec![],
            content: None,
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
            content: None,
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_api/user", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("url_test");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/collection",
            params: vec![],
            content: None,
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/url_test/_api/collection", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_one_param() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("the big data");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25")],
            content: None,
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/the%20big%20data/_api/document\
                ?id=25", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_two_params() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("the büg data");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25"), ("name", "JuneReport")],
            content: None,
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/the%20b%C3%BCg%20data/_api/document\
                ?id=25&name=JuneReport", uri.to_string());
    }

    #[test]
    fn build_request_uri_for_specific_database_with_three_params() {
        let datasource = DataSource::from_url("https://localhost:8529").unwrap()
            .use_database("the big data");
        let prepared = Prepared {
            operation: Operation::Read,
            path: "/_api/document",
            params: vec![("id", "25"), ("name", "JuneReport"), ("max", "42")],
            content: None,
        };

        let uri = build_request_uri(&datasource, &prepared);

        assert_eq!("https://localhost:8529/_db/the%20big%20data/_api/document\
                ?id=25&name=JuneReport&max=42", uri.to_string());
    }

}
