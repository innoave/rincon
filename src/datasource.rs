
use url::{ParseError, Url};

pub const DEFAULT_PROTOCOL: &str = "https";
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 8529;
pub const DEFAULT_USERNAME: &str = "root";
pub const DEFAULT_PASSWORD: &str = "";
pub const DEFAULT_DATABASE_NAME: &str = "_system";

#[derive(Debug)]
pub struct DataSource {
    protocol: String,
    host: String,
    port: u16,
    credentials: Credentials,
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
            port: port,
            credentials: Credentials::Basic(
                username.to_string(),
                password.to_string()),
            use_explicit_database: false,
            database_name: database_name.to_string(),
        })
    }

    pub fn use_database(&mut self, database_name: &str) {
        self.database_name = database_name.to_string();
        self.use_explicit_database = true;
    }

    pub fn use_default_database(&mut self) {
        self.use_explicit_database = false;
    }

    pub fn use_basic_auth(&mut self, username: &str, password: &str) {
        self.credentials = Credentials::Basic(
            username.to_string(),
            password.to_string());
    }

    pub fn without_auth(&mut self) {
        self.credentials = Credentials::None;
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

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
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
            credentials: Credentials::Basic(
                DEFAULT_USERNAME.to_string(),
                DEFAULT_PASSWORD.to_string()),
            use_explicit_database: false,
            database_name: DEFAULT_DATABASE_NAME.to_string(),
        }
    }
}

//TODO support JWT authentication
#[derive(Debug)]
pub enum Credentials {
    Basic(String, String),
    None,
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
