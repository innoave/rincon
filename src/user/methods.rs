
use std::collections::HashMap;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use api::types::Empty;
use arango::protocol::{FIELD_CODE, FIELD_RESULT, PATH_DATABASE, PATH_API_USER};
use super::types::*;

/// Creates a new user.
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
        code_field: Some(FIELD_CODE),
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
        String::from(PATH_API_USER)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
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
        result_field: Some(FIELD_RESULT),
        code_field: Some(FIELD_CODE),
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
        String::from(PATH_API_USER)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
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
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RemoveUser {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
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
        code_field: Some(FIELD_CODE),
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
        String::from(PATH_API_USER) + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
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
        code_field: Some(FIELD_CODE),
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
        String::from(PATH_API_USER) + "/" + &self.user_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
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
        code_field: Some(FIELD_CODE),
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
        String::from(PATH_API_USER) + "/" + &self.user_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.updates)
    }
}

/// Fetches the list of databases available to the specified user.
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
        result_field: Some(FIELD_RESULT),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ListDatabasesForUser {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Fetch the database access level for a specific database.
#[derive(Clone, Debug, PartialEq)]
pub struct GetDatabaseAccessLevel {
    user_name: String,
    database: String,
}

impl GetDatabaseAccessLevel {
    /// Constructs a new instance of the `GetDatabaseAccessLevel` method.
    pub fn new(user_name: String, database: String) -> Self {
        GetDatabaseAccessLevel {
            user_name,
            database,
        }
    }

    /// Returns the user name of for which the database access level should
    /// be fetched.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the database access level
    /// should be fetched.
    pub fn database(&self) -> &str {
        &self.database
    }
}

impl Method for GetDatabaseAccessLevel {
    type Result = Permission;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_RESULT),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetDatabaseAccessLevel {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Set the database access level for an user.
///
/// You need permission to the _system database in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
pub struct SetDatabaseAccessLevel {
    user_name: String,
    database: String,
    access_level: NewAccessLevel,
}

impl SetDatabaseAccessLevel {
    /// Constructs a new instance of the `SetDatabaseAccessLevel` method.
    pub fn new(user_name: String, database: String, grant: Permission) -> Self {
        SetDatabaseAccessLevel {
            user_name,
            database,
            access_level: NewAccessLevel::new(grant),
        }
    }

    /// Returns the user name for which the database access level should
    /// be set.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the database access level
    /// should be set.
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Returns the access level that should be set.
    pub fn access_level(&self) -> &NewAccessLevel {
        &self.access_level
    }
}

impl Method for SetDatabaseAccessLevel {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for SetDatabaseAccessLevel {
    type Content = NewAccessLevel;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.access_level)
    }
}

/// Reset the database access level for an user. As consequence the default
/// database access level is applied. If there is no defined default database
/// access level, it defaults to 'No access'.
///
/// You need permission to the _system database in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
pub struct ResetDatabaseAccessLevel {
    user_name: String,
    database: String,
}

impl ResetDatabaseAccessLevel {
    /// Constructs a new instance of the `ResetDatabaseAccessLevel` method.
    pub fn new(user_name: String, database: String) -> Self {
        ResetDatabaseAccessLevel {
            user_name,
            database,
        }
    }

    /// Returns the user name for which the database access level should
    /// be reset.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the database access level
    /// should be reset.
    pub fn database(&self) -> &str {
        &self.database
    }
}

impl Method for ResetDatabaseAccessLevel {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ResetDatabaseAccessLevel {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Fetch the collection access level for a specific collection.
#[derive(Clone, Debug, PartialEq)]
pub struct GetCollectionAccessLevel {
    user_name: String,
    database: String,
    collection: String,
}

impl GetCollectionAccessLevel {
    /// Constructs a new instance of the `GetCollectionAccessLevel` method.
    pub fn new(user_name: String, database: String, collection: String) -> Self {
        GetCollectionAccessLevel {
            user_name,
            database,
            collection,
        }
    }

    /// Returns the user name for which the collection access level should
    /// be fetched.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the collection access level
    /// should be fetched.
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Returns the name of the collection for which the collection access level
    /// should be fetched.
    pub fn collection(&self) -> &str {
        &self.collection
    }
}

impl Method for GetCollectionAccessLevel {
    type Result = Permission;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_RESULT),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetCollectionAccessLevel {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
            + "/" + &self.collection
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Set the collection access level for an user.
///
/// You need permission to the _system database in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
pub struct SetCollectionAccessLevel {
    user_name: String,
    database: String,
    collection: String,
    access_level: NewAccessLevel,
}

impl SetCollectionAccessLevel {
    /// Constructs a new instance of the `SetCollectionAccessLevel` method.
    pub fn new(user_name: String,
        database: String,
        collection: String,
        grant: Permission
    ) -> Self {
        SetCollectionAccessLevel {
            user_name,
            database,
            collection,
            access_level: NewAccessLevel::new(grant),
        }
    }

    /// Returns the user name for which the collection access level should
    /// be set.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the collection access level
    /// should be set.
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Returns the name of the collection for which the collection access level
    /// should be set.
    pub fn collection(&self) -> &str {
        &self.collection
    }

    /// Returns the access level that should be set.
    pub fn access_level(&self) -> &NewAccessLevel {
        &self.access_level
    }
}

impl Method for SetCollectionAccessLevel {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for SetCollectionAccessLevel {
    type Content = NewAccessLevel;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
            + "/" + &self.collection
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.access_level)
    }
}

/// Reset the collection access level for an user. As consequence the default
/// collection access level is applied. If there is no defined default
/// collection access level, it defaults to 'No access'.
///
/// You need permission to the _system database in order to execute this method
/// call.
#[derive(Clone, Debug, PartialEq)]
pub struct ResetCollectionAccessLevel {
    user_name: String,
    database: String,
    collection: String,
}

impl ResetCollectionAccessLevel {
    /// Constructs a new instance of the `ResetCollectionAccessLevel` method.
    pub fn new(user_name: String, database: String, collection: String) -> Self {
        ResetCollectionAccessLevel {
            user_name,
            database,
            collection,
        }
    }

    /// Returns the user name for which the collection access level should
    /// be reset.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the name of the database for which the collection access level
    /// should be reset.
    pub fn database(&self) -> &str {
        &self.database
    }

    /// Returns the name of the collection for which the collection access level
    /// should be reset.
    pub fn collection(&self) -> &str {
        &self.collection
    }
}

impl Method for ResetCollectionAccessLevel {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ResetCollectionAccessLevel {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_USER) + "/" + &self.user_name
            + "/" + PATH_DATABASE + "/" + &self.database
            + "/" + &self.collection
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
