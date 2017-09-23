
use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::{Empty, Method, Operation, Parameters, Prepare, RpcReturnType};
use super::types::*;

/// Create a new user.
///
/// You need 'Administrate server' access level in order to execute this
/// method call.
#[derive(Clone, Debug, PartialEq)]
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
/// You need the 'Administrate server' access level in order to execute this
/// method call. Otherwise, you will only get information about yourself.
#[derive(Clone, Debug, PartialEq)]
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

/// Removes an existing user, identified by name.
///
/// You need 'Administrate server' access level in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
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

/// Fetches data about the specified user.
///
/// You can fetch information about yourself or you need the 'Administrate
/// server' access level in order to execute this method.
#[derive(Clone, Debug, PartialEq)]
pub struct GetUser<E>
    where E: UserExtra
{
    name: String,
    user_info_type: PhantomData<E>,
}

impl<E> GetUser<E>
    where E: UserExtra
{
    /// Constructs a new instance of the `GetUser` method with all attributes
    /// explicitly set.
    pub fn new(user_name: String) -> Self {
        GetUser {
            name: user_name,
            user_info_type: PhantomData,
        }
    }

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

/// Partially updates the data of an existing user. The name of an existing
/// user must be specified in `user_name`.
///
/// You need 'Administrate server' access level in order to execute this method
/// call. Additionally, a user can change his/her own data.
#[derive(Clone, Debug, PartialEq)]
pub struct ModifyUser<E>
    where E: UserExtra
{
    /// The name of the user to be modified.
    user_name: String,
    /// The `UserUpdate` that holds the values that should be modified.
    updates: UserUpdate<E>,
}

impl<E> ModifyUser<E>
    where E: UserExtra
{
    /// Constructs a new instance of `ModifyUser` for the given user and with
    /// the given `UserUpdate`.
    pub fn new(user_name: String, updates: UserUpdate<E>) -> Self {
        ModifyUser {
            user_name,
            updates,
        }
    }

    /// Constructs a new instance of `ModifyUser` for the given user with no
    /// defined updates (an empty `UserUpdate`).
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        ModifyUser {
            user_name: name.into(),
            updates: UserUpdate::empty(),
        }
    }

    /// Returns the user name of the user to be modified.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the `UserUpdate` of this method.
    pub fn updates(&self) -> &UserUpdate<E> {
        &self.updates
    }
}

impl<E> Method for ModifyUser<E>
    where E: UserExtra + DeserializeOwned
{
    type Result = User<E>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<E> Prepare for ModifyUser<E>
    where E: UserExtra + Serialize
{
    type Content = UserUpdate<E>;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from("/_api/user/") + &self.user_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.updates)
    }
}

/// Replaces the data of an existing user. The name of an existing user must
/// be specified in `user_name`.
///
/// You need 'Administrate server' access level in order to execute this method
/// call. Additionally, a user can change his/her own data.
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceUser<E>
    where E: UserExtra
{
    /// The name of the user to be modified.
    user_name: String,
    /// The `UserUpdate` that holds the values that should be modified.
    updates: UserUpdate<E>,
}

impl<E> ReplaceUser<E>
    where E: UserExtra
{
    /// Constructs a new instance of `ReplaceUser` with the given `UserUpdate`.
    pub fn new(user_name: String, updates: UserUpdate<E>) -> Self {
        ReplaceUser {
            user_name,
            updates,
        }
    }

    /// Constructs a new instance of `ReplaceUser` for the given user with no
    /// defined updates (an empty `UserUpdate`).
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        ReplaceUser {
            user_name: name.into(),
            updates: UserUpdate::empty(),
        }
    }

    /// Returns the user name of the user to be modified.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the `UserUpdate` of this method.
    pub fn updates(&self) -> &UserUpdate<E> {
        &self.updates
    }

    /// Returns a mutable `UserUpdate` of this method.
    pub fn updates_mut(&mut self) -> &mut UserUpdate<E> {
        &mut self.updates
    }
}

impl<E> Method for ReplaceUser<E>
    where E: UserExtra + DeserializeOwned
{
    type Result = User<E>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<E> Prepare for ReplaceUser<E>
    where E: UserExtra + Serialize
{
    type Content = UserUpdate<E>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from("/_api/user/") + &self.user_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.updates)
    }
}

/// Fetch the list of databases available to the specified user.
///
/// You need 'Administrate server' access level in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
pub struct ListDatabasesForUser {
    user_name: String,
}

impl ListDatabasesForUser {
    /// Constructs a new instance of `ListDatabasesForUser` with the given
    /// user name.
    pub fn new(user_name: String) -> Self {
        ListDatabasesForUser {
            user_name,
        }
    }

    /// Constructs a new instance of `ListDatabasesForUser` with the given
    /// user name.
    pub fn for_user<N>(name: N) -> Self
        where N: Into<String>
    {
        ListDatabasesForUser {
            user_name: name.into(),
        }
    }

    /// Returns the user name for which the available databases should be
    /// listed.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
}

impl Method for ListDatabasesForUser {
    type Result = HashMap<String, Permission>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListDatabasesForUser {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/user/") + &self.user_name + "/database"
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
