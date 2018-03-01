
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

use rincon_client::document::types::{Document, DocumentHeader, DocumentKey, DocumentId,
    DocumentUpdate, NewDocument, UpdatedDocumentHeader};
use rincon_client::graph::methods::*;
use rincon_client::graph::types::{EdgeCollection, EdgeDefinition, Graph, VertexCollection};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};
use rincon_core::api::types::Entity;

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

/// A session for operating with a specific vertex collection.
#[derive(Debug)]
pub struct VertexCollectionSession<C> {
    entity: VertexCollection,
    graph_name: String,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> VertexCollectionSession<C>
    where C: 'static + Connector
{
    /// Instantiates a new `VertexCollectionSession` for the given vertex
    /// collection entity.
    pub(crate) fn new(
        entity: VertexCollection,
        graph_name: String,
        database_name: String,
        connector: Rc<C>,
        core: Rc<RefCell<Core>>,
    ) -> Self {
        VertexCollectionSession {
            entity,
            graph_name,
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

    /// Returns the name of the database this vertex collection is located in.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the name of the graph this vertex collection is part of.
    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Returns the name of the vertex collection this `VertexCollectionSession`
    /// operates with.
    pub fn name(&self) -> &str {
        self.entity.collection()
    }

    /// Returns the `VertexCollection` entity this `VertexCollectionSession`
    /// operates with.
    pub fn entity(&self) -> &VertexCollection {
        &self.entity
    }

    /// Unwraps the vertex collection entity out of this session.
    pub fn unwrap(self) -> VertexCollection {
        self.entity
    }

    /// Removes the vertex collection represented by this session from the graph
    /// and optionally deletes the collection if it is not used in any other
    /// graph.
    ///
    /// After calling this function the associated `VertexCollectionSession` is
    /// no longer valid.
    pub fn drop(self) -> Result<Graph> {
        self.execute(RemoveVertexCollection::new(self.graph_name(), self.name()))
    }

    /// Creates a new vertex in this collection.
    pub fn insert_vertex<V, T>(&self, vertex: V) -> Result<DocumentHeader>
        where
            V: Into<NewDocument<T>>,
            T: 'static + Serialize + Debug,
    {
        self.execute(InsertVertex::new(self.graph_name(), self.name(), vertex.into()))
    }

    /// Creates a new vertex in this collection and waits until the changes are
    /// written to disk.
    pub fn insert_vertex_synced<V, T>(&self, vertex: V) -> Result<DocumentHeader>
        where
            V: Into<NewDocument<T>>,
            T: 'static + Serialize + Debug,
    {
        self.execute(InsertVertex::new(self.graph_name(), self.name(), vertex.into())
            .with_force_wait_for_sync(true))
    }

    /// Fetches the vertex with the given key from this collection.
    pub fn get_vertex<T>(&self, key: DocumentKey) -> Result<Document<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(GetVertex::new(self.graph_name(), self.name(), key))
    }

    /// Fetches the vertex with the given key from this collection if the
    /// revision matches the given predicate.
    pub fn get_vertex_if_match<IfMatch, T>(&self, key: DocumentKey, if_match: IfMatch) -> Result<Document<T>>
        where
            IfMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetVertex::new(self.graph_name(), self.name(), key)
            .with_if_match(if_match))
    }

