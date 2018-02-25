
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{IntoIterator, Iterator};
use std::rc::Rc;
use std::vec::IntoIter;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

pub use rincon_core::api::connector::{Connector, Error};
pub use rincon_core::api::method::ResultList;
pub use rincon_core::api::query::Query;
pub use rincon_core::api::types::{Empty, Entity};
pub use rincon_client::admin::types::{ServerVersion, TargetVersion};
pub use rincon_client::aql::types::{ExplainedQuery, ExplainOptions, ParsedQuery};
pub use rincon_client::collection::types::{Collection, CollectionProperties,
    CollectionPropertiesUpdate, CollectionRevision, NewCollection, RenameTo};
pub use rincon_client::cursor::types::{Cursor, CursorStatistics, NewCursor,
    Warning};
pub use rincon_client::database::types::{Database, NewDatabase};
pub use rincon_client::document::types::{Document, DocumentHeader, DocumentKey,
    DocumentId, DocumentModifyOptions, DocumentReplaceOptions, DocumentUpdate,
    NewDocument, UpdatedDocument};
pub use rincon_client::graph::types::{EdgeDefinition, Graph, NewGraph};
pub use rincon_client::user::types::{NewUser, Permission, User, UserExtra,
    UserUpdate};

use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::connector::Execute;
use rincon_core::arango::protocol::SYSTEM_DATABASE;
use rincon_client::admin::methods::*;
use rincon_client::aql::methods::*;
use rincon_client::collection::methods::*;
use rincon_client::cursor::methods::*;
use rincon_client::database::methods::*;
use rincon_client::document::methods::*;
use rincon_client::graph::methods::*;
use rincon_client::user::methods::*;

