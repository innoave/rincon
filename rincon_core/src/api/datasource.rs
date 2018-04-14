//! A datasource defines parameters for establishing a connection to a server.

use std::env;
use std::str::FromStr;
use std::time::Duration;

use api::auth::{Authentication, Credentials};
use api::types::Url;

/// The default transport protocol to be used
pub const DEFAULT_PROTOCOL: &str = "http";
/// The default host name to be used
pub const DEFAULT_HOST: &str = "localhost";
/// The default port number to be used
pub const DEFAULT_PORT: u16 = 8529;
/// The default username to be used
pub const DEFAULT_USERNAME: &str = "root";
/// The default password to be used
pub const DEFAULT_PASSWORD: &str = "";
/// The default system database to be used
pub const DEFAULT_DATABASE_NAME: &str = "_system";
/// The default timeout to be used during method call execution
pub const DEFAULT_TIMEOUT: u64 = 30;

/// The name of the system environment variable that contains the root password
pub const ENV_ROOT_PASSWORD: &str = "ARANGO_ROOT_PASSWORD";

/// An error that may occur when dealing with datasources.
///
/// Currently this error is returned when an URL string can not be parsed.
#[derive(Clone, PartialEq, Eq, Debug, Fail)]
pub enum Error {
    /// The given URL is not valid
    #[fail(display = "Invalid URL: {}", _0)]
    InvalidUrl(String),
}

/// Holds the parameters for establishing connections to a server.
#[derive(Debug, Clone)]
pub struct DataSource {
    protocol: String,
    host: String,
    port: u16,
    database_name: Option<String>,
    authentication: Authentication,
    timeout: Duration,
}

impl DataSource {
    /// Creates a new `DataSource` with the parameters taken from the given URL.
    pub fn from_url(url: &Url) -> Self {
        let protocol = url.scheme();
        let host = url.host_str().unwrap_or(DEFAULT_HOST);
        let port = url.port().unwrap_or(DEFAULT_PORT);
        let username = if url.username().is_empty() {
            DEFAULT_USERNAME
        } else {
            url.username()
        };
        let password = if let Some(passwd) = url.password() {
            passwd.to_owned()
        } else if let Ok(passwd) = env::var(ENV_ROOT_PASSWORD) {
            passwd
        } else {
            DEFAULT_PASSWORD.to_owned()
        };
        //TODO parse database name from url path segments
        let database_name = None;
        DataSource {
            protocol: protocol.to_owned(),
            host: host.to_owned(),
            port,
            database_name,
            authentication: Authentication::Basic(Credentials::new(
                username.to_owned(),
                password)),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }

    /// Returns a new copy of this `DataSource` with the database parameter set
    /// to the given database name.
    pub fn use_database<DbName>(&self, database_name: DbName) -> Self
        where DbName: Into<String>
    {
        let database_name = database_name.into();
        let database_name = if database_name.is_empty() {
            None
        } else {
            Some(database_name.to_owned())
        };
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            database_name,
            authentication: self.authentication.clone(),
            timeout: self.timeout,
        }
    }

    /// Returns a new copy of this `DataSource` with the database parameter set
    /// to `None` which means that the default database of the authenticated
    /// user will be used.
    pub fn use_default_database(&self) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            database_name: None,
            authentication: self.authentication.clone(),
            timeout: self.timeout,
        }
    }

    /// Returns a new copy of this `DataSource` which uses basic authentication
    /// with the given username and password.
    pub fn with_basic_authentication(&self, username: &str, password: &str) -> Self {
        let authentication = Authentication::Basic(
            Credentials::new(username.to_owned(), password.to_owned())
        );
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            database_name: self.database_name.clone(),
            authentication,
            timeout: self.timeout,
        }
    }

    /// Returns a new copy of this `DataSource` with the authentication
    /// parameter set to the given authentication method.
    pub fn with_authentication(&self, authentication: Authentication) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            database_name: self.database_name.clone(),
            authentication,
            timeout: self.timeout,
        }
    }

    /// Returns a new copy of this `DataSource` that does not use any
    /// authentication at all.
    pub fn without_authentication(&self) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            database_name: self.database_name.clone(),
            authentication: Authentication::None,
            timeout: self.timeout,
        }
    }

    /// Returns a new copy of this `DataSource` but with the timeout for method
    /// calls set to the given value.
    pub fn with_timeout<D>(&self, timeout: D) -> Self
        where D: Into<Duration>
    {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port,
            authentication: self.authentication.clone(),
            database_name: self.database_name.clone(),
            timeout: timeout.into(),
        }
    }

    /// Returns the protocol defined for this `DataSource`.
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    /// Returns the host name defined for this `DataSource`.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the port number defined for this `DataSource`.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the name of the database defined for this `DataSource`.
    pub fn database_name(&self) -> Option<&String> {
        self.database_name.as_ref()
    }

    /// Returns the authentication method defined for this `DataSource`.
    pub fn authentication(&self) -> &Authentication {
        &self.authentication
    }

    /// Returns the timeout for method calls defined for this `DataSource`.
    pub fn timeout(&self) -> &Duration {
        &self.timeout
    }
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource {
            protocol: DEFAULT_PROTOCOL.to_owned(),
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
            authentication: Authentication::Basic(Credentials::new(
                DEFAULT_USERNAME.to_owned(),
                DEFAULT_PASSWORD.to_owned())),
            database_name: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }
}

/// Parses an URL string for the parameters needed to create a `DataSource`.
impl FromStr for DataSource {
    type Err = Error;

    fn from_str(url_str: &str) -> Result<Self, <Self as FromStr>::Err> {
        Url::parse(url_str)
            .map_err(|cause| Error::InvalidUrl(cause.to_string()))
            .map(|url| DataSource::from_url(&url))
    }
}
