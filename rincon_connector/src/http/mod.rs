//! Connectors that use HTTP/HTTPS as transport protocol.
//!
//! This module provides `Connector` implementation that use HTTP or HTTPS as
//! the transport protocol.
//!
//! The currently provided `Connector`s are:
//!
//! * `JsonHttpConnector` : uses JSON over HTTP/HTTPS
//!
//! For an example on how to use a connector see the crate level documentation.

#[cfg(test)]
mod tests;

use std::str::FromStr;
use std::sync::Arc;

use futures::{future, Future, Stream};
use hyper::client::HttpConnector;
use hyper::header::{self, Authorization, Basic, Bearer, ContentLength, ContentType};
use hyper::{self, Client, HttpVersion, Request, StatusCode, Uri};
use hyper_timeout::TimeoutConnector;
use hyper_tls::HttpsConnector;
use serde::ser::Serialize;
use serde_json::{self, Value};
use tokio_core::reactor;
use url;
use url::percent_encoding::DEFAULT_ENCODE_SET;

use rincon_core::api;
use rincon_core::api::auth::{Authentication, Jwt};
use rincon_core::api::connector::{Connector, Error, Execute, FutureResult};
use rincon_core::api::datasource::DataSource;
use rincon_core::api::method::{Method, Operation, Prepare, RpcReturnType};
use rincon_core::api::user_agent::{RinconUserAgent, UserAgent};
use rincon_core::arango::protocol::{PATH_DB, SYSTEM_DATABASE};

type HttpClient = Client<TimeoutConnector<HttpsConnector<HttpConnector>>>;

/// A connector that uses JSON over HTTP/HTTPS.
///
/// This `Connector` implementation uses JSON for serializing the payload and
/// HTTP or HTTPS as the transport protocol.
///
/// This connector supports both authentication methods of the [ArangoDB] REST
/// API: Json Web Token (JWT) and basic authentication. The authentication
/// method to be used is specified in the `DataSource`. If `Authentication:Jwt`
/// is specified but not JWT is set in this connection method calls will return
/// an `Error::NotAuthenticated` error.
///
/// For an example on how to use it see the crate level documentation.
#[derive(Debug)]
pub struct JsonHttpConnector {
    user_agent: &'static UserAgent,
    datasource: Arc<DataSource>,
    token: Arc<Option<Jwt>>,
    client: Arc<HttpClient>,
}

impl JsonHttpConnector {
    /// Creates a new instance of the `JsonHttpConnector`.
    ///
    /// The returned `JsonHttpConnector` sets the User-Agent header field of the
    /// HTTP protocol to information about the rincon driver. If you want to use
    /// information about your application as the User-Agent use the alternative
    /// `with_user_agent()` method to create the `JsonHttpConnector` instance.
    ///
    /// # Arguments
    ///
    /// * `datasource` : a `DataSource` that holds the connection parameters
    ///   used to connect to the database server.
    /// * `reactor` : a handle of a `reactor::Core` instance of the `tokio-core`
    ///   crate.
    pub fn new(datasource: DataSource, reactor: &reactor::Handle) -> Result<Self, Error> {
        let https_connector = HttpsConnector::new(4, reactor)
            .map_err(|cause| Error::Communication(cause.to_string()))?;
        let mut timeout_connector = TimeoutConnector::new(https_connector, reactor);
        timeout_connector.set_connect_timeout(Some(*datasource.timeout()));
        let client = Client::configure()
            .connector(timeout_connector)
            .build(reactor);
        debug!("Creating new JSON/HTTP connector for {:?}", &datasource);
        Ok(JsonHttpConnector {
            user_agent: &RinconUserAgent,
            datasource: Arc::new(datasource),
            token: Arc::new(None),
            client: Arc::new(client),
        })
    }

    /// Creates a new instance of the `JsonHttpConnector` that uses the given
    /// user agent.
    ///
    /// The `user_agent` is used to set the User-Agent header field of the HTTP
    /// protocol on each request sent by this connector.
    ///
    /// # Arguments
    ///
    /// * `user_agent` : a `UserAgent` used to set the User-Agent header field
    ///   of the HTTP protocol
    /// * `datasource` : a `DataSource` that holds the connection parameters
    ///   used to connect to the database server.
    /// * `reactor` : a handle of a `reactor::Core` instance of the `tokio-core`
    ///   crate.
    pub fn with_user_agent(
        user_agent: &'static UserAgent,
        datasource: DataSource,
        reactor: &reactor::Handle,
    ) -> Result<Self, Error> {
        let https_connector = HttpsConnector::new(4, reactor)
            .map_err(|cause| Error::Communication(cause.to_string()))?;
        let mut timeout_connector = TimeoutConnector::new(https_connector, reactor);
        timeout_connector.set_connect_timeout(Some(*datasource.timeout()));
        let client = Client::configure()
            .connector(timeout_connector)
            .build(reactor);
        debug!("Creating new JSON/HTTP connector for {:?}", &datasource);
        Ok(JsonHttpConnector {
            user_agent,
            datasource: Arc::new(datasource),
            token: Arc::new(None),
            client: Arc::new(client),
        })
    }
}

impl Connector for JsonHttpConnector {
    type Connection = JsonHttpConnection;

    fn connection(&self, database_name: &str) -> JsonHttpConnection {
        JsonHttpConnection {
            user_agent: self.user_agent,
            datasource: self.datasource.clone(),
            database: Some(database_name.to_owned()),
            token: self.token.clone(),
            client: self.client.clone(),
        }
    }

