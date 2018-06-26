//! Types used in methods for user authentication.

use rincon_core::api::auth::Jwt;

/// This structs holds the properties necessary to authenticate a user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationRequest {
    username: String,
    password: String,
}

impl AuthenticationRequest {
    /// Constructs a new instance of an `AuthenticationRequest` containing the
    /// given username and password.
    pub fn new<N, P>(username: N, password: P) -> Self
    where
        N: Into<String>,
        P: Into<String>,
    {
        AuthenticationRequest {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns the username in this `AuthenticationRequest`.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the password in this `AuthenticationRequest`.
    pub fn password(&self) -> &str {
        &self.password
    }
}

/// This struct holds the result of a successful authentication.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    jwt: Jwt,
    must_change_password: Option<bool>,
}

impl AuthenticationResponse {
    /// Returns the JSON Web Token (JWT).
    pub fn jwt(&self) -> &Jwt {
        &self.jwt
    }

    /// Returns whether the password must be changed.
    pub fn is_must_change_password(&self) -> Option<bool> {
        self.must_change_password
    }
}
