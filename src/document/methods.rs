
use std::fmt::Debug;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use api::method::{Method, Operation, Parameters, Prepare, RpcReturnType};
use api::types::Empty;
use arango::protocol::{FIELD_CODE, HEADER_IF_MATCH, HEADER_IF_NON_MATCH,
    PARAM_RETURN_NEW, PARAM_SILENT, PARAM_WAIT_FOR_SYNC, PATH_API_DOCUMENT};
use super::types::*;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocument<T> {
    collection_name: String,
    document: NewDocument<T>,
    wait_for_sync: Option<bool>,
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
    where T: DeserializeOwned + Debug
{
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocument<T>
    where T: Serialize + Debug
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
        params.insert(PARAM_RETURN_NEW, false);
        if let Some(wait_for_sync) = self.wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, wait_for_sync);
        }
        if let Some(silent) = self.silent {
            params.insert(PARAM_SILENT, silent);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct InsertDocumentReturnNew<T> {
    collection_name: String,
    document: NewDocument<T>,
    wait_for_sync: Option<bool>,
    silent: Option<bool>,
}

impl<T> InsertDocumentReturnNew<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
        where N: Into<String>
    {
        InsertDocumentReturnNew {
            collection_name: collection_name.into(),
            document,
            wait_for_sync: None,
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

impl<T> Method for InsertDocumentReturnNew<T>
    where T: DeserializeOwned + Debug
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocumentReturnNew<T>
    where T: Serialize + Debug
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
        params.insert(PARAM_RETURN_NEW, true);
        if let Some(wait_for_sync) = self.wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, wait_for_sync);
        }
        if let Some(silent) = self.silent {
            params.insert(PARAM_SILENT, silent);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetDocument<T> {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
    content: PhantomData<T>,
}

impl<T> GetDocument<T> {
    pub fn new(id: DocumentId) -> Self {
        GetDocument {
            id,
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_if_match<Im>(mut self, if_match: Im) -> Self
        where Im: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<Inm>(mut self, if_non_match: Inm) -> Self
        where Inm: Into<Option<String>>
    {
        self.if_non_match = if_non_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl<T> Method for GetDocument<T>
    where T: DeserializeOwned
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for GetDocument<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.id.as_string()
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

#[derive(Clone, Debug, PartialEq)]
pub struct GetDocumentHeader {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
}

impl GetDocumentHeader {
    pub fn new(id: DocumentId) -> Self {
        GetDocumentHeader {
            id,
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_if_match<Im>(mut self, if_match: Im) -> Self
        where Im: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<Inm>(mut self, if_non_match: Inm) -> Self
        where Inm: Into<Option<String>>
    {
        self.if_non_match = if_non_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn if_non_match(&self) -> Option<&String> {
        self.if_non_match.as_ref()
    }
}

impl Method for GetDocumentHeader {
    type Result = Empty;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for GetDocumentHeader {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::ReadHeader
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.id.as_string()
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
