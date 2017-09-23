
use std::env;
use std::time::Duration;

use url::{ParseError, Url};

pub const DEFAULT_PROTOCOL: &str = "http";
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 8529;
pub const DEFAULT_USERNAME: &str = "root";
pub const DEFAULT_PASSWORD: &str = "";
pub const DEFAULT_DATABASE_NAME: &str = "_system";
pub const DEFAULT_TIMEOUT: u64 = 30;

pub const ENV_ROOT_PASSWORD: &str = "ARANGO_ROOT_PASSWORD";

#[derive(Clone, Debug)]
pub struct DataSource {
    protocol: String,
    host: String,
    port: u16,
    authentication: Authentication,
    database_name: Option<String>,
    timeout: Duration,
}

impl DataSource {
    pub fn from_url(url: &str) -> Result<Self, self::Error> {
        let url = Url::parse(url)?;
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
        Ok(DataSource {
            protocol: protocol.to_owned(),
            host: host.to_owned(),
            port,
            authentication: Authentication::Basic(Credentials::new(
                username.to_owned(),
                password)),
            database_name,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        })
    }

    pub fn with_basic_authentication(&self, username: &str, password: &str) -> Self {
        let mut ds = self.clone();
        ds.authentication = Authentication::Basic(Credentials::new(
            username.to_owned(),
            password.to_owned()));
        ds
    }

    pub fn without_authentication(&self) -> Self {
        let mut ds = self.clone();
        ds.authentication = Authentication::None;
        ds
    }

    pub fn with_authentication(&self, authentication: Authentication) -> Self {
        let mut ds = self.clone();
        ds.authentication = authentication;
        ds
    }

    pub fn use_database(&self, database_name: &str) -> Self {
        let mut ds = self.clone();
        ds.database_name = if database_name.is_empty() {
            None
        } else {
            Some(database_name.to_owned())
        };
        ds
    }

    pub fn use_default_database(&self) -> Self {
        let mut ds = self.clone();
        ds.database_name = None;
        ds
    }

    pub fn with_timeout(&self, timeout: Duration) -> Self {
        let mut ds = self.clone();
        ds.timeout = timeout;
        ds
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn authentication(&self) -> &Authentication {
        &self.authentication
    }

    pub fn database_name(&self) -> Option<&String> {
        self.database_name.as_ref()
    }

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

//TODO support JWT authentication
#[derive(Clone, Debug)]
pub enum Authentication {
    Basic(Credentials),
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new<S>(username: S, password: S) -> Self
        where S: Into<String>
    {
        Credentials {
            username: username.into(),
            password: password.into(),
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidUrl(ParseError),
}

impl From<ParseError> for Error {
    fn from(parse_error: ParseError) -> Self {
        Error::InvalidUrl(parse_error)
    }
}
