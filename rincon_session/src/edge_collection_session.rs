use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

use rincon_client::document::types::{
    DocumentHeader, DocumentId, DocumentKey, UpdatedDocumentHeader,
};
use rincon_client::graph::methods::*;
use rincon_client::graph::types::{Edge, EdgeCollection, Graph, NewEdge};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare};

use super::Result;

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
where
    C: 'static + Connector,
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
    where
        M: 'static + Method + Prepare,
    {
        self.core.borrow_mut().run(
            self.connector
                .connection(&self.database_name)
                .execute(method),
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
    /// This will only remove the edge collection, the edge collections remain
    /// untouched.
    ///
    /// After calling this function the associated `EdgeCollectionSession` is
    /// no longer valid.
    pub fn drop(self) -> Result<Graph> {
        self.execute(RemoveEdgeDefinition::new(self.graph_name(), self.name()))
    }

    /// Creates a new edge in this collection.
    pub fn insert_edge<E, T>(&self, edge: E) -> Result<DocumentHeader>
    where
        E: Into<NewEdge<T>>,
        T: 'static + Serialize + Debug,
    {
        self.execute(InsertEdge::new(self.graph_name(), self.name(), edge.into()))
    }

    /// Creates a new edge in this collection and waits until the changes are
    /// written to disk.
    pub fn insert_edge_synced<E, T>(&self, edge: E) -> Result<DocumentHeader>
    where
        E: Into<NewEdge<T>>,
        T: 'static + Serialize + Debug,
    {
        self.execute(
            InsertEdge::new(self.graph_name(), self.name(), edge.into())
                .with_force_wait_for_sync(true),
        )
    }

    /// Fetches the edge with the given key from this collection.
    pub fn get_edge<T>(&self, key: DocumentKey) -> Result<Edge<T>>
    where
        T: 'static + DeserializeOwned,
    {
        self.execute(GetEdge::new(self.graph_name(), self.name(), key))
    }

    /// Fetches the edge with the given key from this collection if the
    /// revision matches the given predicate.
    pub fn get_edge_if_match<IfMatch, T>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<Edge<T>>
    where
        IfMatch: Into<String>,
        T: 'static + DeserializeOwned,
    {
        self.execute(GetEdge::new(self.graph_name(), self.name(), key).with_if_match(if_match))
    }

    /// Fetches the edge with the given key from this collection if the
    /// revision does not match the given predicate.
    pub fn get_edge_if_non_match<IfNonMatch, T>(
        &self,
        key: DocumentKey,
        if_non_match: IfNonMatch,
    ) -> Result<Edge<T>>
    where
        IfNonMatch: Into<String>,
        T: 'static + DeserializeOwned,
    {
        self.execute(
            GetEdge::new(self.graph_name(), self.name(), key).with_if_non_match(if_non_match),
        )
    }

    /// Replaces an existing edge with a new edge.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be replaced
    /// * `new_edge` : The new edge
    pub fn replace_edge<New, E>(
        &self,
        key: DocumentKey,
        new_edge: E,
    ) -> Result<UpdatedDocumentHeader>
    where
        New: 'static + Serialize + Debug,
        E: Into<NewEdge<New>>,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(ReplaceEdge::new(
            self.graph_name(),
            edge_id,
            new_edge.into(),
        ))
    }

    /// Replaces an existing edge with a new edge if the match condition
    /// is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be replaced
    /// * `new_edge` : The new edge
    /// * `if_match` : The match condition that must be met to replace a edge
    pub fn replace_edge_if_match<IfMatch, New, E>(
        &self,
        key: DocumentKey,
        new_edge: E,
        if_match: IfMatch,
    ) -> Result<UpdatedDocumentHeader>
    where
        IfMatch: Into<String>,
        New: 'static + Serialize + Debug,
        E: Into<NewEdge<New>>,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(
            ReplaceEdge::new(self.graph_name(), edge_id, new_edge.into()).with_if_match(if_match),
        )
    }

    /// Replaces an existing edge with a new edge if the match condition
    /// is met and waits until the changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be replaced
    /// * `new_edge` : The new edge
    /// * `if_match` : The match condition that must be met to replace a edge
    pub fn replace_edge_if_match_synced<IfMatch, New, E>(
        &self,
        key: DocumentKey,
        new_edge: E,
        if_match: IfMatch,
    ) -> Result<UpdatedDocumentHeader>
    where
        IfMatch: Into<String>,
        New: 'static + Serialize + Debug,
        E: Into<NewEdge<New>>,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(
            ReplaceEdge::new(self.graph_name(), edge_id, new_edge.into())
                .with_if_match(if_match)
                .with_force_wait_for_sync(true),
        )
    }

    /// Replaces an existing edge with a new edge and waits until the
    /// changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be replaced
    /// * `new_edge` : The new edge
    pub fn replace_edge_synced<New, E>(
        &self,
        key: DocumentKey,
        new_edge: E,
    ) -> Result<UpdatedDocumentHeader>
    where
        New: 'static + Serialize + Debug,
        E: Into<NewEdge<New>>,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        self.execute(
            ReplaceEdge::new(self.graph_name(), edge_id, new_edge.into())
                .with_force_wait_for_sync(true),
        )
    }

    /// Partially modifies an existing edge.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing edge if they do not exist yet or
    /// overwritten in the existing edge if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_edge<Upd>(
        &self,
        key: DocumentKey,
        update: Upd,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
    where
        Upd: 'static + Serialize + Debug,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        let modify_edge = if let Some(keep_none) = keep_none {
            ModifyEdge::new(self.graph_name(), edge_id, update).with_keep_none(keep_none)
        } else {
            ModifyEdge::new(self.graph_name(), edge_id, update)
        };
        self.execute(modify_edge)
    }

    /// Partially modifies an existing edge if the match condition is met.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing edge if they do not exist yet or
    /// overwritten in the existing edge if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `if_match` : The match condition that must be met to modify a edge
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_edge_if_match<IfMatch, Upd>(
        &self,
        key: DocumentKey,
        update: Upd,
        if_match: IfMatch,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
    where
        IfMatch: Into<String>,
        Upd: 'static + Serialize + Debug,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        let modify_edge = if let Some(keep_none) = keep_none {
            ModifyEdge::new(self.graph_name(), edge_id, update).with_keep_none(keep_none)
        } else {
            ModifyEdge::new(self.graph_name(), edge_id, update)
        };
        self.execute(modify_edge.with_if_match(if_match))
    }

    /// Partially modifies an existing edge if the match condition is met and
    /// waits until the changes are written to disk.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing edge if they do not exist yet or
    /// overwritten in the existing edge if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `if_match` : The match condition that must be met to modify a edge
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_edge_if_match_synced<IfMatch, Upd>(
        &self,
        key: DocumentKey,
        update: Upd,
        if_match: IfMatch,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
    where
        IfMatch: Into<String>,
        Upd: 'static + Serialize + Debug,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        let modify_edge = if let Some(keep_none) = keep_none {
            ModifyEdge::new(self.graph_name(), edge_id, update).with_keep_none(keep_none)
        } else {
            ModifyEdge::new(self.graph_name(), edge_id, update)
        };
        self.execute(
            modify_edge
                .with_if_match(if_match)
                .with_force_wait_for_sync(true),
        )
    }

    /// Partially modifies an existing edge and waits until the changes are
    /// written to disk.
    ///
    /// The update argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing edge if they do not exist yet or
    /// overwritten in the existing edge if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the edge to be modified
    /// * `update` : A document with the content to be modified (patch document)
    /// * `keep_none` : Whether values set to `None` shall be stored. By default
    ///   the attribute is not removed from the document.
    pub fn modify_edge_synced<Upd>(
        &self,
        key: DocumentKey,
        update: Upd,
        keep_none: Option<bool>,
    ) -> Result<UpdatedDocumentHeader>
    where
        Upd: 'static + Serialize + Debug,
    {
        let edge_id = DocumentId::new(self.name(), key.unwrap());
        let modify_edge = if let Some(keep_none) = keep_none {
            ModifyEdge::new(self.graph_name(), edge_id, update).with_keep_none(keep_none)
        } else {
            ModifyEdge::new(self.graph_name(), edge_id, update)
        };
        self.execute(modify_edge.with_force_wait_for_sync(true))
    }

    /// Removes the edge with the given key from this collection.
    pub fn remove_edge(&self, key: DocumentKey) -> Result<bool> {
        self.execute(RemoveEdge::new(self.graph_name(), self.name(), key))
    }

    /// Removes the edge with the given key from this collection if the match
    /// condition is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to remove the
    ///   edge
    pub fn remove_edge_if_match<IfMatch>(&self, key: DocumentKey, if_match: IfMatch) -> Result<bool>
    where
        IfMatch: Into<String>,
    {
        self.execute(RemoveEdge::new(self.graph_name(), self.name(), key).with_if_match(if_match))
    }

    /// Removes the edge with the given key from this collection if the match
    /// condition is met and waits until the changes are written to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to remove the
    ///   edge
    pub fn remove_edge_if_match_synced<IfMatch>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<bool>
    where
        IfMatch: Into<String>,
    {
        self.execute(
            RemoveEdge::new(self.graph_name(), self.name(), key)
                .with_if_match(if_match)
                .with_force_wait_for_sync(true),
        )
    }

    /// Removes the edge with the given key from this collection and waits
    /// until the changes are written to disk.
    pub fn remove_edge_synced(&self, key: DocumentKey) -> Result<bool> {
        self.execute(
            RemoveEdge::new(self.graph_name(), self.name(), key).with_force_wait_for_sync(true),
        )
    }
}