    fn system_connection(&self) -> JsonHttpConnection {
        self.connection(SYSTEM_DATABASE)
    }

    fn use_auth_token(&mut self, token: Jwt) {
        self.token = Arc::new(Some(token));
    }

    fn invalidate_auth_token(&mut self) {
        self.token = Arc::new(None)
    }
}

/// A connection to a server that actually executes method calls using JSON
/// over HTTP/HTTPS.
#[derive(Debug)]
pub struct JsonHttpConnection {
    user_agent: &'static UserAgent,
    datasource: Arc<DataSource>,
    database: Option<String>,
    token: Arc<Option<Jwt>>,
    client: Arc<HttpClient>,
}

impl JsonHttpConnection {
    /// Returns the `UserAgent` used for the HTTP User-Agent header field.
    pub fn user_agent(&self) -> &UserAgent {
        self.user_agent
    }

    /// Returns the `DataSource` used by the connection.
    pub fn datasource(&self) -> &DataSource {
        &self.datasource
    }

    /// Returns the name of the default database addressed by method calls
    /// if none is specified by the actual method.
    ///
    /// If no default database is set this method returns `None`. In this case
    /// calls are executed against the system database if the actual method call
    /// does not specify a database.
    pub fn database(&self) -> Option<&String> {
        self.database
            .as_ref()
            .or_else(|| self.datasource.database_name())
    }

    /// Returns the authentication token used by this connection.
    pub fn token(&self) -> Option<&Jwt> {
        self.token.as_ref().as_ref()
    }

    /// Builds a HTTP-request for a concrete method call and returns it.
    pub fn prepare_request<'p, P>(&self, method: &'p P) -> Result<Request, Error>
    where
        P: 'p + Prepare,
    {
        let operation = method.operation();
        let http_method = http_method_for_operation(&operation);
        let uri = build_request_uri(&self.datasource, self.database(), method);
        let mut request = Request::new(http_method, uri);
        request.set_version(HttpVersion::Http11);
        {
            let headers = request.headers_mut();
            headers.set(header_user_agent_for(self.user_agent));
            match *self.datasource.authentication() {
                Authentication::Basic(ref credentials) => headers.set(Authorization(Basic {
                    username: credentials.username().to_owned(),
                    password: Some(credentials.password().to_owned()),
                })),
                Authentication::Jwt(_) => match *self.token.as_ref() {
                    Some(ref token) => headers.set(Authorization(Bearer {
                        token: token.to_owned(),
                    })),
                    None => {
                        return Err(Error::NotAuthenticated(
                            "the client must be authenticated first, \
                             when using JWT authentication"
                                .into(),
                        ));
                    },
                },
                Authentication::None => {},
            }
            for &(ref name, ref value) in method.header().iter() {
                headers.set_raw(name.to_string(), value.to_string());
            }
        }
        if let Some(content) = method.content() {
            let json = serialize_payload(content)?;
            trace!("| request body: {:?}", String::from_utf8(json.clone()));
            request.headers_mut().set(ContentType::json());
            request.headers_mut().set(ContentLength(json.len() as u64));
            request.set_body(json);
        }
        Ok(request)
    }
}

impl Execute for JsonHttpConnection {
    fn execute<M>(&self, method: M) -> FutureResult<M>
    where
        M: Method + Prepare + 'static,
    {
        match self.prepare_request(&method) {
            Ok(request) => {
                debug!("Sending {:?}", &request);
                Box::new(
                    self.client
                        .request(request)
                        .map_err(|cause| Error::Communication(cause.to_string()))
                        .and_then(move |response| {
                            let status_code = response.status();
                            response
                                .body()
                                .concat2()
                                .map_err(|cause| Error::Communication(cause.to_string()))
                                .and_then(move |buffer| {
                                    parse_return_type::<M>(
                                        &method.return_type(),
                                        status_code,
                                        &buffer,
                                    )
                                })
                        }),
                )
            },
            Err(error) => Box::new(future::err(error)),
        }
    }
}

fn parse_return_type<M>(
    return_type: &RpcReturnType,
    status_code: StatusCode,
    payload: &[u8],
) -> Result<<M as Method>::Result, Error>
where
    M: Method,
{
    debug!("Received response with code {:?}", status_code);
    if status_code.is_success() {
        let parse_result = match return_type.result_field {
            Some(result_field) => match serde_json::from_slice(payload) {
                Ok(Value::Object(ref mut obj)) => match obj.remove(result_field) {
                    Some(result_value) => serde_json::from_value(result_value),
                    None => serde_json::from_slice(payload),
                },
                _ => serde_json::from_slice(payload),
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
where
    T: Serialize,
{
    serde_json::to_vec(content).map_err(|cause| Error::Serialization(cause.to_string()))
}

fn header_user_agent_for(agent: &UserAgent) -> header::UserAgent {
    let agent_string = format!(
        "Mozilla/5.0 (compatible; {}/{}.{}; +{})",
        agent.name(),
        agent.version().major(),
        agent.version().minor(),
        agent.homepage()
    );
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

fn build_request_uri<P>(datasource: &DataSource, database_name: Option<&String>, prepare: &P) -> Uri
where
    P: Prepare,
{
    let mut request_uri = String::new();
    request_uri.push_str(datasource.protocol());
    request_uri.push_str("://");
    request_uri.push_str(datasource.host());
    request_uri.push(':');
    request_uri.push_str(&datasource.port().to_string());
    if let Some(database_name) = database_name {
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
