
use std::str::FromStr;

use futures::{future, Future, Stream};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper::client::HttpConnector;
use hyper::header::{self, Authorization, Basic, Bearer, ContentLength, ContentType};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use serde::ser::Serialize;
use serde_json::{self, Value};
use tokio_core::reactor;
use url;
use url::percent_encoding::DEFAULT_ENCODE_SET;

use rincon_core::api;
use rincon_core::api::auth::{Authentication, Credentials, Jwt};
use rincon_core::api::connector::{Error, Execute, FutureResult};
use rincon_core::api::datasource::UseDatabase;
use rincon_core::api::method::{Method, Operation, Prepare, RpcReturnType};
use rincon_core::api::user_agent::UserAgent;
use rincon_core::arango::protocol::PATH_DB;
use datasource::DataSource;

//pub type FutureResult<M> = Box<Future<Item=<M as Method>::Result, Error=Error>>;

#[derive(Debug)]
pub struct Connection {
    user_agent: &'static UserAgent,
    datasource: DataSource,
    client: Client<HttpsConnector<HttpConnector>>,
    token: Option<Jwt>,
}

impl Connection {
    pub fn establish(user_agent: &'static UserAgent, datasource: DataSource, reactor: &reactor::Handle)
        -> Result<Self, Error>
    {
        let https_connector = HttpsConnector::new(4, &reactor)
            .map_err(|cause| Error::Communication(cause.to_string()))?;
//        let mut timeout_connector = TimeoutConnector::new(https_connector, &reactor);
//        timeout_connector.set_connect_timeout(Some(*datasource.timeout()));
        let client = Client::configure()
            .connector(https_connector)
            .build(reactor);
        debug!("Created connection for {:?}", &datasource);
        Ok(Connection {
            datasource,
            client,
            user_agent,
            token: None,
        })
    }

    fn authenticate(&mut self, credentials: &Credentials) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn prepare_request<'p, P>(&self, prepare: &'p P) -> Result<Request, Error>
        where P: 'p + Prepare
    {
        let operation = prepare.operation();
        let http_method = http_method_for_operation(&operation);
        let uri = build_request_uri(&self.datasource, prepare);
        let mut request = Request::new(http_method, uri);
        request.set_version(HttpVersion::Http11);
        {
            let headers = request.headers_mut();
            headers.set(header_user_agent_for(self.user_agent));
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
                                "the client must be authenticated first, \
                                 when using JWT authentication".into(),
                            ));
                        },
                    }
                },
                Authentication::None => {},
            }
            for &(ref name, ref value) in prepare.header().iter() {
                headers.set_raw(name.to_string(), value.to_string());
            }
        }
        if let Some(content) = prepare.content() {
            let json = serialize_payload(content)?;
            trace!("| request body: {:?}", String::from_utf8(json.clone()));
            request.headers_mut().set(ContentType::json());
            request.headers_mut().set(ContentLength(json.len() as u64));
            request.set_body(json);
        }
        Ok(request)
    }

    pub fn datasource(&self) -> &DataSource {
        &self.datasource
    }

    pub fn user_agent(&self) -> &UserAgent {
        self.user_agent
    }
}

impl UseDatabase for Connection {
    fn use_database<DbName>(&self, database_name: DbName) -> Self
        where DbName: Into<String>
    {
        Connection {
            user_agent: self.user_agent.clone(),
            datasource: self.datasource.use_database(database_name),
            client: self.client.clone(),
            token: self.token.clone(),
        }
    }

    fn use_default_database(&self) -> Self {
        Connection {
            user_agent: self.user_agent.clone(),
            datasource: self.datasource.use_default_database(),
            client: self.client.clone(),
            token: self.token.clone(),
        }
    }

    fn database_name(&self) -> Option<&String> {
        self.datasource.database_name()
    }
}

impl Execute for Connection {
    fn execute<M>(&self, method: M) -> FutureResult<M>
        where M: Method + Prepare + 'static
    {
        match self.prepare_request(&method) {
            Ok(request) => {
                debug!("Sending {:?}", &request);
                Box::new(self.client.request(request)
                    .map_err(|cause| Error::Communication(cause.to_string()))
                    .and_then(move |response| {
                        let status_code = response.status();
                        response.body().concat2()
                            .map_err(|cause| Error::Communication(cause.to_string()))
                            .and_then(move |buffer|
                                parse_return_type::<M>(&method.return_type(), status_code, &buffer))
                    })
                )
            },
            Err(error) =>
                Box::new(future::err(error)),
        }
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
        } else {
            trace!("| response body: {}", String::from_utf8_lossy(payload));
        }
        parse_result.map_err(|cause| Error::Deserialization(cause.to_string()))
    } else {
        debug!("| response body: {}", String::from_utf8_lossy(payload));
        let api_error = serde_json::from_slice(payload).unwrap_or_else(|_| {
            let error_code = api::ErrorCode::from_u16(status_code.as_u16());
            let message = if payload.is_empty() {
                error_code.description().to_owned()
            } else {
                String::from_utf8_lossy(payload).to_string()
            };
            api::Error::new(status_code.as_u16(), error_code, message)
        });
        Err(Error::Method(api_error))
    }
}

fn serialize_payload<T>(content: &T) -> Result<Vec<u8>, Error>
    where T: Serialize
{
    serde_json::to_vec(content).map_err(|cause| Error::Serialization(cause.to_string()))
}

fn header_user_agent_for(agent: &UserAgent) -> header::UserAgent {
    let agent_string = format!("Mozilla/5.0 (compatible; {}/{}.{}; +{})",
        agent.name(), agent.version().major(), agent.version().minor(), agent.homepage());
    header::UserAgent::new(agent_string)
}

fn http_method_for_operation(operation: &Operation) -> hyper::Method {
    use self::hyper::Method;
    match *operation {
        Operation::Create => Method::Post,
        Operation::Read => Method::Get,
        Operation::Modify => Method::Patch,
        Operation::Replace => Method::Put,
        Operation::Delete => Method::Delete,
        Operation::ReadHeader => Method::Head,
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
            request_uri.push_str(&percent_encode(&value.to_string()));
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

    use rincon_core::api::auth::{Authentication, Credentials};
    use rincon_core::api::method::{Parameters, Prepare};
    use rincon_core::api::user_agent::Version;
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

        fn header(&self) -> Parameters {
            Parameters::empty()
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

    #[test]
    fn header_user_agent_for_my_user_agent() {
        #[derive(Debug)]
        struct MyUserAgent;
        #[derive(Debug)]
        struct MyVersion;

        impl UserAgent for MyUserAgent {
            fn name(&self) -> &str {
                "rincon"
            }

            fn version(&self) -> &Version {
                &MyVersion
            }

            fn homepage(&self) -> &str {
                "https://github.com/innoave/rincon"
            }
        }

        impl Version for MyVersion {
            fn major(&self) -> &str {
                "2"
            }

            fn minor(&self) -> &str {
                "5"
            }

            fn patch(&self) -> &str {
                "9"
            }

            fn pre(&self) -> &str {
                ""
            }
        }

        let agent = header_user_agent_for(&MyUserAgent);

        assert_eq!(
            header::UserAgent::new("Mozilla/5.0 (compatible; rincon/2.5; +https://github.com/innoave/rincon)"),
            agent
        );
    }

}