pub type Result<T> = ::std::result::Result<T, Error>;

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
    where C: 'static + Connector
{
    /// Instantiates a new `ArangoSession` using the given `Connector`.
    pub fn new(connector: C, core: Core) -> Self {
        ArangoSession {
            connector: Rc::new(connector),
            core: Rc::new(RefCell::new(core)),
        }
    }

    pub fn close(self) {
        //TODO see if a close() method has any purpose
    }

    /// Executes an API method applied to the system database.
    pub fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(self.connector.system_connection().execute(method))
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
        DatabaseSession::new(SYSTEM_DATABASE.to_owned(), self.connector.clone(), self.core.clone())
    }

    /// Returns a new `DatabaseSession` for the given database name.
    pub fn use_database_with_name<N>(&self, database_name: N) -> DatabaseSession<C>
        where N: Into<String>
    {
        DatabaseSession::new(database_name.into(), self.connector.clone(), self.core.clone())
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
    pub fn create_database<N, E>(&self, name: N, users: Vec<NewUser<E>>) -> Result<DatabaseSession<C>>
        where N: Into<String>, E: 'static + UserExtra + Serialize
    {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = name.into();
        self.execute(CreateDatabase::with_name_for_users(database_name.clone(), users))
            .map(move |_| DatabaseSession::new(database_name, connector, core))
    }

    /// Drops an existing database with the given name.
    ///
    /// Returns true if the database has been dropped successfully.
    pub fn drop_database<N>(&self, name: N) -> Result<bool>
        where N: Into<String>
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
        where N: Into<String>, P: Into<String>,
              E: 'static + UserExtra + Serialize + DeserializeOwned
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
        where E: 'static + UserExtra + Serialize + DeserializeOwned
    {
        self.execute(CreateUser::new(user))
    }

    /// Deletes an existing user with the given name.
    ///
    /// The user set in the `Connector` must have permission to write to the
    /// system database in order to execute this method.
    pub fn delete_user<N>(&self, username: N) -> Result<Empty>
        where N: Into<String>
    {
        self.execute(DeleteUser::with_name(username))
    }

    /// Fetches data about a user with the given name.
    ///
    /// This method can fetch data about the user set in the `Connector`. To
    /// retrieve data about any user the user set in the `Connector` must have
    /// permission to read from the system database.
    pub fn get_user<N, E>(&self, username: N) -> Result<User<E>>
        where N: Into<String>, E: 'static + UserExtra + Serialize + DeserializeOwned
    {
        self.execute(GetUser::with_name(username))
    }

    /// Fetches data about all available users.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    pub fn list_users<E>(&self) -> Result<Vec<User<E>>>
        where E: 'static + UserExtra + Serialize + DeserializeOwned
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
        where N: Into<String>, E: 'static + UserExtra + Serialize + DeserializeOwned
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
        where N: Into<String>, E: 'static + UserExtra + Serialize + DeserializeOwned
    {
        self.execute(ReplaceUser::new(username.into(), updates))
    }

    /// Lists all databases accessible by the given user.
    ///
    /// The user set in the `Connector` must have permission to read from the
    /// system database in order to execute this method.
    pub fn list_databases_for_user<N>(&self, username: N) -> Result<HashMap<String, Permission>>
        where N: Into<String>
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
    pub fn grant_default_database_access<N>(&self, username: N, permission: Permission) -> Result<Empty>
        where N: Into<String>
    {
        self.execute(SetDatabaseAccessLevel::new(username.into(), "*".into(), permission))
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
    pub fn grant_default_collection_access<N>(&self, username: N, permission: Permission) -> Result<Empty>
        where N: Into<String>
    {
        self.execute(SetCollectionAccessLevel::new(username.into(), "*".into(), "*".into(), permission))
    }

    pub fn get_database_access_level<N, Db>(&self, username: N, database: Db) -> Result<Permission>
        where N: Into<String>, Db: Into<String>
    {
        self.execute(GetDatabaseAccessLevel::new(username.into(), database.into()))
    }

    pub fn grant_database_access<N, Db>(&self, username: N, database: Db, permission: Permission) -> Result<Empty>
        where N: Into<String>, Db: Into<String>
    {
        self.execute(SetDatabaseAccessLevel::new(username.into(), database.into(), permission))
    }

    pub fn revoke_database_access<N, Db>(&self, username: N, database: Db) -> Result<Empty>
        where N: Into<String>, Db: Into<String>
    {
        self.execute(SetDatabaseAccessLevel::new(username.into(), database.into(), Permission::None))
    }

    pub fn reset_database_access<N, Db>(&self, username: N, database: Db) -> Result<Empty>
        where N: Into<String>, Db: Into<String>
    {
        self.execute(ResetDatabaseAccessLevel::new(username.into(), database.into()))
    }

    pub fn get_collection_access_level<N, Db, Coll>(&self, username: N, database: Db, collection: Coll) -> Result<Permission>
        where N: Into<String>, Db: Into<String>, Coll: Into<String>
    {
        self.execute(GetCollectionAccessLevel::new(username.into(), database.into(), collection.into()))
    }

    pub fn grant_collection_access<N, Db, Coll>(&self, username: N, database: Db, collection: Coll, permission: Permission) -> Result<Empty>
        where N: Into<String>, Db: Into<String>, Coll: Into<String>
    {
        self.execute(SetCollectionAccessLevel::new(username.into(), database.into(), collection.into(), permission))
    }

    pub fn revoke_collection_access<N, Db, Coll>(&self, username: N, database: Db, collection: Coll) -> Result<Empty>
        where N: Into<String>, Db: Into<String>, Coll: Into<String>
    {
        self.execute(SetCollectionAccessLevel::new(username.into(), database.into(), collection.into(), Permission::None))
    }

    pub fn reset_collection_access<N, Db, Coll>(&self, username: N, database: Db, collection: Coll) -> Result<Empty>
        where N: Into<String>, Db: Into<String>, Coll: Into<String>
    {
        self.execute(ResetCollectionAccessLevel::new(username.into(), database.into(), collection.into()))
    }
}

/// A session for operating with a specific database.
#[derive(Debug)]
pub struct DatabaseSession<C> {
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> DatabaseSession<C>
    where C: 'static + Connector
{
    /// Instantiates a new `DatabaseSession` for the database with the given
    /// name.
    fn new(database_name: String, connector: Rc<C>, core: Rc<RefCell<Core>>) -> Self {
        DatabaseSession {
            database_name,
            connector,
            core,
        }
    }

    /// Executes an API method applied to the database of this session.
    pub fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database this `DatabaseSession` operates with.
    pub fn name(&self) -> &str {
        &self.database_name
    }

    /// Drops the database that is used in this session.
    ///
    /// Returns true if the database has been dropped successfully.
    ///
    /// After calling this function the associated `DatabaseSession` is no
    /// longer valid.
    pub fn drop(self) -> Result<bool> {
        let database_name = self.database_name.to_owned();
        self.core.borrow_mut().run(self.connector.system_connection()
            .execute(DropDatabase::new(database_name))
        )
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// All cursor options and query execution options are left to their default
    /// settings.
    ///
    /// To specify cursor options and/or query execution options use the
    /// `query_opt(&self, NewCursor)` function.
    pub fn query<T>(&self, query: Query) -> Result<Cursor<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(CreateCursor::from_query(query))
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// It requires a `NewCursor` struct as a parameter which allows full
    /// control over all supported cursor options and query execution options.
    ///
    /// To execute a query with all options left at their defaults the
    /// `query(&self, Query)` function might be more suitable.
    pub fn query_opt<T>(&self, new_cursor: NewCursor) -> Result<Cursor<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(CreateCursor::new(new_cursor))
    }

    /// Generates an execution plan for a query but does not execute it.
    pub fn explain_query(&self, query: Query) -> Result<ExplainedQuery> {
        self.execute(ExplainQuery::with_defaults(query))
    }

    /// Generates an execution plan for a query but does not execute it.
    ///
    /// Some options about how many execution plans are generated and the
    /// configuration options for the query optimizer can be provided.
    pub fn explain_query_opt(&self, query: Query, options: ExplainOptions) -> Result<ExplainedQuery> {
        self.execute(ExplainQuery::with_options(query, options))
    }

    /// Parses a query a validates the syntax but does not execute it.
    ///
    /// If the query can be parsed without error the abstract syntax tree (AST)
    /// of the query is returned.
    pub fn parse_query<Q>(&self, query: Q) -> Result<ParsedQuery>
        where Q: Into<String>
    {
        self.execute(ParseQuery::from_query(query.into()))
    }

    /// Fetch the document with the given id from the database of this session.
    pub fn get_document<T>(&self, id: DocumentId) -> Result<Document<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(GetDocument::with_id(id))
    }

    /// Returns a new `CollectionSession` for the collection with the given
    /// name.
    pub fn use_collection_with_name<N>(&self, collection_name: N) -> CollectionSession<C>
        where N: Into<String>
    {
        CollectionSession {
            entity: Entity::Name(collection_name.into()),
            database_name: self.database_name.clone(),
            connector: self.connector.clone(),
            core: self.core.clone(),
        }
    }

    /// Returns a new `CollectionSession` for the given collection.
    pub fn use_collection(&self, collection: Collection) -> CollectionSession<C> {
        CollectionSession {
            entity: Entity::Object(collection),
            database_name: self.database_name.clone(),
            connector: self.connector.clone(),
            core: self.core.clone(),
        }
    }

    /// Creates a new collection within the database of this session.
    pub fn create_collection<N>(&self, collection_name: N) -> Result<CollectionSession<C>>
        where N: Into<String>
    {
        self.execute(CreateCollection::with_name(collection_name))
            .map(|props|
                CollectionSession {
                    entity: Entity::Object(Collection::from(props)),
                    database_name: self.database_name.clone(),
                    connector: self.connector.clone(),
                    core: self.core.clone(),
                }
            )
    }

    /// Fetches a list of all collections in this database.
    ///
    /// System collections are not included in the returned list.
    pub fn list_collections(&self) -> Result<Vec<Collection>> {
        self.execute(ListCollections::new())
    }

    /// Fetches a list of all collections in this database including system
    /// collections.
    pub fn list_collections_including_system(&self) -> Result<Vec<Collection>> {
        self.execute(ListCollections::including_system())
    }

    /// Returns a new `GraphSession` for the graph with the given name.
    pub fn use_graph_with_name<N>(&self, graph_name: N) -> GraphSession<C>
        where N: Into<String>
    {
        GraphSession {
            entity: Entity::Name(graph_name.into()),
            database_name: self.database_name.clone(),
            connector: self.connector.clone(),
            core: self.core.clone(),
        }
    }

    /// Returns a new `GraphSession` for the given graph.
    pub fn use_graph(&self, graph: Graph) -> GraphSession<C> {
        GraphSession {
            entity: Entity::Object(graph),
            database_name: self.database_name.clone(),
            connector: self.connector.clone(),
            core: self.core.clone(),
        }
    }

    /// Creates a new graph in the database represented by this
    /// `DatabaseSession`.
    pub fn create_graph(&self, new_graph: NewGraph) -> Result<GraphSession<C>> {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = self.database_name.clone();
        self.execute(CreateGraph::new(new_graph))
            .map(|graph|
                GraphSession {
                    entity: Entity::Object(graph),
                    database_name,
                    connector,
                    core,
                }
            )
    }

    /// Fetches a list of all graphs in this database.
    pub fn list_graphs(&self) -> Result<Vec<Graph>> {
        self.execute(ListGraphs::new())
    }
}

/// A session for operating with a specific `Cursor`.
#[derive(Debug)]
pub struct CursorSession<T, C> {
    cursor: Cursor<T>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<T, C> CursorSession<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database the query has been executed for.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the `Cursor` entity of this session.
    pub fn entity(&self) -> &Cursor<T> {
        &self.cursor
    }

    /// Unwraps the `Cursor` entity out of this session.
    pub fn unwrap_entity(self) -> Cursor<T> {
        self.cursor
    }

    /// Returns the id of this cursor.
    pub fn id(&self) -> Option<&String> {
        self.cursor.id()
    }

    /// Returns the slice of result documents retrieved with this cursor.
    ///
    /// The query may have more results. Whether a query has more results can
    /// be checked by the `has_more()` attribute function. To fetch the next
    /// batch of results use the `next_cursor()` function or iterate over all
    /// results by using the `Iterator` returned by the `into_iter()` function.
    pub fn result(&self) -> &[T] {
        self.cursor.result()
    }

    /// Returns whether there are more results available for this cursor on
    /// the server.
    pub fn has_more(&self) -> bool {
        self.cursor.has_more()
    }

    /// Checks whether this cursor has more results and if yes fetches a
    /// cursor with the next batch of results and returns it as a new
    /// `CursorSession`.
    ///
    /// This function returns `None` if there are no more results for this
    /// cursor. It returns `Some(Error)` if fetching the next batch of results
    /// fails.
    pub fn next_cursor(&self) -> Option<Result<CursorSession<T, C>>> {
        self.cursor.id().map(|v| v.to_owned()).map(|id|
            self.execute(ReadNextBatchFromCursor::with_id(id))
                .map(|cursor| CursorSession {
                    cursor,
                    database_name: self.database_name.clone(),
                    connector: self.connector.clone(),
                    core: self.core.clone(),
                })
        )
    }

    /// Returns whether the query result was served from the query cache or not.
    ///
    /// If the query result is served from the query cache, the stats attribute
    /// will be `None`.
    pub fn is_cached(&self) -> bool {
        self.cursor.is_cached()
    }

    /// Returns the total number of result documents available (only available
    /// if the query was executed with the count attribute set).
    pub fn count(&self) -> Option<u64> {
        self.cursor.count()
    }

    /// Returns the statistics about the execution of data modification queries.
    ///
    /// The stats will be `None` if the query is not a data modification query
    /// or the result is served from the query cache.
    pub fn stats(&self) -> Option<&CursorStatistics> {
        self.cursor.stats()
    }

    /// Returns warnings that occurred during query execution.
    pub fn warnings(&self) -> Option<&Vec<Warning>> {
        self.cursor.warnings()
    }
}

impl<T, C> IntoIterator for CursorSession<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    type Item = Result<T>;
    type IntoIter = CursorSessionIntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let has_more = self.cursor.has_more();
        let (cursor_id, count, result) = self.cursor.unwrap();
        CursorSessionIntoIter {
            batch: result.into_iter(),
            count,
            has_more,
            cursor_id,
            database_name: self.database_name,
            connector: self.connector,
            core: self.core,
        }
    }
}

