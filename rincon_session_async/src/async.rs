use std::rc::Rc;

use futures::Future;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub use rincon_client::cursor::types::{Cursor, NewCursor};
pub use rincon_client::database::types::{Database, NewDatabase};
pub use rincon_client::graph::types::{EdgeDefinition, Graph, NewGraph};
pub use rincon_client::user::types::{NewUser, UserExtra};
pub use rincon_core::api::connector::Error;
pub use rincon_core::api::query::Query;
pub use rincon_core::api::types::Empty;

use rincon_client::cursor::methods::CreateCursor;
use rincon_client::database::methods::{CreateDatabase, DropDatabase};
use rincon_client::graph::methods::CreateGraph;
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::arango::protocol::SYSTEM_DATABASE;

pub type FutureResult<T> = Box<Future<Item = T, Error = Error>>;

#[derive(Debug)]
pub struct ArangoSession<C> {
    connector: Rc<C>,
}

impl<C> ArangoSession<C>
where
    C: 'static + Connector,
{
    pub fn new(connector: C) -> Self {
        ArangoSession {
            connector: Rc::new(connector),
        }
    }

    pub fn close(self) {
        //TODO see if a close() method has any purpose
    }

    pub fn use_system_database(&self) -> DatabaseSession<C> {
        DatabaseSession::new(SYSTEM_DATABASE.to_owned(), self.connector.clone())
    }

    pub fn use_database<DbName>(&self, database_name: DbName) -> DatabaseSession<C>
    where
        DbName: Into<String>,
    {
        DatabaseSession::new(database_name.into(), self.connector.clone())
    }

    pub fn create_database<UserInfo>(
        &self,
        new_database: NewDatabase<UserInfo>,
    ) -> FutureResult<DatabaseSession<C>>
    where
        UserInfo: UserExtra + Serialize + 'static,
    {
        let connector = self.connector.clone();
        let database_name = new_database.name().to_owned();
        Box::new(
            self.connector
                .system_connection()
                .execute(CreateDatabase::new(new_database))
                .map(move |_| DatabaseSession::new(database_name, connector)),
        )
    }
}

#[derive(Debug)]
pub struct DatabaseSession<C> {
    database_name: String,
    connector: Rc<C>,
}

impl<C> DatabaseSession<C>
where
    C: 'static + Connector,
{
    fn new(database_name: String, connector: Rc<C>) -> Self {
        DatabaseSession {
            database_name,
            connector,
        }
    }

    pub fn name(&self) -> &str {
        &self.database_name
    }

    /// Drops the database that is used in this session.
    ///
    /// After calling this function the associated `DatabaseSession` is no
    /// longer valid.
    pub fn drop(self) -> FutureResult<bool> {
        let database_name = self.database_name.to_owned();
        Box::new(
            self.connector
                .system_connection()
                .execute(DropDatabase::new(database_name)),
        )
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// All cursor options and query execution options are left to their default
    /// settings.
    ///
    /// To specify cursor options and/or query execution options use the
    /// `query_opt(&self, NewCursor)` function.
    pub fn query<T>(&self, query: Query) -> FutureResult<Cursor<T>>
    where
        T: 'static + DeserializeOwned,
    {
        Box::new(
            self.connector
                .connection(&self.database_name)
                .execute(CreateCursor::from_query(query)),
        )
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// It requires a `NewCursor` struct as a parameter which allows full
    /// control over all supported cursor options and query execution options.
    ///
    /// To execute a query with all options left at their defaults the
    /// `query(&self, Query)` function might be more suitable.
    pub fn query_opt<T>(&self, new_cursor: NewCursor) -> FutureResult<Cursor<T>>
    where
        T: 'static + DeserializeOwned,
    {
        Box::new(
            self.connector
                .connection(&self.database_name)
                .execute(CreateCursor::new(new_cursor)),
        )
    }

    /// Creates a new graph in the database represented by this
    /// `DatabaseSession`.
    pub fn create_graph(&self, new_graph: NewGraph) -> FutureResult<GraphSession<C>> {
        let connector = self.connector.clone();
        let database_name = self.database_name.clone();
        Box::new(
            self.connector
                .connection(&self.database_name)
                .execute(CreateGraph::new(new_graph))
                .map(|graph| GraphSession::new(graph, database_name, connector)),
        )
    }
}

#[derive(Debug)]
pub struct GraphSession<C> {
    graph: Graph,
    database_name: String,
    connector: Rc<C>,
}

impl<C> GraphSession<C>
where
    C: 'static + Connector,
{
    fn new(graph: Graph, database_name: String, connector: Rc<C>) -> Self {
        GraphSession {
            graph,
            database_name,
            connector,
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}
