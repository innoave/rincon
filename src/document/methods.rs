
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use arango::protocol::{FIELD_CODE, FIELD_ID, FIELD_RESULT, PARAM_RETURN_NEW, PARAM_SILENT,
    PARAM_WAIT_FOR_SYNC, PATH_API_DOCUMENT, PATH_PROPERTIES, PATH_RENAME};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocument<T> {
    collection_name: String,
    document: NewDocument<T>,
    wait_for_sync: Option<bool>,
    return_new: Option<bool>,
    silent: Option<bool>,
}

impl<T> InsertDocument<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
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

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn document(&self) -> &NewDocument<T> {
        &self.document
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.wait_for_sync
    }

    pub fn with_return_new<Rn>(mut self, return_new: Rn) -> Self
        where Rn: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn is_return_new(&self) -> Option<bool> {
        self.return_new
    }

    pub fn with_silent<S>(mut self, silent: S) -> Self
        where S: Into<Option<bool>>
    {
        self.silent = silent.into();
        self
    }

    pub fn is_silent(&self) -> Option<bool> {
        self.silent
    }
}

impl<T> Method for InsertDocument<T>
    where T: DeserializeOwned
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocument<T>
    where T: Serialize
{
    type Content = NewDocument<T>;

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
