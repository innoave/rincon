use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

use rincon_client::admin::methods::*;
use rincon_client::admin::types::{ServerVersion, TargetVersion};
use rincon_client::database::methods::{
    CreateDatabase, DropDatabase, ListAccessibleDatabases, ListDatabases,
};
use rincon_client::user::methods::*;
use rincon_client::user::types::{NewUser, Permission, User, UserExtra, UserUpdate};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::types::Empty;
use rincon_core::arango::protocol::SYSTEM_DATABASE;

use super::Result;
use database_session::DatabaseSession;

/// A session for administrating databases and users.
///
/// An `ArangoSession` defines the entry point to the session api. It basically
/// determines which `Connector` implementation shall be used in an application
/// and provides functions for administrating databases and users.
#[derive(Debug)]
pub struct ArangoSession<C> {
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> ArangoSession<C>
where
    C: 'static + Connector,
{
    /// Instantiates a new `ArangoSession` using the given `Connector`.
    pub fn new(connector: C, core: Core) -> Self {
        ArangoSession {
            connector: Rc::new(connector),
            core: Rc::new(RefCell::new(core)),
        }
    }

    /// Executes an API method applied to the system database.
    pub fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
    where
        M: 'static + Method + Prepare,
    {
        self.core
            .borrow_mut()
            .run(self.connector.system_connection().execute(method))
    }

    /// Gets the server name and version number.
    pub fn get_server_version(&self) -> Result<ServerVersion> {
        self.execute(GetServerVersion::new())
    }

    /// Gets the server name and version number with additional details.
    pub fn get_server_version_details(&self) -> Result<ServerVersion> {
        self.execute(GetServerVersion::with_details())
    }

    /// Gets the database version a server requires.
    pub fn get_target_version(&self) -> Result<TargetVersion> {
        self.execute(GetTargetVersion::new())
    }

    /// Returns a new `DatabaseSession` for the system database.
    ///
    /// In *ArangoDB* the system database usually has the name `_system`.
    pub fn use_system_database(&self) -> DatabaseSession<C> {
        DatabaseSession::new(
            SYSTEM_DATABASE.to_owned(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Returns a new `DatabaseSession` for the given database name.
    pub fn use_database_with_name<N>(&self, database_name: N) -> DatabaseSession<C>
    where
        N: Into<String>,
    {
        DatabaseSession::new(
            database_name.into(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Creates a new database with the given attributes.
    ///
    /// If the database could be created successfully a `DatabaseSession` using
    /// the just created database is returned.
    ///
    /// The user provided with the `Connector` must have permission to access
    /// the system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `name`  : the name of the database to be created
    /// * `users` : a list of users to be assigned to the new database
    pub fn create_database<N, E>(
        &self,
        name: N,
        users: Vec<NewUser<E>>,
    ) -> Result<DatabaseSession<C>>
    where
        N: Into<String>,
        E: 'static + UserExtra + Serialize,
    {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = name.into();
        self.execute(CreateDatabase::with_name_for_users(
            database_name.clone(),
            users,
        )).map(move |_| DatabaseSession::new(database_name, connector, core))
    }

    /// Drops an existing database with the given name.
    ///
    /// Returns true if the database has been dropped successfully.
    pub fn drop_database<N>(&self, name: N) -> Result<bool>
    where
        N: Into<String>,
    {
        self.execute(DropDatabase::with_name(name))
    }

    /// Retrieves a list of all existing databases.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    pub fn list_databases(&self) -> Result<Vec<String>> {
        self.execute(ListDatabases::new())
    }

    /// Retrieves a list of all databases the current user has access to.
    pub fn list_accessible_databases(&self) -> Result<Vec<String>> {
        self.execute(ListAccessibleDatabases::new())
    }

    /// Creates a new user with default options.
    ///
    /// The created user will not have access to any database until database
    /// access privileges are explicitly granted to it.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    pub fn create_user<N, P, E>(&self, username: N, password: P) -> Result<User<E>>
    where
        N: Into<String>,
        P: Into<String>,
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(CreateUser::new(NewUser::with_name(username, password)))
    }

    /// Creates a new user with extra information.
    ///
    /// The created user will not have access to any database until database
    /// access privileges are explicitly granted to it.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    pub fn create_user_with_details<E>(&self, user: NewUser<E>) -> Result<User<E>>
    where
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(CreateUser::new(user))
    }

    /// Deletes an existing user with the given name.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    pub fn delete_user<N>(&self, username: N) -> Result<Empty>
    where
        N: Into<String>,
    {
        self.execute(DeleteUser::with_name(username))
    }

    /// Fetches data about a user with the given name.
    ///
    /// This method can fetch data about the user set in the `Connector`. To
    /// retrieve data about any user the user set in the `Connector` must have
    /// permission to read from the system database.
    pub fn get_user<N, E>(&self, username: N) -> Result<User<E>>
    where
        N: Into<String>,
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(GetUser::with_name(username))
    }

    /// Fetches data about all available users.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    pub fn list_users<E>(&self) -> Result<Vec<User<E>>>
    where
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(ListUsers::new())
    }

    /// Partially updates the data of an existing user.
    ///
    /// The password can only be changed for the user set in the `Connector`.
    /// To change the active status the user set in the `Connector` must have
    /// permission to write to the system database.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which the data shall be replaced
    /// * `updates`  : the data to be updated for the given user
    pub fn modify_user<N, E>(&self, username: N, updates: UserUpdate<E>) -> Result<User<E>>
    where
        N: Into<String>,
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(ModifyUser::new(username.into(), updates))
    }

    /// Replaces the data of an existing user.
    ///
    /// The password can only be changed for the user set in the `Connector`.
    /// To change the active status the user set in the `Connector` must have
    /// permission to write to the system database.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which the data shall be replaced
    /// * `updates`  : the new data of the user
    pub fn replace_user<N, E>(&self, username: N, updates: UserUpdate<E>) -> Result<User<E>>
    where
        N: Into<String>,
        E: 'static + UserExtra + Serialize + DeserializeOwned,
    {
        self.execute(ReplaceUser::new(username.into(), updates))
    }

    /// Lists all databases accessible by the given user.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    pub fn list_databases_for_user<N>(&self, username: N) -> Result<HashMap<String, Permission>>
    where
        N: Into<String>,
    {
        self.execute(ListDatabasesForUser::new(username.into()))
    }

    /// Sets the default access level for databases for the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which to grant default access
    /// * `permission` : the access level to grant
    pub fn grant_default_database_access<N>(
        &self,
        username: N,
        permission: Permission,
    ) -> Result<Empty>
    where
        N: Into<String>,
    {
        self.execute(SetDatabaseAccessLevel::new(
            username.into(),
            "*".into(),
            permission,
        ))
    }

    /// Sets the default access level for collections for the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which to grant default access
    /// * `permission` : the access level to grant
    pub fn grant_default_collection_access<N>(
        &self,
        username: N,
        permission: Permission,
    ) -> Result<Empty>
    where
        N: Into<String>,
    {
        self.execute(SetCollectionAccessLevel::new(
            username.into(),
            "*".into(),
            "*".into(),
            permission,
        ))
    }

    /// Gets the effective access level to the specified database for the given
    /// user.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which the permissions are
    /// queried * `database` : the name of the database for which the
    /// permissions are queried
    pub fn get_database_access_level<N, Db>(&self, username: N, database: Db) -> Result<Permission>
    where
        N: Into<String>,
        Db: Into<String>,
    {
        self.execute(GetDatabaseAccessLevel::new(
            username.into(),
            database.into(),
        ))
    }

    /// Grants access to the specified database for the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be granted
    /// * `database` : the name of the database to which access shall be granted
    /// * `permission` : the access level that shall be granted
    pub fn grant_database_access<N, Db>(
        &self,
        username: N,
        database: Db,
        permission: Permission,
    ) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
    {
        self.execute(SetDatabaseAccessLevel::new(
            username.into(),
            database.into(),
            permission,
        ))
    }

    /// Revokes the access to the specified database for the given user.
    ///
    /// After this call the given user has no access to the specified database.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be revoked
    /// * `database` : the name of the database from which access shall be
    /// revoked
    pub fn revoke_database_access<N, Db>(&self, username: N, database: Db) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
    {
        self.execute(SetDatabaseAccessLevel::new(
            username.into(),
            database.into(),
            Permission::None,
        ))
    }

    /// Resets the access to the specified database for the given user to the
    /// default access level.
    ///
    /// After this call the default access level to the database is applied for
    /// the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be reset to
    /// the default * `database` : the name of the database for which
    /// access shall be reset to the default
    pub fn reset_database_access<N, Db>(&self, username: N, database: Db) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
    {
        self.execute(ResetDatabaseAccessLevel::new(
            username.into(),
            database.into(),
        ))
    }

    /// Gets the effective access level to the specified collection for the
    /// given user.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which the permissions are
    /// queried * `database` : the name of the database where the
    /// collection is located in * `collection` : the name of the
    /// collection for which the permissions are queried
    pub fn get_collection_access_level<N, Db, Coll>(
        &self,
        username: N,
        database: Db,
        collection: Coll,
    ) -> Result<Permission>
    where
        N: Into<String>,
        Db: Into<String>,
        Coll: Into<String>,
    {
        self.execute(GetCollectionAccessLevel::new(
            username.into(),
            database.into(),
            collection.into(),
        ))
    }

    /// Grants access to the specified collection for the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be granted
    /// * `database` : the name of the database where the collection is located
    /// * `collection` : the name of the collection to which access shall be
    /// granted * `permission` : the access level that shall be granted
    pub fn grant_collection_access<N, Db, Coll>(
        &self,
        username: N,
        database: Db,
        collection: Coll,
        permission: Permission,
    ) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
        Coll: Into<String>,
    {
        self.execute(SetCollectionAccessLevel::new(
            username.into(),
            database.into(),
            collection.into(),
            permission,
        ))
    }

    /// Revokes the access to the specified collection for the given user.
    ///
    /// After this call the given user has no access to the specified
    /// collection.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be revoked
    /// * `database` : the name of the database where the collection is located
    /// * `collection` : the name of the collection from which access shall be
    /// revoked
    pub fn revoke_collection_access<N, Db, Coll>(
        &self,
        username: N,
        database: Db,
        collection: Coll,
    ) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
        Coll: Into<String>,
    {
        self.execute(SetCollectionAccessLevel::new(
            username.into(),
            database.into(),
            collection.into(),
            Permission::None,
        ))
    }

    /// Resets the access to the specified collection for the given user to the
    /// default access level.
    ///
    /// After this call the default access level to the collection is applied
    /// for the given user.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    ///
    /// # Arguments
    ///
    /// * `username` : the name of the user for which access shall be reset to
    /// the default * `database` : the name of the database where the
    /// collection is located * `collection` : the name of the collection
    /// for which access shall be reset to the default
    pub fn reset_collection_access<N, Db, Coll>(
        &self,
        username: N,
        database: Db,
        collection: Coll,
    ) -> Result<Empty>
    where
        N: Into<String>,
        Db: Into<String>,
        Coll: Into<String>,
    {
        self.execute(ResetCollectionAccessLevel::new(
            username.into(),
            database.into(),
            collection.into(),
        ))
    }
}
