
use std::sync::Arc;

use futures::Future;
use serde::ser::Serialize;

use rincon_core::api::connector::{Error, Execute, UseDatabase};
use rincon_core::arango::protocol::SYSTEM_DATABASE;
use rincon_client::database::*;
use rincon_client::graph::*;
use rincon_client::user::*;

pub type FutureResult<T> = Box<Future<Item=T, Error=Error>>;

#[derive(Clone, Debug)]
pub struct ArangoSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    connector: Arc<Connector>,
}

impl<Connector> ArangoSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    pub fn new(connector: Connector) -> Self {
        ArangoSession {
            connector: Arc::new(connector),
        }
    }

    pub fn close(self) {
    }

    pub fn use_system_database(&self) -> DatabaseSession<Connector> {
        let connector = Arc::new(self.connector.use_database(SYSTEM_DATABASE));
        DatabaseSession::new(connector)
    }

    pub fn use_database<DbName>(&self, database_name: DbName) -> DatabaseSession<Connector>
        where DbName: Into<String>
    {
        let connector = Arc::new(self.connector.use_database(database_name));
        DatabaseSession::new(connector)
    }

    pub fn create_database<UserInfo>(&self, new_database: NewDatabase<UserInfo>) -> FutureResult<DatabaseSession<Connector>>
        where UserInfo: UserExtra + Serialize + 'static
    {
        let connector = self.connector.clone();
        let database_name = new_database.name().to_owned();
        Box::new(self.connector.execute(CreateDatabase::new(new_database))
            .map(move |_| DatabaseSession::new(Arc::new(connector.use_database(database_name)))))
    }
}

#[derive(Clone, Debug)]
pub struct DatabaseSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    connector: Arc<Connector>,
}

impl<Connector> DatabaseSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    fn new(connector: Arc<Connector>) -> Self {
        DatabaseSession {
            connector,
        }
    }

    pub fn name(&self) -> &str {
        self.connector.database_name()
            .expect("A DatabaseSession should always use an explicit database")
    }

    /// Drops the database that is used in this session.
    ///
    /// After calling this function the associated `DatabaseSession` is no
    /// longer valid.
    pub fn drop(self) -> FutureResult<bool> {
        let database_name = self.name().to_owned();
        Box::new(self.connector.execute(DropDatabase::new(database_name)))
    }

    /// Creates a new graph in the database represented by this
    /// `DatabaseSession`.
    pub fn create_graph(&self, new_graph: NewGraph) -> FutureResult<GraphSession<Connector>> {
        let connector = self.connector.clone();
        Box::new(self.connector.execute(CreateGraph::new(new_graph))
            .map(|graph| GraphSession::new(graph, connector)))
    }
}

#[derive(Clone, Debug)]
pub struct GraphSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    graph: Graph,
    connector: Arc<Connector>,
}

impl<Connector> GraphSession<Connector>
    where Connector: 'static + Execute + UseDatabase
{
    fn new(graph: Graph, connector: Arc<Connector>) -> Self {
        GraphSession {
            graph,
            connector,
        }
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}
