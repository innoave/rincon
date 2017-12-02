
use std::fmt::Debug;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use rincon_core::api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use rincon_core::arango::protocol::{FIELD_CODE, FIELD_COLLECTIONS, FIELD_EDGE,
    FIELD_GRAPH, FIELD_GRAPHS, FIELD_REMOVED, FIELD_VERTEX, PATH_API_GHARIAL,
    PATH_EDGE, PATH_VERTEX};
use document::{Document, DocumentHeader, DocumentId, DocumentKey, NewDocument,
    UpdatedDocumentHeader};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CreateGraph {
    graph: NewGraph,
}

impl CreateGraph {
    pub fn new(graph: NewGraph) -> Self {
        CreateGraph {
            graph,
        }
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

#[derive(Clone, Debug, PartialEq)]
pub struct DropGraph {
    name: String,
}

impl DropGraph {
    pub fn new<Name>(name: Name) -> Self
        where Name: Into<String>
    {
        DropGraph {
            name: name.into(),
        }
    }

    pub fn with_name<Name>(name: Name) -> Self
        where Name: Into<String>
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

#[derive(Clone, Debug, PartialEq)]
pub struct GetGraph {
    name: String,
}

impl GetGraph {
    pub fn new<Name>(name: Name) -> Self
        where Name: Into<String>
    {
        GetGraph {
            name: name.into(),
        }
    }

    pub fn with_name<Name>(name: Name) -> Self
        where Name: Into<String>
    {
        GetGraph::new(name)
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
#[derive(Clone, Debug, PartialEq)]
pub struct ListGraphs {}

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

#[derive(Clone, Debug, PartialEq)]
pub struct AddVertexCollection {
    graph_name: String,
    vertex_collection: VertexCollection,
}

impl AddVertexCollection {
    pub fn new<G>(graph_name: G, vertex_collection: VertexCollection) -> Self
        where G: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX
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

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveVertexCollection {
    graph_name: String,
    collection_name: String,
}

impl RemoveVertexCollection {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll) -> Self
        where G: Into<String>, Coll: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + &self.collection_name
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

#[derive(Clone, Debug, PartialEq)]
pub struct ListVertexCollections {
    graph_name: String,
}

impl ListVertexCollections {
    pub fn new<G>(graph_name: G) -> Self
        where G: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX
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

#[derive(Clone, Debug, PartialEq)]
pub struct InsertVertex<T> {
    graph_name: String,
    collection_name: String,
    vertex: NewDocument<T>,
}

impl<T> InsertVertex<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex: NewDocument<T>) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        InsertVertex {
            graph_name: graph_name.into(),
            collection_name: collection_name.into(),
            vertex,
        }
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
}

impl<T> Method for InsertVertex<T> {
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertVertex<T>
    where T: Serialize + Debug
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.vertex)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveVertex {
    graph_name: String,
    vertex_id: DocumentId,
}

impl RemoveVertex {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.deconstruct()),
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.deconstruct()),
        }
    }

    pub fn with_id<G>(graph_name: G, vertex_id: DocumentId) -> Self
        where G: Into<String>
    {
        RemoveVertex {
            graph_name: graph_name.into(),
            vertex_id,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + self.vertex_id.collection_name()
            + "/" + self.vertex_id.document_key()
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

#[derive(Clone, Debug, PartialEq)]
pub struct GetVertex<T> {
    graph_name: String,
    vertex_id: DocumentId,
    content: PhantomData<T>,
}

impl<T> GetVertex<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.deconstruct()),
            content: PhantomData,
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, vertex_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id: DocumentId::new(collection_name, vertex_key.deconstruct()),
            content: PhantomData,
        }
    }

