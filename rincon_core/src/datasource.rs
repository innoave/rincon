
use std::env;
use std::time::Duration;

use url::Url;

use api::auth::{Authentication, Credentials};
use api::connector::UseDatabase;

pub const DEFAULT_PROTOCOL: &str = "http";
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 8529;
pub const DEFAULT_USERNAME: &str = "root";
pub const DEFAULT_PASSWORD: &str = "";
pub const DEFAULT_DATABASE_NAME: &str = "_system";
pub const DEFAULT_TIMEOUT: u64 = 30;

pub const ENV_ROOT_PASSWORD: &str = "ARANGO_ROOT_PASSWORD";

#[derive(Clone, PartialEq, Eq, Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid URL: {}", _0)]
    InvalidUrl(String),
}

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
    pub fn from_url(url: &str) -> Result<Self, Error> {
        let url = Url::parse(url).map_err(|cause| Error::InvalidUrl(cause.to_string()))?;
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
        let authentication = Authentication::Basic(
            Credentials::new(username.to_owned(), password.to_owned())
        );
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            authentication,
            database_name: self.database_name.clone(),
            timeout: self.timeout.clone(),
        }
    }

    pub fn with_authentication(&self, authentication: Authentication) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            authentication,
            database_name: self.database_name.clone(),
            timeout: self.timeout.clone(),
        }
    }

    pub fn without_authentication(&self) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            authentication: Authentication::None,
            database_name: self.database_name.clone(),
            timeout: self.timeout.clone(),
        }
    }

    pub fn with_timeout<D>(&self, timeout: D) -> Self
        where D: Into<Duration>
    {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            authentication: self.authentication.clone(),
            database_name: self.database_name.clone(),
            timeout: timeout.into(),
        }
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

    pub fn timeout(&self) -> &Duration {
        &self.timeout
    }
}

impl UseDatabase for DataSource {
    fn use_database<DbName>(&self, database_name: DbName) -> Self
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
            port: self.port.clone(),
            authentication: self.authentication.clone(),
            database_name,
            timeout: self.timeout.clone(),
        }
    }

    fn use_default_database(&self) -> Self {
        DataSource {
            protocol: self.protocol.clone(),
            host: self.host.clone(),
            port: self.port.clone(),
            authentication: self.authentication.clone(),
            database_name: None,
            timeout: self.timeout.clone(),
        }
    }

    fn database_name(&self) -> Option<&String> {
        self.database_name.as_ref()
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
