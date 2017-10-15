
use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use index::types::*;

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
        code_field: Some("code"),
    };
}

impl Prepare for GetIndexList {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/index")
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        params.push("collection", self.collection_name.as_ref());
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Returns the index description for an index of a collection.
#[derive(Clone, Debug, PartialEq)]
pub struct GetIndex {
    collection_name: String,
    index_id: String,
}

impl GetIndex {
    pub fn new<C, I>(collection_name: C, index_id: I) -> Self
        where C: Into<String>, I: Into<String>
    {
        GetIndex {
            collection_name: collection_name.into(),
            index_id: index_id.into(),
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn index_id(&self) -> &str {
        &self.index_id
    }
}

impl Method for GetIndex {
    type Result = Index;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for GetIndex {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/index/") + &self.collection_name + "/" + &self.index_id
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Returns the index description for an index of a collection.
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
        code_field: Some("code"),
    };
}

impl Prepare for CreateIndex {
    type Content = NewIndex;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/index")
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        params.push("collection", self.collection_name.as_ref());
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.index)
    }
}
