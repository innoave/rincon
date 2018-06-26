//! Methods for managing graphs.

use std::fmt::Debug;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use super::types::*;
use document::types::{
    Document, DocumentHeader, DocumentId, DocumentKey, NewDocument, UpdatedDocumentHeader,
};
use rincon_core::api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use rincon_core::arango::protocol::{
    FIELD_CODE, FIELD_COLLECTIONS, FIELD_EDGE, FIELD_GRAPH, FIELD_GRAPHS, FIELD_REMOVED,
    FIELD_VERTEX, HEADER_IF_MATCH, HEADER_IF_NON_MATCH, PARAM_KEEP_NULL, PARAM_WAIT_FOR_SYNC,
    PATH_API_GHARIAL, PATH_EDGE, PATH_VERTEX,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateGraph {
    graph: NewGraph,
}

impl CreateGraph {
    pub fn new(graph: NewGraph) -> Self {
        CreateGraph { graph }
    }

    pub fn graph(&self) -> &NewGraph {
        &self.graph
    }
}

impl Method for CreateGraph {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for CreateGraph {
    type Content = NewGraph;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.graph)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DropGraph {
    name: String,
}

impl DropGraph {
    pub fn new<Name>(name: Name) -> Self
    where
        Name: Into<String>,
    {
        DropGraph { name: name.into() }
    }

    pub fn with_name<Name>(name: Name) -> Self
    where
        Name: Into<String>,
    {
        DropGraph::new(name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for DropGraph {
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_REMOVED),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DropGraph {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetGraph {
    name: String,
}

impl GetGraph {
    pub fn new(name: String) -> Self {
        GetGraph { name }
    }

    pub fn with_name<Name>(name: Name) -> Self
    where
        Name: Into<String>,
    {
        GetGraph::new(name.into())
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for GetGraph {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetGraph {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub struct ListGraphs {}

#[cfg_attr(feature = "cargo-clippy", allow(new_without_default_derive))]
impl ListGraphs {
    pub fn new() -> Self {
        ListGraphs {}
    }
}

impl Method for ListGraphs {
    type Result = Vec<Graph>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPHS),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ListGraphs {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddVertexCollection {
    graph_name: String,
    vertex_collection: VertexCollection,
}

impl AddVertexCollection {
    pub fn new<G>(graph_name: G, vertex_collection: VertexCollection) -> Self
    where
        G: Into<String>,
    {
        AddVertexCollection {
            graph_name: graph_name.into(),
            vertex_collection,
        }
    }

    pub fn vertex_collection(&self) -> &VertexCollection {
        &self.vertex_collection
    }
}

impl Method for AddVertexCollection {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for AddVertexCollection {
    type Content = VertexCollection;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name + PATH_VERTEX
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.vertex_collection)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveVertexCollection {
    graph_name: String,
    collection_name: String,
}

impl RemoveVertexCollection {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        RemoveVertexCollection {
            graph_name: graph_name.into(),
            collection_name: collection_name.into(),
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }
}

impl Method for RemoveVertexCollection {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RemoveVertexCollection {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListVertexCollections {
    graph_name: String,
}

impl ListVertexCollections {
    pub fn new<G>(graph_name: G) -> Self
    where
        G: Into<String>,
    {
        ListVertexCollections {
            graph_name: graph_name.into(),
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

impl Method for ListVertexCollections {
    type Result = Vec<String>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_COLLECTIONS),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ListVertexCollections {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name + PATH_VERTEX
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertVertex<T> {
    graph_name: String,
    collection_name: String,
    vertex: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertVertex<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex: NewDocument<T>) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        InsertVertex {
            graph_name: graph_name.into(),
            collection_name: collection_name.into(),
            vertex,
            force_wait_for_sync: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn vertex(&self) -> &NewDocument<T> {
        &self.vertex
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertVertex<T> {
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertVertex<T>
where
    T: Serialize + Debug,
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.vertex)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveVertex {
    graph_name: String,
    vertex_id: DocumentId,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl RemoveVertex {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.unwrap()),
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.unwrap()),
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_id<G>(graph_name: G, vertex_id: DocumentId) -> Self
    where
        G: Into<String>,
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl Method for RemoveVertex {
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_REMOVED),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RemoveVertex {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + self.vertex_id.collection_name()
            + "/"
            + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetVertex<T> {
    graph_name: String,
    vertex_id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
    content: PhantomData<T>,
}

impl<T> GetVertex<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.unwrap()),
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.unwrap()),
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_id<G>(graph_name: G, vertex_id: DocumentId) -> Self
    where
        G: Into<String>,
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id,
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn with_if_non_match<IfNonMatch>(mut self, if_non_match: IfNonMatch) -> Self
    where
        IfNonMatch: Into<String>,
    {
        self.if_non_match = Some(if_non_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl<T> Method for GetVertex<T>
where
    T: DeserializeOwned,
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_REMOVED),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for GetVertex<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + self.vertex_id.collection_name()
            + "/"
            + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        if let Some(ref if_non_match) = self.if_non_match {
            header.insert(HEADER_IF_NON_MATCH, if_non_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceVertex<T> {
    graph_name: String,
    vertex_id: DocumentId,
    new_vertex: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl<T> ReplaceVertex<T> {
    pub fn new<G>(graph_name: G, vertex_id: DocumentId, new_vertex: NewDocument<T>) -> Self
    where
        G: Into<String>,
    {
        ReplaceVertex {
            graph_name: graph_name.into(),
            vertex_id,
            new_vertex,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
    }

    pub fn new_vertex(&self) -> &NewDocument<T> {
        &self.new_vertex
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl<T> Method for ReplaceVertex<T> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for ReplaceVertex<T>
where
    T: Serialize + Debug,
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + self.vertex_id.collection_name()
            + "/"
            + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_vertex)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyVertex<Upd> {
    graph_name: String,
    vertex_id: DocumentId,
    update: Upd,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
    keep_none: Option<bool>,
}

impl<Upd> ModifyVertex<Upd> {
    pub fn new<G>(graph_name: G, vertex_id: DocumentId, update: Upd) -> Self
    where
        G: Into<String>,
    {
        ModifyVertex {
            graph_name: graph_name.into(),
            vertex_id,
            update,
            force_wait_for_sync: None,
            if_match: None,
            keep_none: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn with_keep_none(mut self, keep_none: bool) -> Self {
        self.keep_none = Some(keep_none);
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
    }

    pub fn update(&self) -> &Upd {
        &self.update
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn is_keep_none(&self) -> Option<bool> {
        self.keep_none
    }
}

impl<Upd> Method for ModifyVertex<Upd> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd> Prepare for ModifyVertex<Upd>
where
    Upd: Serialize + Debug,
{
    type Content = Upd;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_VERTEX
            + "/"
            + self.vertex_id.collection_name()
            + "/"
            + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(keep_none) = self.keep_none {
            params.insert(PARAM_KEEP_NULL, keep_none);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.update)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddEdgeDefinition {
    graph_name: String,
    edge_definition: EdgeDefinition,
}

impl AddEdgeDefinition {
    pub fn new<G>(graph_name: G, edge_definition: EdgeDefinition) -> Self
    where
        G: Into<String>,
    {
        AddEdgeDefinition {
            graph_name: graph_name.into(),
            edge_definition,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_definition(&self) -> &EdgeDefinition {
        &self.edge_definition
    }
}

impl Method for AddEdgeDefinition {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for AddEdgeDefinition {
    type Content = EdgeDefinition;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + self.graph_name() + PATH_EDGE
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.edge_definition)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveEdgeDefinition {
    graph_name: String,
    edge_definition_name: String,
}

impl RemoveEdgeDefinition {
    pub fn new<G, E>(graph_name: G, edge_definition_name: E) -> Self
    where
        G: Into<String>,
        E: Into<String>,
    {
        RemoveEdgeDefinition {
            graph_name: graph_name.into(),
            edge_definition_name: edge_definition_name.into(),
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_definition_name(&self) -> &str {
        &self.edge_definition_name
    }
}

impl Method for RemoveEdgeDefinition {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RemoveEdgeDefinition {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + &self.edge_definition_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceEdgeDefinition {
    graph_name: String,
    edge_definition_name: String,
    edge_definition: EdgeDefinition,
}

impl ReplaceEdgeDefinition {
    pub fn new<G, E>(
        graph_name: G,
        edge_definition_name: E,
        edge_definition: EdgeDefinition,
    ) -> Self
    where
        G: Into<String>,
        E: Into<String>,
    {
        ReplaceEdgeDefinition {
            graph_name: graph_name.into(),
            edge_definition_name: edge_definition_name.into(),
            edge_definition,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_definition_name(&self) -> &str {
        &self.edge_definition_name
    }

    pub fn edge_definition(&self) -> &EdgeDefinition {
        &self.edge_definition
    }
}

impl Method for ReplaceEdgeDefinition {
    type Result = Graph;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_GRAPH),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ReplaceEdgeDefinition {
    type Content = EdgeDefinition;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + &self.edge_definition_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.edge_definition)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListEdgeCollections {
    graph_name: String,
}

impl ListEdgeCollections {
    pub fn new<G>(graph_name: G) -> Self
    where
        G: Into<String>,
    {
        ListEdgeCollections {
            graph_name: graph_name.into(),
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}

impl Method for ListEdgeCollections {
    type Result = Vec<String>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_COLLECTIONS),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ListEdgeCollections {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name + PATH_EDGE
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertEdge<T> {
    graph_name: String,
    collection_name: String,
    edge: NewEdge<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertEdge<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge: NewEdge<T>) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        InsertEdge {
            graph_name: graph_name.into(),
            collection_name: collection_name.into(),
            edge,
            force_wait_for_sync: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn edge(&self) -> &NewEdge<T> {
        &self.edge
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertEdge<T> {
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertEdge<T>
where
    T: Serialize + Debug,
{
    type Content = NewEdge<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.edge)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveEdge {
    graph_name: String,
    edge_id: DocumentId,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl RemoveEdge {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        RemoveEdge::with_id(
            graph_name,
            DocumentId::new(collection_name, edge_key.unwrap()),
        )
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        RemoveEdge::new(graph_name, collection_name, edge_key)
    }

    pub fn with_id<G>(graph_name: G, edge_id: DocumentId) -> Self
    where
        G: Into<String>,
    {
        RemoveEdge {
            graph_name: graph_name.into(),
            edge_id,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl Method for RemoveEdge {
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_REMOVED),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for RemoveEdge {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + self.edge_id.collection_name()
            + "/"
            + self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetEdge<T> {
    graph_name: String,
    edge_id: DocumentId,
    content: PhantomData<T>,
    if_match: Option<String>,
    if_non_match: Option<String>,
}

impl<T> GetEdge<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        GetEdge {
            graph_name: graph_name.into(),
            edge_id: DocumentId::new(collection_name, edge_key.unwrap()),
            content: PhantomData,
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
    where
        G: Into<String>,
        Coll: Into<String>,
    {
        GetEdge::new(graph_name, collection_name, edge_key)
    }

    pub fn with_id<G>(graph_name: G, edge_id: DocumentId) -> Self
    where
        G: Into<String>,
    {
        GetEdge {
            graph_name: graph_name.into(),
            edge_id,
            content: PhantomData,
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn with_if_non_match<IfNonMatch>(mut self, if_non_match: IfNonMatch) -> Self
    where
        IfNonMatch: Into<String>,
    {
        self.if_non_match = Some(if_non_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl<T> Method for GetEdge<T>
where
    T: DeserializeOwned,
{
    type Result = Edge<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for GetEdge<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + self.edge_id.collection_name()
            + "/"
            + self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        if let Some(ref if_non_match) = self.if_non_match {
            header.insert(HEADER_IF_NON_MATCH, if_non_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceEdge<T> {
    graph_name: String,
    edge_id: DocumentId,
    new_edge: NewEdge<T>,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl<T> ReplaceEdge<T> {
    pub fn new<G>(graph_name: G, edge_id: DocumentId, new_edge: NewEdge<T>) -> Self
    where
        G: Into<String>,
    {
        ReplaceEdge {
            graph_name: graph_name.into(),
            edge_id,
            new_edge,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
    }

    pub fn new_edge(&self) -> &NewEdge<T> {
        &self.new_edge
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl<T> Method for ReplaceEdge<T> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for ReplaceEdge<T>
where
    T: Serialize + Debug,
{
    type Content = NewEdge<T>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + self.edge_id.collection_name()
            + "/"
            + self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_edge)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyEdge<Upd> {
    graph_name: String,
    edge_id: DocumentId,
    update: Upd,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
    keep_none: Option<bool>,
}

impl<Upd> ModifyEdge<Upd> {
    pub fn new<G>(graph_name: G, edge_id: DocumentId, update: Upd) -> Self
    where
        G: Into<String>,
    {
        ModifyEdge {
            graph_name: graph_name.into(),
            edge_id,
            update,
            force_wait_for_sync: None,
            if_match: None,
            keep_none: None,
        }
    }

    pub fn with_force_wait_for_sync(mut self, force_wait_for_sync: bool) -> Self {
        self.force_wait_for_sync = Some(force_wait_for_sync);
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
    where
        IfMatch: Into<String>,
    {
        self.if_match = Some(if_match.into());
        self
    }

    pub fn with_keep_none(mut self, keep_none: bool) -> Self {
        self.keep_none = Some(keep_none);
        self
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
    }

    pub fn update(&self) -> &Upd {
        &self.update
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn is_keep_none(&self) -> Option<bool> {
        self.keep_none
    }
}

impl<Upd> Method for ModifyEdge<Upd> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd> Prepare for ModifyEdge<Upd>
where
    Upd: Serialize,
{
    type Content = Upd;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL)
            + "/"
            + &self.graph_name
            + PATH_EDGE
            + "/"
            + self.edge_id.collection_name()
            + "/"
            + self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(keep_none) = self.keep_none {
            params.insert(PARAM_KEEP_NULL, keep_none);
        }
        params
    }

    fn header(&self) -> Parameters {
        let mut header = Parameters::new();
        if let Some(ref if_match) = self.if_match {
            header.insert(HEADER_IF_MATCH, if_match.to_owned());
        }
        header
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.update)
    }
}
