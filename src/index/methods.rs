
use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use arango::protocol::{FIELD_CODE, FIELD_ID, PARAM_COLLECTION, PATH_API_INDEX};
use super::types::*;

/// Returns an `IndexList` with an attribute indexes containing an array of all
/// index descriptions for the given collection.
///
/// The same information is also available in the identifiers as a HashMap with
/// the index handles as keys.
#[derive(Clone, Debug, PartialEq)]
pub struct GetIndexList {
    collection_name: String,
}

impl GetIndexList {
    pub fn new<C>(collection_name: C) -> Self
        where C: Into<String>
    {
        GetIndexList {
            collection_name: collection_name.into(),
        }
    }

    pub fn of_collection<C>(collection_name: C) -> Self
        where C: Into<String>
    {
        GetIndexList::new(collection_name)
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }
}

impl Method for GetIndexList {
    type Result = IndexList;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetIndexList {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_INDEX)
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        params.insert(PARAM_COLLECTION, self.collection_name.to_owned());
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Returns the index description for an index of a collection.
#[derive(Clone, Debug, PartialEq)]
pub struct GetIndex {
    index_id: IndexId,
}

impl GetIndex {
    pub fn new(index_id: IndexId) -> Self {
        GetIndex {
            index_id,
        }
    }

    pub fn index_id(&self) -> &IndexId {
        &self.index_id
    }
}

impl Method for GetIndex {
    type Result = Index;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetIndex {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_INDEX) + "/" + &self.index_id.to_string()
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

/// Creates a new index in the collection of the given collection name. The
/// type of the index and its details are given in the index parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateIndex {
    collection_name: String,
    index: NewIndex,
}

impl CreateIndex {
    pub fn new<C, I>(collection_name: C, index: I) -> Self
        where C: Into<String>, I: Into<NewIndex>
    {
        CreateIndex {
            collection_name: collection_name.into(),
            index: index.into(),
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn index(&self) -> &NewIndex {
        &self.index
    }
}

impl Method for CreateIndex {
    type Result = Index;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for CreateIndex {
    type Content = NewIndex;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_INDEX)
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        params.insert(PARAM_COLLECTION, self.collection_name.to_owned());
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.index)
    }
}

/// Deletes the index with the given index id.
#[derive(Clone, Debug, PartialEq)]
pub struct DeleteIndex {
    index_id: IndexId,
}

impl DeleteIndex {
    pub fn new(index_id: IndexId) -> Self {
        DeleteIndex {
            index_id,
        }
    }

    pub fn index_id(&self) -> &IndexId {
        &self.index_id
    }
}

impl Method for DeleteIndex {
    type Result = IndexIdOption;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some(FIELD_ID),
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DeleteIndex {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_INDEX) + "/" + &self.index_id.to_string()
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
