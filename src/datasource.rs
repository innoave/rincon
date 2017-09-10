
use url::{ParseError, Url};

pub const DEFAULT_PROTOCOL: &str = "https";
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 8529;
pub const DEFAULT_USERNAME: &str = "root";
pub const DEFAULT_PASSWORD: &str = "";
pub const DEFAULT_DATABASE_NAME: &str = "_system";

#[derive(Clone, Debug)]
pub struct DataSource {
    protocol: String,
    host: String,
    port: u16,
    authentication: Authentication,
    use_explicit_database: bool,
    database_name: String,
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
        let password = url.password().unwrap_or(DEFAULT_PASSWORD);
        //TODO parse database name from url path segments
        let database_name = DEFAULT_DATABASE_NAME;
        Ok(DataSource {
            protocol: protocol.to_string(),
            host: host.to_string(),
            port,
            authentication: Authentication::Basic(Credentials::new(
                username.to_string(),
                password.to_string())),
            use_explicit_database: false,
            database_name: database_name.to_string(),
        })
    }

    pub fn use_database(&self, database_name: &str) -> Self {
        let mut ds = self.clone();
        ds.database_name = database_name.to_string();
        ds.use_explicit_database = true;
        ds
    }

    pub fn use_default_database(&self) -> Self {
        let mut ds = self.clone();
        ds.use_explicit_database = false;
        ds
    }

    pub fn with_basic_authentication(&self, username: &str, password: &str) -> Self {
        let mut ds = self.clone();
        ds.authentication = Authentication::Basic(Credentials::new(
            username.to_string(),
            password.to_string()));
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

    pub fn is_use_explicit_database(&self) -> bool {
        self.use_explicit_database
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource {
            protocol: DEFAULT_PROTOCOL.to_string(),
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            authentication: Authentication::Basic(Credentials::new(
                DEFAULT_USERNAME.to_string(),
                DEFAULT_PASSWORD.to_string())),
            use_explicit_database: false,
            database_name: DEFAULT_DATABASE_NAME.to_string(),
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
    pub fn new(username: String, password: String) -> Self {
        Credentials {
            username,
            password,
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
