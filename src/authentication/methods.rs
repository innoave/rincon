
use api::{Credentials, Method, Parameters, Operation, Prepare, RpcReturnType};
use super::types::*;

/// Authenticates a user.
#[derive(Clone, Debug, PartialEq)]
pub struct Authenticate {
    request: AuthenticationRequest,
}

impl Authenticate {
    /// Constructs a new instance of the `Authenticate` initialized with the
    /// given credentials.
    pub fn with_credentials(credentials: Credentials) -> Self {
        Authenticate {
            request: AuthenticationRequest::new(credentials.username(), credentials.password()),
        }
    }

    /// Constructs a new instance of the `Authenticate` initialized with the
    /// given username and password.
    pub fn with_user<N, P>(username: N, password: P) -> Self
        where N: Into<String>, P: Into<String>
    {
        Authenticate {
            request: AuthenticationRequest::new(username, password),
        }
    }
}

impl Method for Authenticate {
    type Result = AuthenticationResponse;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for Authenticate {
    type Content = AuthenticationRequest;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_open/auth")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.request)
    }
}
