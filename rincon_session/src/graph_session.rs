
use std::cell::RefCell;
use std::rc::Rc;

use tokio_core::reactor::Core;

use rincon_client::graph::methods::*;
use rincon_client::graph::types::Graph;
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
    /// of the graph or the `Graph` struct.
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
                GraphSession {
                    entity: Entity::Object(graph),
                    database_name: self.database_name,
                    connector: self.connector,
                    core: self.core,
                }
            )
    }
}
