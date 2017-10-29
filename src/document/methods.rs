
use serde::ser::Serialize;

use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use arango_protocol::{FIELD_CODE, FIELD_ID, FIELD_RESULT, PARAM_RETURN_NEW, PARAM_SILENT,
    PARAM_WAIT_FOR_SYNC, PATH_API_DOCUMENT, PATH_PROPERTIES, PATH_RENAME};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocument<T> {
    collection_name: String,
    document: T,
    wait_for_sync: Option<bool>,
    return_new: Option<bool>,
    silent: Option<bool>,
}

impl<T> InsertDocument<T> {
    pub fn new<N>(collection_name: N, document: T) -> Self
        where N: Into<String>
    {
        InsertDocument {
            collection_name: collection_name.into(),
            document,
            wait_for_sync: None,
            return_new: None,
            silent: None,
        }
    }
}

impl<T> Method for InsertDocument<T> {
    type Result = Document;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocument<T>
    where T: Serialize
{
    type Content = T;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(wait_for_sync) = self.wait_for_sync {
            params.insert_bool(PARAM_WAIT_FOR_SYNC, wait_for_sync);
        }
        if let Some(return_new) = self.return_new {
            params.insert_bool(PARAM_RETURN_NEW, return_new);
        }
        if let Some(silent) = self.silent {
            params.insert_bool(PARAM_SILENT, silent);
        }
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.document)
    }
}