/// An `Iterator` over all results for a specific cursor.
#[derive(Debug)]
pub struct CursorSessionIntoIter<T, C> {
    batch: IntoIter<T>,
    count: Option<u64>,
    has_more: bool,
    cursor_id: Option<String>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<T, C> CursorSessionIntoIter<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }
}

impl<T, C> Iterator for CursorSessionIntoIter<T, C>
    where T: 'static + DeserializeOwned, C: 'static + Connector
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.batch.next() {
            Some(Ok(next))
        } else if self.has_more {
            self.cursor_id.clone().and_then(|id| {
                let cursor = self.execute(ReadNextBatchFromCursor::new(id));
                match cursor {
                    Ok(cursor) => {
                        self.has_more = cursor.has_more();
                        let (id, count, result) = cursor.unwrap();
                        self.cursor_id = id;
                        self.count = count;
                        self.batch = result.into_iter();
                        self.batch.next().map(|v| Ok(v))
                    },
                    Err(error) => Some(Err(error)),
                }
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.batch.len(), self.count.map(|c| c as usize))
    }

    fn count(self) -> usize where Self: Sized {
        self.count.map(|c| c as usize)
            .unwrap_or_else(|| self.fold(0, |cnt, _| cnt + 1))
    }
}

/// A session for operating with a specific collection.
#[derive(Debug)]
pub struct CollectionSession<C> {
    entity: Entity<Collection>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> CollectionSession<C>
    where C: 'static + Connector
{
    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database this collection is located in.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the name of the collection this `CollectionSession` operates
    /// with.
    pub fn name(&self) -> &str {
        match self.entity {
            Entity::Name(ref name) => name,
            Entity::Object(ref obj) => obj.name(),
        }
    }

    /// Returns the `Collection` entity this `CollectionSession` operates with.
    ///
    /// It returns `Some(&Collection)` if this session holds a loaded collection
    /// entity or `None` otherwise.
    ///
    /// If the collection entity is not loaded the `load()` function can be
    /// used to get a session with a loaded collection entity.
    pub fn entity(&self) -> Option<&Collection> {
        match self.entity {
            Entity::Name(_) => None,
            Entity::Object(ref collection) => Some(collection),
        }
    }

    /// Unwraps the collection entity out of this session which is either the
    /// name of the collection or the `Collection` struct.
    pub fn unwrap_entity(self) -> Entity<Collection> {
        self.entity
    }

    /// Returns whether this session holds a loaded collection entity.
    ///
    /// It returns true if this session holds the collection entity or false
    /// otherwise. If this function returns true the `entity()` function will
    /// return `Some(&Collection)` otherwise that function returns `None`.
    pub fn is_entity(&self) -> bool {
        match self.entity {
            Entity::Name(_) => false,
            Entity::Object(_) => true,
        }
    }

    /// Fetches the entity of the collection represented by this session and
    /// returns a new `CollectionSession` with the entity set in the session.
    pub fn fetch(self) -> Result<CollectionSession<C>> {
        self.execute(GetCollection::with_name(self.name().clone()))
            .map(|collection|
                CollectionSession {
                    entity: Entity::Object(collection),
                    database_name: self.database_name,
                    connector: self.connector,
                    core: self.core,
                }
            )
    }

    /// Renames the collection represented by this session and returns the
    /// renamed collection as a new `CollectionSession`.
    pub fn rename<N>(self, new_name: N) -> Result<CollectionSession<C>>
        where N: Into<String>
    {
        self.execute(RenameCollection::new(self.name().into(), RenameTo::new(new_name)))
            .map(|collection| CollectionSession {
                entity: Entity::Object(collection),
                database_name: self.database_name,
                connector: self.connector,
                core: self.core,
            })
    }

    /// Gets the revision of the collection represented by this session.
    pub fn get_revision(&self) -> Result<CollectionRevision> {
        self.execute(GetCollectionRevision::new(self.name().into()))
    }

    /// Gets the properties of the collection represented by this session.
    pub fn get_properties(&self) -> Result<CollectionProperties> {
        self.execute(GetCollectionProperties::with_name(self.name().clone()))
    }

    /// Changes the properties of the collection represented by this session
    /// and returns the updated collection properties.
    pub fn change_properties(&self, properties: CollectionPropertiesUpdate) -> Result<CollectionProperties> {
        self.execute(ChangeCollectionProperties::new(self.name().into(), properties))
    }

    /// Inserts a new document into this collection.
    pub fn insert_document<D, T>(&self, document: D) -> Result<DocumentHeader>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocument::new(self.name(), document.into()))
    }

    /// Inserts a new document into this collection with forced waiting for the
    /// document being synced to disk.
    pub fn insert_document_synced<D, T>(&self, document: D) -> Result<DocumentHeader>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocument::new(self.name(), document.into())
            .with_force_wait_for_sync(true))
    }

    /// Inserts a new document into this collection and returns the newly
    /// created document.
    pub fn insert_document_return_new<D, T>(&self, document: D) -> Result<Document<T>>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentReturnNew::new(self.name(), document.into()))
    }

    /// Inserts a new document into this collection with forced waiting for the
    /// document being synced to disk and returns the newly created document
    pub fn insert_document_return_new_synced<D, T>(&self, document: D) -> Result<Document<T>>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentReturnNew::new(self.name(), document.into())
            .with_force_wait_for_sync(true))
    }

    /// Inserts multiple documents into this collection.
    pub fn insert_documents<D, T>(&self, documents: D) -> Result<ResultList<DocumentHeader>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocuments::new(self.name(), documents))
    }

    /// Inserts multiple documents into this collection with forced waiting for
    /// the documents being synced to disk.
    pub fn insert_documents_synced<D, T>(&self, documents: D) -> Result<ResultList<DocumentHeader>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocuments::new(self.name(), documents)
            .with_force_wait_for_sync(true))
    }

    /// Inserts multiple documents into this collection and returns the newly
    /// created documents.
    pub fn insert_documents_return_new<D, T>(&self, documents: D) -> Result<ResultList<Document<T>>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentsReturnNew::new(self.name(), documents))
    }

    /// Inserts multiple documents into this collection with forced waiting for
    /// the documents being synced to disk and returns the newly created
    /// documents.
    pub fn insert_documents_return_new_synced<D, T>(&self, documents: D) -> Result<ResultList<Document<T>>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentsReturnNew::new(self.name(), documents)
            .with_force_wait_for_sync(true))
    }

    /// Fetches the document with the given key from this collection.
    pub fn get_document<T>(&self, key: DocumentKey) -> Result<Document<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(GetDocument::new(self.name(), key))
    }

    /// Fetches the document with the given key from this collection if the
    /// revision matches the given predicate.
    pub fn get_document_if_match<IfMatch, T>(&self, key: DocumentKey, if_match: IfMatch) -> Result<Document<T>>
        where
            IfMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocument::new(self.name(), key).with_if_match(if_match))
    }

    /// Fetches the document with the given key from this collection if the
    /// revision does not match the given predicate.
    pub fn get_document_if_non_match<IfNonMatch, T>(&self, key: DocumentKey, if_non_match: IfNonMatch) -> Result<Document<T>>
        where
            IfNonMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocument::new(self.name(), key).with_if_non_match(if_non_match))
    }

    /// Fetches multiple documents with the given keys from this collection.
    pub fn get_documents<Keys, T>(&self, keys: Keys) -> Result<ResultList<Document<T>>>
        where
            Keys: IntoIterator<Item=DocumentKey>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocuments::with_keys(self.name(), keys))
    }

    /// Replaces an existing document with new content.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    pub fn replace_document<Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document))
    }

    /// Replaces an existing document with new content if the match condition
    /// is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    pub fn replace_document_if_match<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        if_match: IfMatch,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document)
            .with_if_match(if_match)
        )
    }

    /// Replaces an existing document with new content if the match condition
    /// is met. This function allows to specify detailed options for the
    /// replace method.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    /// * `options` : Additional options for the replace method
    pub fn replace_document_if_match_opt<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        if_match: IfMatch,
        options: DocumentReplaceOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document)
            .with_if_match(if_match)
            .with_options(options)
        )
    }

    /// Replaces an existing document with new content. This function allows
    /// to specify detailed options for the replace method.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `options` : Additional options for the replace method
    pub fn replace_document_opt<Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        options: DocumentReplaceOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document).with_options(options))
    }

    /// Partially modifies an existing document.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    pub fn modify_document<Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications))
    }

    /// Partially modifies an existing document if the match condition is met.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    pub fn modify_document_if_match<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        if_match: IfMatch,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_if_match(if_match)
        )
    }

    /// Partially modifies an existing document if the match condition is met.
    /// This function allows to specify detailed options for the modify method.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    /// * `options` : Additional options for the modify method
    pub fn modify_document_if_match_opt<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        if_match: IfMatch,
        options: DocumentModifyOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_if_match(if_match)
            .with_options(options)
        )
    }

    /// Partially modifies an existing document. This function allows to
    /// specify detailed options for the modify method.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `options` : Additional options for the modify method
    pub fn modify_document_opt<Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        options: DocumentModifyOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_options(options)
        )
    }

    pub fn delete_document(&self, document_key: DocumentKey) -> Result<DocumentHeader> {
        self.execute(DeleteDocument::new(self.name(), document_key))
    }

    pub fn delete_document_if_match<IfMatch>(
        &self,
        document_key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<DocumentHeader>
        where
            IfMatch: Into<String>,
    {
        self.execute(DeleteDocument::new(self.name(), document_key)
            .with_if_match(if_match)
        )
    }

    pub fn delete_document_if_match_synced<IfMatch>(
        &self,
        document_key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<DocumentHeader>
        where
            IfMatch: Into<String>,
    {
        self.execute(DeleteDocument::new(self.name(), document_key)
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    pub fn delete_document_synced(&self, document_key: DocumentKey) -> Result<DocumentHeader> {
        self.execute(DeleteDocument::new(self.name(), document_key)
            .with_force_wait_for_sync(true)
        )
    }

    pub fn delete_document_return_old<Old>(&self, document_key: DocumentKey) -> Result<Document<Old>>
        where
            Old: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), document_key))
    }

    pub fn delete_document_if_match_return_old<IfMatch, T>(
        &self,
        document_key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<Document<T>>
        where
            IfMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), document_key)
            .with_if_match(if_match)
        )
    }

    pub fn delete_document_if_match_return_old_synced<IfMatch, T>(
        &self,
        document_key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<Document<T>>
        where
            IfMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), document_key)
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    pub fn delete_document_return_old_synced<T>(
        &self,
        document_key: DocumentKey,
    ) -> Result<Document<T>>
        where
            T: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), document_key)
            .with_force_wait_for_sync(true))
    }
}