    pub fn with_id<G>(graph_name: G, vertex_id: DocumentId) -> Self
        where G: Into<String>
    {
        GetVertex {
            graph_name: graph_name.into(),
            vertex_id,
            content: PhantomData,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn vertex_id(&self) -> &DocumentId {
        &self.vertex_id
    }
}

impl<T> Method for GetVertex<T>
    where T: DeserializeOwned
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + self.vertex_id.collection_name()
            + "/" + self.vertex_id.document_key()
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

#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceVertex<T> {
    graph_name: String,
    vertex_id: DocumentId,
    new_vertex: NewDocument<T>,
}

impl<T> ReplaceVertex<T> {
    pub fn new<G>(graph_name: G, vertex_id: DocumentId, new_vertex: NewDocument<T>) -> Self
        where G: Into<String>
    {
        ReplaceVertex {
            graph_name: graph_name.into(),
            vertex_id,
            new_vertex,
        }
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
}

impl<T> Method for ReplaceVertex<T> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for ReplaceVertex<T>
    where T: Serialize + Debug
{
    type Content = NewDocument<T>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + self.vertex_id.collection_name()
            + "/" + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_vertex)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModifyVertex<Upd> {
    graph_name: String,
    vertex_id: DocumentId,
    update: Upd,
}

impl<Upd> Method for ModifyVertex<Upd> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_VERTEX),
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd> Prepare for ModifyVertex<Upd>
    where Upd: Serialize
{
    type Content = Upd;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_VERTEX + "/" + self.vertex_id.collection_name()
            + "/" + self.vertex_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.update)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddEdgeDefinition {
    graph_name: String,
    edge_definition: EdgeDefinition,
}

impl AddEdgeDefinition {
    pub fn new<G>(graph_name: G, edge_definition: EdgeDefinition) -> Self
        where G: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name()
            + PATH_EDGE
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

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveEdgeDefinition {
    graph_name: String,
    edge_definition_name: String,
}

impl RemoveEdgeDefinition {
    pub fn new<G, E>(graph_name: G, edge_definition_name: E) -> Self
        where G: Into<String>, E: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + &self.edge_definition_name
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

#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceEdgeDefinition {
    graph_name: String,
    edge_definition_name: String,
    edge_definition: EdgeDefinition,
}

impl ReplaceEdgeDefinition {
    pub fn new<G, E>(graph_name: G, edge_definition_name: E, edge_definition: EdgeDefinition) -> Self
        where G: Into<String>, E: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + &self.edge_definition_name
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

#[derive(Clone, Debug, PartialEq)]
pub struct ListEdgeCollections {
    graph_name: String
}

impl ListEdgeCollections {
    pub fn new<G>(graph_name: G) -> Self
        where G: Into<String>
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE
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

#[derive(Clone, Debug, PartialEq)]
pub struct InsertEdge<T> {
    graph_name: String,
    collection_name: String,
    edge: NewEdge<T>,
}

impl<T> InsertEdge<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge: NewEdge<T>) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        InsertEdge {
            graph_name: graph_name.into(),
            collection_name: collection_name.into(),
            edge,
        }
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
}

impl<T> Method for InsertEdge<T> {
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertEdge<T>
    where T: Serialize + Debug
{
    type Content = NewEdge<T>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.edge)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RemoveEdge {
    graph_name: String,
    edge_id: DocumentId,
}

impl RemoveEdge {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        RemoveEdge::with_id(graph_name, DocumentId::new(
            collection_name,
            edge_key.deconstruct(),
        ))
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        RemoveEdge::new(graph_name, collection_name, edge_key)
    }

    pub fn with_id<G>(graph_name: G, edge_id: DocumentId) -> Self
        where G: Into<String>
    {
        RemoveEdge {
            graph_name: graph_name.into(),
            edge_id,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + self.edge_id.collection_name()
            + "/" + self.edge_id.document_key()
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

#[derive(Clone, Debug, PartialEq)]
pub struct GetEdge<T> {
    graph_name: String,
    edge_id: DocumentId,
    content: PhantomData<T>,
}

impl<T> GetEdge<T> {
    pub fn new<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        GetEdge {
            graph_name: graph_name.into(),
            edge_id: DocumentId::new(collection_name, edge_key.deconstruct()),
            content: PhantomData,
        }
    }

    pub fn with_key<G, Coll>(graph_name: G, collection_name: Coll, edge_key: DocumentKey) -> Self
        where G: Into<String>, Coll: Into<String>
    {
        GetEdge {
            graph_name: graph_name.into(),
            edge_id: DocumentId::new(collection_name, edge_key.deconstruct()),
            content: PhantomData,
        }
    }

    pub fn with_id<G>(graph_name: G, edge_id: DocumentId) -> Self
        where G: Into<String>
    {
        GetEdge {
            graph_name: graph_name.into(),
            edge_id,
            content: PhantomData,
        }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }

    pub fn edge_id(&self) -> &DocumentId {
        &self.edge_id
    }
}

impl<T> Method for GetEdge<T>
    where T: DeserializeOwned
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
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + self.edge_id.collection_name()
            + "/" + self.edge_id.document_key()
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

#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceEdge<T> {
    graph_name: String,
    edge_id: DocumentId,
    new_edge: NewEdge<T>,
}

impl<T> ReplaceEdge<T> {
    pub fn new<G, Coll>(graph_name: G, edge_id: DocumentId, new_edge: NewEdge<T>) -> Self
        where G: Into<String>
    {
        ReplaceEdge {
            graph_name: graph_name.into(),
            edge_id,
            new_edge,
        }
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
}

impl<T> Method for ReplaceEdge<T> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for ReplaceEdge<T>
    where T: Serialize + Debug
{
    type Content = NewEdge<T>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + self.edge_id.collection_name()
            + "/" + self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_edge)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModifyEdge<Upd> {
    graph_name: String,
    edge_id: DocumentId,
    update: Upd,
}

impl<Upd> ModifyEdge<Upd> {
    pub fn new<G>(graph_name: G, edge_id: DocumentId, update: Upd) -> Self
        where G: Into<String>
    {
        ModifyEdge {
            graph_name: graph_name.into(),
            edge_id,
            update,
        }
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
}

impl<Upd> Method for ModifyEdge<Upd> {
    type Result = UpdatedDocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_EDGE),
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd> Prepare for ModifyEdge<Upd>
    where Upd: Serialize
{
    type Content = Upd;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_GHARIAL) + "/" + &self.graph_name
            + PATH_EDGE + "/" + &self.edge_id.collection_name()
            + "/" + &self.edge_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.update)
    }
}
