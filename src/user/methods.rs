
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::{Empty, Method, Operation, Parameters, Prepare, RpcReturnType};
use super::types::*;

/// Create a new user.
///
/// You need 'Administrate server' access level in order to execute this
/// method call.
#[derive(Debug, PartialEq, Eq)]
pub struct CreateUser<E>
    where E: UserExtra
{
    user: NewUser<E>,
}

impl<E> CreateUser<E>
    where E: UserExtra
{
    /// Constructs a new `CreateUser` method with the given user parameter.
    pub fn new(user: NewUser<E>) -> Self {
        CreateUser {
            user,
        }
    }

    /// Returns the user parameter of this `CreateUser` method.
    pub fn user(&self) -> &NewUser<E> {
        &self.user
    }
}

impl<E> Method for CreateUser<E>
    where E: UserExtra + DeserializeOwned
{
    type Result = User<E>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<E> Prepare for CreateUser<E>
    where E: UserExtra + Serialize
{
    type Content = NewUser<E>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/user")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.user)
    }
}

/// Fetches data about all users.
///
/// You need the Administrate server access level in order to execute this REST
/// call. Otherwise, you will only get information about yourself.
#[derive(Debug, PartialEq, Eq)]
pub struct ListAvailableUsers<E>
    where E: UserExtra
{
    user_info_type: PhantomData<E>,
}

impl<E> ListAvailableUsers<E>
    where E: UserExtra
{
    /// Constructs a new `ListAvailableUsers` method.
    pub fn new() -> Self {
        ListAvailableUsers {
            user_info_type: PhantomData,
        }
    }
}

impl<E> Method for ListAvailableUsers<E>
    where E: UserExtra + DeserializeOwned
{
    type Result = Vec<User<E>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl<E> Prepare for ListAvailableUsers<E>
    where E: UserExtra
{
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/user")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Fetches data about the specified user.
///
/// You can fetch information about yourself or you need the 'Administrate
/// server' access level in order to execute this method.
#[derive(Debug, PartialEq, Eq)]
pub struct GetUser<E>
    where E: UserExtra
{
    name: String,
    user_info_type: PhantomData<E>,
}

impl<E> GetUser<E>
    where E: UserExtra
{
    /// Constructs a new `GetUser` method with the given user name.
    pub fn with_name<S>(user_name: S) -> Self
        where S: Into<String>
    {
        GetUser {
            name: user_name.into(),
            user_info_type: PhantomData,
        }
    }

    /// Returns the user name of the user to be fetched.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<E> Method for GetUser<E>
    where E: UserExtra + DeserializeOwned
{
    type Result = User<E>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<E> Prepare for GetUser<E>
    where E: UserExtra
{
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/user/") + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Removes an existing user, identified by user.
///
/// You need 'Administrate server' access level in order to execute this method
/// call.
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveUser {
    name: String,
}

impl RemoveUser {
    /// Constructs a new `RemoveUser` instance with the given user name of the
    /// user to be removed.
    pub fn with_name<S>(user_name: S) -> Self
        where S: Into<String>
    {
        RemoveUser {
            name: user_name.into(),
        }
    }

    /// Returns the name of the user to be removed.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for RemoveUser {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for RemoveUser {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from("/_api/user/") + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
