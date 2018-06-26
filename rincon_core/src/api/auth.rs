//! Type definitions for various authentication methods used by the rincon
//! driver.

/// This enum defines the supported authentication methods.
#[derive(Debug, Clone)]
pub enum Authentication {
    /// Basic authentication.
    Basic(Credentials),
    /// Authentication via JSON Web Token (JWT).
    Jwt(Credentials),
    /// No authentication.
    None,
}

/// This struct holds the credentials needed to authenticate a user.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Credentials {
    /// The username registered for a user.
    username: String,
    /// The password registered with a users username.
    password: String,
}

impl Credentials {
    /// Constructs new `Credentials` with the given username and password.
    pub fn new<S>(username: S, password: S) -> Self
    where
        S: Into<String>,
    {
        Credentials {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns the username of this `Credentials`.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the password of this `Credentials`.
    pub fn password(&self) -> &str {
        &self.password
    }
}

/// Type definition for a JSON Web Token (JWT).
pub type Jwt = String;