    /// Fetches the vertex with the given key from this collection if the
    /// revision does not match the given predicate.
    pub fn get_vertex_if_non_match<IfNonMatch, T>(&self, key: DocumentKey, if_non_match: IfNonMatch) -> Result<Document<T>>
        where
            IfNonMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetVertex::new(self.graph_name(), self.name(), key)
            .with_if_non_match(if_non_match))
    }

    /// Replaces an existing vertex with a new vertex.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be replaced
    /// * `new_vertex` : The new vertex
    pub fn replace_vertex<New, V>(
        &self,
        key: DocumentKey,
        new_vertex: V,
    ) -> Result<UpdatedDocumentHeader>
        where
            New: 'static + Serialize + Debug,
            V: Into<NewDocument<New>>,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(ReplaceVertex::new(self.graph_name(), vertex_id, new_vertex.into()))
    }

    /// Replaces an existing vertex with a new vertex if the match condition
    /// is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be replaced
    /// * `new_vertex` : The new vertex
    /// * `if_match` : The match condition that must be met to replace a vertex
    pub fn replace_vertex_if_match<IfMatch, New, V>(
        &self,
        key: DocumentKey,
        new_vertex: V,
        if_match: IfMatch,
    ) -> Result<UpdatedDocumentHeader>
        where
            IfMatch: Into<String>,
            New: 'static + Serialize + Debug,
            V: Into<NewDocument<New>>,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(ReplaceVertex::new(self.graph_name(), vertex_id, new_vertex.into())
            .with_if_match(if_match)
        )
    }

    /// Replaces an existing vertex with a new vertex if the match condition
    /// is met and waits until the changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be replaced
    /// * `new_vertex` : The new vertex
    /// * `if_match` : The match condition that must be met to replace a vertex
    pub fn replace_vertex_if_match_synced<IfMatch, New, V>(
        &self,
        key: DocumentKey,
        new_vertex: V,
        if_match: IfMatch,
    ) -> Result<UpdatedDocumentHeader>
        where
            IfMatch: Into<String>,
            New: 'static + Serialize + Debug,
            V: Into<NewDocument<New>>,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(ReplaceVertex::new(self.graph_name(), vertex_id, new_vertex.into())
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    /// Replaces an existing vertex with a new vertex and waits until the
    /// changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be replaced
    /// * `new_vertex` : The new vertex
    pub fn replace_vertex_synced<New, V>(
        &self,
        key: DocumentKey,
        new_vertex: V,
    ) -> Result<UpdatedDocumentHeader>
        where
            New: 'static + Serialize + Debug,
            V: Into<NewDocument<New>>,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(ReplaceVertex::new(self.graph_name(), vertex_id, new_vertex.into())
            .with_force_wait_for_sync(true)
        )
    }

    /// Partially modifies an existing vertex.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing vertex if they do not exist yet or
    /// overwritten in the existing vertex if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_vertex<Upd>(
        &self,
        key: DocumentKey,
        update: DocumentUpdate<Upd>,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
        where
            Upd: 'static + Serialize + Debug,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        let modify_vertex = if let Some(keep_none) = keep_none {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
                .with_keep_none(keep_none)
        } else {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
        };
        self.execute(modify_vertex)
    }

    /// Partially modifies an existing vertex if the match condition is met.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing vertex if they do not exist yet or
    /// overwritten in the existing vertex if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `if_match` : The match condition that must be met to modify a vertex
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_vertex_if_match<IfMatch, Upd>(
        &self,
        key: DocumentKey,
        update: DocumentUpdate<Upd>,
        if_match: IfMatch,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
        where
            IfMatch: Into<String>,
            Upd: 'static + Serialize + Debug,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        let modify_vertex = if let Some(keep_none) = keep_none {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
                .with_keep_none(keep_none)
        } else {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
        };
        self.execute(modify_vertex.with_if_match(if_match))
    }

    /// Partially modifies an existing vertex if the match condition is met and
    /// waits until the changes are written to disk.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing vertex if they do not exist yet or
    /// overwritten in the existing vertex if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `if_match` : The match condition that must be met to modify a vertex
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_vertex_if_match_synced<IfMatch, Upd>(
        &self,
        key: DocumentKey,
        update: DocumentUpdate<Upd>,
        if_match: IfMatch,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
        where
            IfMatch: Into<String>,
            Upd: 'static + Serialize + Debug,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        let modify_vertex = if let Some(keep_none) = keep_none {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
                .with_keep_none(keep_none)
        } else {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
        };
        self.execute(modify_vertex
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    /// Partially modifies an existing vertex and waits until the changes are
    /// written to disk.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing vertex if they do not exist yet or
    /// overwritten in the existing vertex if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the vertex to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_vertex_synced<Upd>(
        &self,
        key: DocumentKey,
        update: DocumentUpdate<Upd>,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
        where
            Upd: 'static + Serialize + Debug,
    {
        let vertex_id = DocumentId::new(self.name(), key.unwrap());
        let modify_vertex = if let Some(keep_none) = keep_none {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
                .with_keep_none(keep_none)
        } else {
            ModifyVertex::new(self.graph_name(), vertex_id, update)
        };
        self.execute(modify_vertex.with_force_wait_for_sync(true))
    }

    /// Removes the vertex with the given key from this collection.
    pub fn remove_vertex(&self, key: DocumentKey) -> Result<bool> {
        self.execute(RemoveVertex::new(self.graph_name(), self.name(), key))
    }

    /// Removes the vertex with the given key from this collection if the match
    /// condition is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to remove the
    ///   vertex
    pub fn remove_vertex_if_match<IfMatch>(&self, key: DocumentKey, if_match: IfMatch) -> Result<bool>
        where IfMatch: Into<String>
    {
        self.execute(RemoveVertex::new(self.graph_name(), self.name(), key)
            .with_if_match(if_match))
    }

    /// Removes the vertex with the given key from this collection if the match
    /// condition is met and waits until the changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to remove the
    ///   vertex
    pub fn remove_vertex_if_match_synced<IfMatch>(&self, key: DocumentKey, if_match: IfMatch) -> Result<bool>
        where IfMatch: Into<String>
    {
        self.execute(RemoveVertex::new(self.graph_name(), self.name(), key)
            .with_if_match(if_match)
            .with_force_wait_for_sync(true))
    }

    /// Removes the vertex with the given key from this collection and waits
    /// until the changes are written to disk.
    pub fn remove_vertex_synced(&self, key: DocumentKey) -> Result<bool> {
        self.execute(RemoveVertex::new(self.graph_name(), self.name(), key)
            .with_force_wait_for_sync(true))
    }
}

/// A session for operating with a specific edge collection.
#[derive(Debug)]
pub struct EdgeCollectionSession<C> {
    entity: EdgeCollection,
    graph_name: String,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> EdgeCollectionSession<C>
    where C: 'static + Connector
{
    /// Instantiates a new `EdgeCollectionSession` for the given edge
    /// collection entity.
    pub(crate) fn new(
        entity: EdgeCollection,
        graph_name: String,
        database_name: String,
        connector: Rc<C>,
        core: Rc<RefCell<Core>>,
    ) -> Self {
        EdgeCollectionSession {
            entity,
            graph_name,
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

    /// Returns the name of the database this edge collection is located in.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the name of the graph this edge collection is part of.
    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    /// Returns the name of the edge collection this `EdgeCollectionSession`
    /// operates with.
    pub fn name(&self) -> &str {
        self.entity.collection()
    }

    /// Returns the `EdgeCollection` entity this `EdgeCollectionSession`
    /// operates with.
    pub fn entity(&self) -> &EdgeCollection {
        &self.entity
    }

    /// Unwraps the edge collection entity out of this session.
    pub fn unwrap(self) -> EdgeCollection {
        self.entity
    }

    /// Removes the edge definition represented by this session from the graph
    ///
    /// This will only remove the edge collection, the vertex collections remain
    /// untouched.
    ///
    /// After calling this function the associated `EdgeCollectionSession` is
    /// no longer valid.
    pub fn drop(self) -> Result<Graph> {
        self.execute(RemoveEdgeDefinition::new(self.graph_name(), self.name()))
    }
}
