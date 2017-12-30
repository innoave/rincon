
use std::cell::RefCell;
use std::rc::Rc;

use futures::Future;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

pub use rincon_core::api::connector::{Connector, Error};
pub use rincon_core::api::query::Query;
pub use rincon_core::api::types::Empty;
pub use rincon_client::cursor::types::{Cursor, NewCursor};
pub use rincon_client::database::types::{Database, NewDatabase};
pub use rincon_client::graph::types::{EdgeDefinition, Graph, NewGraph};
pub use rincon_client::user::types::{NewUser, UserExtra};

use rincon_core::api::connector::Execute;
use rincon_core::arango::protocol::SYSTEM_DATABASE;
use rincon_client::cursor::methods::CreateCursor;
use rincon_client::database::methods::{CreateDatabase, DropDatabase};
use rincon_client::graph::methods::CreateGraph;

#[derive(Debug)]
pub struct ArangoSession<C> {
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> ArangoSession<C>
    where C: 'static + Connector
{
    pub fn new(connector: C, core: Core) -> Result<Self, Error> {
        Ok(ArangoSession {
            connector: Rc::new(connector),
            core: Rc::new(RefCell::new(core)),
        })
    }

    pub fn close(self) {
        //TODO see if a close() method has any purpose
    }

    pub fn use_system_database(&self) -> Result<DatabaseSession<C>, Error> {
        DatabaseSession::new(SYSTEM_DATABASE.to_owned(), self.connector.clone(), self.core.clone())
    }

    pub fn use_database<DbName>(&self, database_name: DbName) -> Result<DatabaseSession<C>, Error>
        where DbName: Into<String>
    {
        DatabaseSession::new(database_name.into(), self.connector.clone(), self.core.clone())
    }

    pub fn create_database<UserInfo>(&self, new_database: NewDatabase<UserInfo>) -> Result<DatabaseSession<C>, Error>
        where UserInfo: UserExtra + Serialize + 'static
    {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = new_database.name().to_owned();
        self.core.borrow_mut().run(self.connector.system_connection()
            .execute(CreateDatabase::new(new_database))
                .and_then(move |_| DatabaseSession::new(database_name, connector, core))
        )
    }
}

#[derive(Debug)]
pub struct DatabaseSession<C> {
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> DatabaseSession<C>
    where C: 'static + Connector
{
    fn new(database_name: String, connector: Rc<C>, core: Rc<RefCell<Core>>) -> Result<Self, Error> {
        Ok(DatabaseSession {
            database_name,
            connector,
            core,
        })
    }

    pub fn name(&self) -> &str {
        &self.database_name
    }

    /// Drops the database that is used in this session.
    ///
    /// After calling this function the associated `DatabaseSession` is no
    /// longer valid.
    pub fn drop(self) -> Result<bool, Error> {
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
    pub fn query<T>(&self, query: Query) -> Result<Cursor<T>, Error>
        where T: 'static + DeserializeOwned
    {
        self.core.borrow_mut().run(self.connector.connection(&self.database_name)
            .execute(CreateCursor::from_query(query))
        )
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// It requires a `NewCursor` struct as a parameter which allows full
    /// control over all supported cursor options and query execution options.
    ///
    /// To execute a query with all options left at their defaults the
    /// `query(&self, Query)` function might be more suitable.
    pub fn query_opt<T>(&self, new_cursor: NewCursor) -> Result<Cursor<T>, Error>
        where T: 'static + DeserializeOwned
    {
        self.core.borrow_mut().run(self.connector.connection(&self.database_name)
            .execute(CreateCursor::new(new_cursor))
        )
    }

    /// Creates a new graph in the database represented by this
    /// `DatabaseSession`.
    pub fn create_graph(&self, new_graph: NewGraph) -> Result<GraphSession<C>, Error> {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = self.database_name.clone();
        self.core.borrow_mut().run(self.connector.connection(&self.database_name)
            .execute(CreateGraph::new(new_graph))
                .and_then(|graph| GraphSession::new(graph, database_name, connector, core))
        )
    }
}

#[derive(Debug)]
pub struct GraphSession<C> {
    graph: Graph,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> GraphSession<C>
    where C: 'static + Connector
{
    fn new(graph: Graph, database_name: String, connector: Rc<C>, core: Rc<RefCell<Core>>) -> Result<Self, Error> {
        Ok(GraphSession {
            graph,
            database_name,
            connector,
            core,
        })
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}
