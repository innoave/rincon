use std::cell::RefCell;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use tokio_core::reactor::Core;

use rincon_client::aql::methods::{ExplainQuery, ParseQuery};
use rincon_client::aql::types::{ExplainOptions, ExplainedQuery, ParsedQuery};
use rincon_client::collection::methods::{CreateCollection, DropCollection, ListCollections};
use rincon_client::collection::types::Collection;
use rincon_client::cursor::methods::CreateCursor;
use rincon_client::cursor::types::NewCursor;
use rincon_client::database::methods::DropDatabase;
use rincon_client::document::methods::GetDocument;
use rincon_client::document::types::{Document, DocumentId};
use rincon_client::graph::methods::{CreateGraph, DropGraph, ListGraphs};
use rincon_client::graph::types::{Graph, NewGraph};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::query::Query;
use rincon_core::api::types::Entity;

use super::Result;
use collection_session::CollectionSession;
use cursor_session::CursorSession;
use graph_session::GraphSession;

/// A session for operating with a specific database.
#[derive(Debug)]
pub struct DatabaseSession<C> {
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> DatabaseSession<C>
where
    C: 'static + Connector,
{
    /// Instantiates a new `DatabaseSession` for the database with the given
    /// name.
    pub(crate) fn new(database_name: String, connector: Rc<C>, core: Rc<RefCell<Core>>) -> Self {
        DatabaseSession {
            database_name,
            connector,
            core,
        }
    }

    /// Executes an API method applied to the database of this session.
    pub fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
    where
        M: 'static + Method + Prepare,
    {
        self.core.borrow_mut().run(
            self.connector
                .connection(&self.database_name)
                .execute(method),
        )
    }

    /// Returns the name of the database this `DatabaseSession` operates with.
    pub fn name(&self) -> &str {
        &self.database_name
    }

    /// Unwraps the database name out of this session.
    pub fn unwrap(self) -> String {
        self.database_name
    }

    /// Drops the database that is used in this session.
    ///
    /// Returns true if the database has been dropped successfully.
    ///
    /// After calling this function the associated `DatabaseSession` is no
    /// longer valid.
    pub fn drop(self) -> Result<bool> {
        let database_name = self.database_name.to_owned();
        self.core.borrow_mut().run(
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
    pub fn query<T>(&self, query: Query) -> Result<CursorSession<T, C>>
    where
        T: 'static + DeserializeOwned,
    {
        self.execute(CreateCursor::from_query(query)).map(|cursor| {
            CursorSession::new(
                cursor,
                self.database_name.clone(),
                self.connector.clone(),
                self.core.clone(),
            )
        })
    }

    /// Executes a query and returns a cursor with the first result set.
    ///
    /// It requires a `NewCursor` struct as a parameter which allows full
    /// control over all supported cursor options and query execution options.
    ///
    /// To execute a query with all options left at their defaults the
    /// `query(&self, Query)` function might be more suitable.
    pub fn query_opt<T>(&self, new_cursor: NewCursor) -> Result<CursorSession<T, C>>
    where
        T: 'static + DeserializeOwned,
    {
        self.execute(CreateCursor::new(new_cursor)).map(|cursor| {
            CursorSession::new(
                cursor,
                self.database_name.clone(),
                self.connector.clone(),
                self.core.clone(),
            )
        })
    }

    /// Generates an execution plan for a query but does not execute it.
    pub fn explain_query(&self, query: Query) -> Result<ExplainedQuery> {
        self.execute(ExplainQuery::with_defaults(query))
    }

    /// Generates an execution plan for a query but does not execute it.
    ///
    /// Some options about how many execution plans are generated and the
    /// configuration options for the query optimizer can be provided.
    pub fn explain_query_opt(
        &self,
        query: Query,
        options: ExplainOptions,
    ) -> Result<ExplainedQuery> {
        self.execute(ExplainQuery::with_options(query, options))
    }

    /// Parses a query a validates the syntax but does not execute it.
    ///
    /// If the query can be parsed without error the abstract syntax tree (AST)
    /// of the query is returned.
    pub fn parse_query<Q>(&self, query: Q) -> Result<ParsedQuery>
    where
        Q: Into<String>,
    {
        self.execute(ParseQuery::from_query(query.into()))
    }

    /// Fetch the document with the given id from the database of this session.
    pub fn get_document<T>(&self, id: DocumentId) -> Result<Document<T>>
    where
        T: 'static + DeserializeOwned,
    {
        self.execute(GetDocument::with_id(id))
    }

    /// Returns a new `CollectionSession` for the collection with the given
    /// name.
    pub fn use_collection_with_name<N>(&self, collection_name: N) -> CollectionSession<C>
    where
        N: Into<String>,
    {
        CollectionSession::new(
            Entity::Name(collection_name.into()),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Returns a new `CollectionSession` for the given collection.
    pub fn use_collection(&self, collection: Collection) -> CollectionSession<C> {
        CollectionSession::new(
            Entity::Object(collection),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Creates a new collection within the database of this session.
    pub fn create_collection<N>(&self, collection_name: N) -> Result<CollectionSession<C>>
    where
        N: Into<String>,
    {
        self.execute(CreateCollection::with_name(collection_name))
            .map(|props| {
                CollectionSession::new(
                    Entity::Object(Collection::from(props)),
                    self.database_name.clone(),
                    self.connector.clone(),
                    self.core.clone(),
                )
            })
    }

    /// Drops the collection with the given name from the database of this
    /// session and returns the identifier of the dropped collection.
    pub fn drop_collection<N>(&self, collection_name: N) -> Result<String>
    where
        N: Into<String>,
    {
        self.execute(DropCollection::with_name(collection_name))
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
    where
        N: Into<String>,
    {
        GraphSession::new(
            Entity::Name(graph_name.into()),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Returns a new `GraphSession` for the given graph.
    pub fn use_graph(&self, graph: Graph) -> GraphSession<C> {
        GraphSession::new(
            Entity::Object(graph),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Creates a new graph in the database represented by this
    /// `DatabaseSession`.
    pub fn create_graph(&self, new_graph: NewGraph) -> Result<GraphSession<C>> {
        let core = self.core.clone();
        let connector = self.connector.clone();
        let database_name = self.database_name.clone();
        self.execute(CreateGraph::new(new_graph))
            .map(|graph| GraphSession::new(Entity::Object(graph), database_name, connector, core))
    }

    /// Drops the graph with the given name from the database of this session.
    ///
    /// This function returns true if the graph has been deleted and false
    /// otherwise.
    pub fn drop_graph<N>(&self, graph_name: N) -> Result<bool>
    where
        N: Into<String>,
    {
        self.execute(DropGraph::with_name(graph_name))
    }

    /// Fetches a list of all graphs in this database.
    pub fn list_graphs(&self) -> Result<Vec<Graph>> {
        self.execute(ListGraphs::new())
    }
}
