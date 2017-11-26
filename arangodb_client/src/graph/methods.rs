
use arangodb_core::api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use arangodb_core::arango::protocol::{FIELD_CODE, FIELD_GRAPH, FIELD_GRAPHS,
    FIELD_REMOVED, PATH_API_GHARIAL};
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
pub struct DeleteGraph {
    name: String,
}

impl DeleteGraph {
    pub fn new<Name>(name: Name) -> Self
        where Name: Into<String>
    {
        DeleteGraph {
            name: name.into(),
        }
    }

    pub fn with_name<Name>(name: Name) -> Self
        where Name: Into<String>
    {
        DeleteGraph::new(name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Method for DeleteGraph {
    type Result = bool;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_REMOVED),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DeleteGraph {
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
