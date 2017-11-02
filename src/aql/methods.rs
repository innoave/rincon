
use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use api::query::Query;
use arango::protocol::{FIELD_CODE, PATH_API_EXPLAIN, PATH_API_QUERY};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ParseQuery {
    options: NewParseQuery,
}

impl ParseQuery {
    pub fn new(options: NewParseQuery) -> Self {
        ParseQuery {
            options,
        }
    }

    pub fn from_query(query: String) -> Self {
        ParseQuery::new(NewParseQuery::new(query))
    }

    pub fn options(&self) -> &NewParseQuery {
        &self.options
    }
}

impl Method for ParseQuery {
    type Result = ParsedQuery;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ParseQuery {
    type Content = NewParseQuery;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_QUERY)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.options)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplainQuery {
    query_options: NewExplainQuery,
}

impl ExplainQuery {
    pub fn new(query_options: NewExplainQuery) -> Self {
        ExplainQuery {
            query_options,
        }
    }

    pub fn from_query(query: Query) -> Self {
        ExplainQuery::new(NewExplainQuery::new(query))
    }

    pub fn query_options(&self) -> &NewExplainQuery {
        &self.query_options
    }
}

impl Method for ExplainQuery {
    type Result = ExplainedQuery;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for ExplainQuery {
    type Content = NewExplainQuery;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_EXPLAIN)
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.query_options)
    }
}
