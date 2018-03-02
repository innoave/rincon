
use std::cell::RefCell;
use std::rc::Rc;

use tokio_core::reactor::Core;

use rincon_client::graph::methods::*;
use rincon_client::graph::types::{EdgeCollection, EdgeDefinition, Graph, VertexCollection};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::types::Entity;

use edge_collection_session::EdgeCollectionSession;
use vertex_collection_session::VertexCollectionSession;
use super::Result;

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
    /// Instantiates a new `GraphSession` for the given graph entity.
    pub(crate) fn new(
        entity: Entity<Graph>,
        database_name: String,
        connector: Rc<C>,
        core: Rc<RefCell<Core>>,
    ) -> Self {
        GraphSession {
            entity,
            database_name,
            connector,
            core,
        }
    }

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
    /// of the graph or a `Graph` instance.
    pub fn unwrap(self) -> Entity<Graph> {
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
                GraphSession::new(
                    Entity::Object(graph),
                    self.database_name,
                    self.connector,
                    self.core,
                )
            )
    }

    /// Drops the graph that is represented by this session.
    ///
    /// Returns true if the graph has been dropped successfully.
    ///
    /// After calling this function the associated `GraphSession` is no longer
    /// valid.
    pub fn drop(self) -> Result<bool> {
        self.execute(DropGraph::with_name(self.name()))
    }

    /// Adds a vertex collection of the given name to the set of collections
    /// of the graph represented by this session. If the collection does not
    /// exist if will be created.
    ///
    /// It returns a new `GraphSession` representing the updated graph.
    pub fn add_vertex_collection<N>(self, collection_name: N) -> Result<GraphSession<C>>
        where N: Into<String>
    {
        self.execute(AddVertexCollection::new(self.name(), VertexCollection::new(collection_name)))
            .map(|graph|
                GraphSession::new(
                    Entity::Object(graph),
                    self.database_name,
                    self.connector,
                    self.core,
                )
            )
    }

    /// Removes the vertex collection of the given name from the graph
    /// represented by this session and optionally deletes the collection if it
    /// is not used in any other graph.
    ///
    /// It returns a new `GraphSession` representing the updated graph.
    pub fn remove_vertex_collection<N>(self, collection_name: N) -> Result<GraphSession<C>>
        where N: Into<String>
    {
        self.execute(RemoveVertexCollection::new(self.name(), collection_name))
            .map(|graph|
                GraphSession::new(
                    Entity::Object(graph),
                    self.database_name,
                    self.connector,
                    self.core,
                )
            )
    }

    /// List all vertex collections used in the graph represented by this
    /// session.
    pub fn list_vertex_collections(&self) -> Result<Vec<String>> {
        self.execute(ListVertexCollections::new(self.name()))
    }

    /// Returns a new `VertexCollectionSession` for the vertex collection of the
    /// given name.
    pub fn use_vertex_collection<N>(&self, collection_name: N) -> VertexCollectionSession<C>
        where N: Into<String>
    {
        VertexCollectionSession::new(
            VertexCollection::new(collection_name),
            self.name().into(),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }

    /// Adds an edge definition to the graph represented by this session.
    ///
    /// # Arguments
    ///
    /// * `collection_name` : The name of the edge collection
    /// * `from` : One or many vertex collections that can contain source vertices
    /// * `to` : One or many vertex collections that can contain target vertices
    ///
    /// It returns a new `GraphSession` representing the updated graph.
    pub fn add_edge_definition<N, From, To>(
        self,
        collection_name: N,
        from: From,
        to: To
    ) -> Result<GraphSession<C>>
        where
            N: Into<String>,
            From: IntoIterator<Item=String>,
            To: IntoIterator<Item=String>,
    {
        self.execute(AddEdgeDefinition::new(self.name(), EdgeDefinition::new(
            collection_name,
            from,
            to,
        ))).map(|graph|
            GraphSession::new(
                Entity::Object(graph),
                self.database_name,
                self.connector,
                self.core,
            )
        )
    }

    /// Removes the edge definition of the given name from the graph represented
    /// by this session.
    ///
    /// This will only remove the edge collection, the vertex collections remain
    /// untouched.
    ///
    /// It returns a new `GraphSession` representing the updated graph.
    pub fn remove_edge_definition<N>(self, collection_name: N) -> Result<GraphSession<C>>
        where N: Into<String>
    {
        self.execute(RemoveEdgeDefinition::new(self.name(), collection_name))
            .map(|graph|
                GraphSession::new(
                    Entity::Object(graph),
                    self.database_name,
                    self.connector,
                    self.core,
                )
            )
    }

    /// List all edge collections used in the graph represented by this
    /// session.
    pub fn list_edge_definitions(&self) -> Result<Vec<String>> {
        self.execute(ListEdgeCollections::new(self.name()))
    }

    /// Returns a new `EdgeCollectionSession` for the edge collection of the
    /// given name.
    pub fn use_edge_collection<N>(&self, collection_name: N) -> EdgeCollectionSession<C>
        where N: Into<String>
    {
        EdgeCollectionSession::new(
            EdgeCollection::new(collection_name),
            self.name().into(),
            self.database_name.clone(),
            self.connector.clone(),
            self.core.clone(),
        )
    }
}
