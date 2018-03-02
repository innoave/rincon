
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