/// A session for operating with a specific graph.
#[derive(Debug)]
pub struct GraphSession<C> {
    entity: Entity<Graph>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> GraphSession<C>
    where C: 'static + Connector
{
    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database this graph is located in.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the name of the graph this `GraphSession` operates with.
    pub fn name(&self) -> &str {
        match self.entity {
            Entity::Name(ref name) => name,
            Entity::Object(ref obj) => obj.name(),
        }
    }

    /// Returns the `Graph` entity this `GraphSession` operates with.
    ///
    /// It returns `Some(&Graph)` when this session holds a loaded graph entity
    /// or `None` otherwise.
    ///
    /// If the graph entity is not loaded the `load()` function can be used
    /// to get a session with a loaded graph entity.
    pub fn entity(&self) -> Option<&Graph> {
        match self.entity {
            Entity::Name(_) => None,
            Entity::Object(ref graph) => Some(graph),
        }
    }

    /// Unwraps the graph entity out of this session which is either the name
    /// of the graph or the `Graph` struct.
    pub fn unwrap_entity(self) -> Entity<Graph> {
        self.entity
    }

    /// Returns whether this session holds a loaded graph entity.
    ///
    /// It returns true if this session holds the graph entity or false
    /// otherwise. If this function returns true the `entity()` function will
    /// return `Some(&Graph)` otherwise that function returns `None`.
    pub fn is_entity(&self) -> bool {
        match self.entity {
            Entity::Name(_) => false,
            Entity::Object(_) => true,
        }
    }

    /// Fetches the entity of the graph represented by this session and returns
    /// a new `GraphSession` with the entity set in the session.
    pub fn fetch(self) -> Result<GraphSession<C>> {
        self.execute(GetGraph::with_name(self.name().clone()))
            .map(|graph|
                GraphSession {
                    entity: Entity::Object(graph),
                    database_name: self.database_name,
                    connector: self.connector,
                    core: self.core,
                }
            )
    }
}
