
use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use api::{Empty, Method, Operation, Parameters, Prepare, RpcReturnType};
use query::Query;
use super::types::*;

/// Executes a query and returns a cursor with the first result set.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateCursor<T> {
    result_type: PhantomData<T>,
    query: NewCursor,
}

impl<T> CreateCursor<T> {
    /// Constructs a new instance of the `CreateCursor` method for the given
    /// `NewCursor`.
    pub fn new(query: NewCursor) -> Self {
        CreateCursor {
            result_type: PhantomData,
            query,
        }
    }

    /// Constructs a new instance of the `CreateCursor` method for the given
    /// query and bind parameters.
    pub fn from_query(query: Query) -> Self {
        CreateCursor {
            result_type: PhantomData,
            query: query.into(),
        }
    }

    /// Returns the query for which the cursor shall be created.
    pub fn query(&self) -> &NewCursor {
        &self.query
    }
}

impl<T> Method for CreateCursor<T>
    where T: DeserializeOwned
{
    type Result = Cursor<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<T> Prepare for CreateCursor<T> {
    type Content = NewCursor;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/cursor")
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.query)
    }
}

/// Deletes the cursor and frees the resources associated with it.
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteCursor {
    /// An id of a cursor that shall be deleted.
    cursor_id: String,
}

impl DeleteCursor {
    /// Constructs a new instance of the `DeleteCursor` method that shall
    /// delete the cursor with the given id.
    pub fn new(cursor_id: String) -> Self {
        DeleteCursor {
            cursor_id,
        }
    }

    /// Constructs a new instance of the `DeleteCursor` method that shall
    /// delete the cursor with the given id.
    pub fn with_id<I>(cursor_id: I) -> Self
        where I: Into<String>
    {
        DeleteCursor {
            cursor_id: cursor_id.into(),
        }
    }

    /// Returns the id of the cursor that shall be deleted.
    pub fn cursor_id(&self) -> &str {
        &self.cursor_id
    }
}

impl Method for DeleteCursor {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for DeleteCursor {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from("/_api/cursor/") + &self.cursor_id
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Reads the next batch of results from a cursor.
#[derive(Clone, Debug, PartialEq)]
pub struct ReadNextBatchFromCursor<T> {
    result_type: PhantomData<T>,
    /// An id of a cursor from which the next batch of results shall be read.
    cursor_id: String,
}

impl<T> ReadNextBatchFromCursor<T> {
    /// Constructs a new instance of the `ReadNextBatchFromCursor` method for
    /// the cursor with the given id.
    pub fn new(cursor_id: String) -> Self {
        ReadNextBatchFromCursor {
            result_type: PhantomData,
            cursor_id,
        }
    }

    /// Constructs a new instance of the `ReadNextBatchFromCursor` method for
    /// the cursor with the given id.
    pub fn with_id<I>(cursor_id: I) -> Self
        where I: Into<String>
    {
        ReadNextBatchFromCursor {
            result_type: PhantomData,
            cursor_id: cursor_id.into(),
        }
    }

    /// Returns the id of the cursor from which the next batch of results shall
    /// be read.
    pub fn cursor_id(&self) -> &str {
        &self.cursor_id
    }
}

impl<T> Method for ReadNextBatchFromCursor<T>
    where T: DeserializeOwned
{
    type Result = Cursor<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl<T> Prepare for ReadNextBatchFromCursor<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from("/_api/cursor/") + &self.cursor_id
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
