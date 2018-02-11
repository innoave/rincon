
use std::fmt::Debug;
use std::iter::FromIterator;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use rincon_core::api::method::{Method, Operation, Parameters, Prepare,
    ResultList, RpcReturnType};
use rincon_core::arango::protocol::{FIELD_CODE, HEADER_IF_MATCH,
    HEADER_IF_NON_MATCH, PARAM_IGNORE_REVISIONS, PARAM_KEEP_NULL,
    PARAM_MERGE_OBJECTS, PARAM_RETURN_NEW, PARAM_RETURN_OLD,
    PARAM_WAIT_FOR_SYNC, PATH_API_DOCUMENT};
use super::types::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GetDocument<T> {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
    content: PhantomData<T>,
}

impl<T> GetDocument<T> {
    pub fn new<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        GetDocument {
            id: DocumentId::new(collection_name, document_key.deconstruct()),
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_key<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        GetDocument {
            id: DocumentId::new(collection_name, document_key.deconstruct()),
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_id(id: DocumentId) -> Self {
        GetDocument {
            id,
            if_match: None,
            if_non_match: None,
            content: PhantomData,
        }
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
        where IfMatch: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<IfNonMatch>(mut self, if_non_match: IfNonMatch) -> Self
        where IfNonMatch: Into<Option<String>>
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
        String::from(PATH_API_DOCUMENT)
            + "/" + self.id.collection_name()
            + "/" + self.id.document_key()
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
pub struct GetDocumentHeader {
    id: DocumentId,
    if_match: Option<String>,
    if_non_match: Option<String>,
}

impl GetDocumentHeader {
    pub fn new<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        GetDocumentHeader {
            id: DocumentId::new(collection_name, document_key.deconstruct()),
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_key<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        GetDocumentHeader {
            id: DocumentId::new(collection_name, document_key.deconstruct()),
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_id(id: DocumentId) -> Self {
        GetDocumentHeader {
            id,
            if_match: None,
            if_non_match: None,
        }
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
        where IfMatch: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_if_non_match<IfNonMatch>(mut self, if_non_match: IfNonMatch) -> Self
        where IfNonMatch: Into<Option<String>>
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
    type Result = ();
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
        String::from(PATH_API_DOCUMENT)
            + "/" + self.id.collection_name()
            + "/" + self.id.document_key()
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
pub struct InsertDocument<T> {
    collection_name: String,
    document: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocument<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
        where N: Into<String>
    {
        InsertDocument {
            collection_name: collection_name.into(),
            document,
            force_wait_for_sync: None,
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
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocument<T>
    where T: DeserializeOwned
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
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
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

#[derive(Debug, Clone, PartialEq)]
pub struct InsertDocumentReturnNew<T> {
    collection_name: String,
    document: NewDocument<T>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocumentReturnNew<T> {
    pub fn new<N>(collection_name: N, document: NewDocument<T>) -> Self
        where N: Into<String>
    {
        InsertDocumentReturnNew {
            collection_name: collection_name.into(),
            document,
            force_wait_for_sync: None,
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
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocumentReturnNew<T>
    where T: DeserializeOwned
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
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
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

#[derive(Debug, Clone, PartialEq)]
pub struct InsertDocuments<T> {
    collection_name: String,
    documents: Vec<NewDocument<T>>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocuments<T> {
    pub fn new<N, Docs>(collection_name: N, documents: Docs) -> Self
        where N: Into<String>, Docs: IntoIterator<Item=NewDocument<T>>
    {
        InsertDocuments {
            collection_name: collection_name.into(),
            documents: Vec::from_iter(documents.into_iter()),
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn documents(&self) -> &[NewDocument<T>] {
        &self.documents
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocuments<T>
    where T: DeserializeOwned
{
    type Result = ResultList<DocumentHeader>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocuments<T>
    where T: Serialize + Debug
{
    type Content = Vec<NewDocument<T>>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, false);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.documents)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InsertDocumentsReturnNew<T> {
    collection_name: String,
    documents: Vec<NewDocument<T>>,
    force_wait_for_sync: Option<bool>,
}

impl<T> InsertDocumentsReturnNew<T> {
    pub fn new<Coll, Docs>(collection_name: Coll, documents: Docs) -> Self
        where Coll: Into<String>, Docs: IntoIterator<Item=NewDocument<T>>
    {
        InsertDocumentsReturnNew {
            collection_name: collection_name.into(),
            documents: Vec::from_iter(documents.into_iter()),
            force_wait_for_sync: None,
        }
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn documents(&self) -> &[NewDocument<T>] {
        &self.documents
    }

    pub fn with_force_wait_for_sync<Wfs>(mut self, force_wait_for_sync: Wfs) -> Self
        where Wfs: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn is_force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }
}

impl<T> Method for InsertDocumentsReturnNew<T>
    where T: DeserializeOwned
{
    type Result = ResultList<Document<T>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for InsertDocumentsReturnNew<T>
    where T: Serialize + Debug
{
    type Content = Vec<NewDocument<T>>;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_NEW, true);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.documents)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceDocument<Old, New> {
    document_id: DocumentId,
    new_document: DocumentUpdate<New>,
    old_content: PhantomData<Old>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    if_match: Option<String>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Old, New> ReplaceDocument<Old, New> {
    pub fn new(document_id: DocumentId, new_document: DocumentUpdate<New>) -> Self {
        ReplaceDocument {
            document_id,
            new_document,
            old_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            if_match: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_if_match<M>(mut self, if_match: M) -> Self
        where M: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn document_id(&self) -> &DocumentId {
        &self.document_id
    }

    pub fn new_document(&self) -> &DocumentUpdate<New> {
        &self.new_document
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Old, New> Method for ReplaceDocument<Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = UpdatedDocument<Old, New>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Old, New> Prepare for ReplaceDocument<Old, New>
    where New: Serialize + Debug
{
    type Content = DocumentUpdate<New>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT)
            + "/" + self.document_id.collection_name()
            + "/" + self.document_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
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
        Some(&self.new_document)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceDocuments<Old, New> {
    collection_name: String,
    new_documents: Vec<DocumentUpdate<New>>,
    old_content: PhantomData<Old>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Old, New> ReplaceDocuments<Old, New> {
    pub fn new<Coll, Docs>(collection_name: Coll, new_documents: Docs) -> Self
        where Coll: Into<String>, Docs: IntoIterator<Item=DocumentUpdate<New>>
    {
        ReplaceDocuments {
            collection_name: collection_name.into(),
            new_documents: Vec::from_iter(new_documents.into_iter()),
            old_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn new_documents(&self) -> &[DocumentUpdate<New>] {
        &self.new_documents
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Old, New> Method for ReplaceDocuments<Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = ResultList<UpdatedDocument<Old, New>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Old, New> Prepare for ReplaceDocuments<Old, New>
    where New: Serialize + Debug
{
    type Content = Vec<DocumentUpdate<New>>;

    fn operation(&self) -> Operation {
        Operation::Replace
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.new_documents)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyDocument<Upd, Old, New> {
    document_id: DocumentId,
    update: DocumentUpdate<Upd>,
    old_content: PhantomData<Old>,
    new_content: PhantomData<New>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    if_match: Option<String>,
    keep_none: Option<bool>,
    merge_objects: Option<bool>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Upd, Old, New> ModifyDocument<Upd, Old, New> {
    pub fn new(document_id: DocumentId, update: DocumentUpdate<Upd>) -> Self {
        ModifyDocument {
            document_id,
            update,
            old_content: PhantomData,
            new_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            if_match: None,
            keep_none: None,
            merge_objects: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_if_match<M>(mut self, if_match: M) -> Self
        where M: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn with_keep_none<K>(mut self, keep_none: K) -> Self
        where K: Into<Option<bool>>
    {
        self.keep_none = keep_none.into();
        self
    }

    pub fn with_merge_objects<M>(mut self, merge_objects: M) -> Self
        where M: Into<Option<bool>>
    {
        self.merge_objects = merge_objects.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn document_id(&self) -> &DocumentId {
        &self.document_id
    }

    pub fn update(&self) -> &DocumentUpdate<Upd> {
        &self.update
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }

    pub fn keep_none(&self) -> Option<bool> {
        self.keep_none
    }

    pub fn merge_objects(&self) -> Option<bool> {
        self.merge_objects
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Upd, Old, New> Method for ModifyDocument<Upd, Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = UpdatedDocument<Old, New>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd, Old, New> Prepare for ModifyDocument<Upd, Old, New>
    where Upd: Serialize + Debug
{
    type Content = DocumentUpdate<Upd>;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT)
            + "/" + self.document_id.collection_name()
            + "/" + self.document_id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(keep_none) = self.keep_none {
            params.insert(PARAM_KEEP_NULL, keep_none);
        }
        if let Some(merge_objects) = self.merge_objects {
            params.insert(PARAM_MERGE_OBJECTS, merge_objects);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
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
pub struct ModifyDocuments<Upd, Old, New> {
    collection_name: String,
    updates: Vec<DocumentUpdate<Upd>>,
    old_content: PhantomData<Old>,
    new_content: PhantomData<New>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
    keep_none: Option<bool>,
    merge_objects: Option<bool>,
    return_old: Option<bool>,
    return_new: Option<bool>,
}

impl<Upd, Old, New> ModifyDocuments<Upd, Old, New> {
    pub fn new<Coll, Upds>(collection_name: Coll, updates: Upds) -> Self
        where Coll: Into<String>, Upds: IntoIterator<Item=DocumentUpdate<Upd>>
    {
        ModifyDocuments {
            collection_name: collection_name.into(),
            updates: Vec::from_iter(updates.into_iter()),
            old_content: PhantomData,
            new_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
            keep_none: None,
            merge_objects: None,
            return_old: None,
            return_new: None,
        }
    }

    pub fn with_force_wait_for_sync<W>(mut self, force_wait_for_sync: W) -> Self
        where W: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn with_keep_none<K>(mut self, keep_none: K) -> Self
        where K: Into<Option<bool>>
    {
        self.keep_none = keep_none.into();
        self
    }

    pub fn with_merge_objects<M>(mut self, merge_objects: M) -> Self
        where M: Into<Option<bool>>
    {
        self.merge_objects = merge_objects.into();
        self
    }

    pub fn with_return_old<O>(mut self, return_old: O) -> Self
        where O: Into<Option<bool>>
    {
        self.return_old = return_old.into();
        self
    }

    pub fn with_return_new<N>(mut self, return_new: N) -> Self
        where N: Into<Option<bool>>
    {
        self.return_new = return_new.into();
        self
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn updates(&self) -> &[DocumentUpdate<Upd>] {
        &self.updates
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }

    pub fn keep_none(&self) -> Option<bool> {
        self.keep_none
    }

    pub fn merge_objects(&self) -> Option<bool> {
        self.merge_objects
    }

    pub fn return_old(&self) -> Option<bool> {
        self.return_old
    }

    pub fn return_new(&self) -> Option<bool> {
        self.return_new
    }
}

impl<Upd, Old, New> Method for ModifyDocuments<Upd, Old, New>
    where Old: DeserializeOwned, New: DeserializeOwned
{
    type Result = ResultList<UpdatedDocument<Old, New>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<Upd, Old, New> Prepare for ModifyDocuments<Upd, Old, New>
    where Upd: Serialize + Debug
{
    type Content = Vec<DocumentUpdate<Upd>>;

    fn operation(&self) -> Operation {
        Operation::Modify
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        if let Some(keep_none) = self.keep_none {
            params.insert(PARAM_KEEP_NULL, keep_none);
        }
        if let Some(merge_objects) = self.merge_objects {
            params.insert(PARAM_MERGE_OBJECTS, merge_objects);
        }
        if let Some(return_old) = self.return_old {
            params.insert(PARAM_RETURN_OLD, return_old);
        }
        if let Some(return_new) = self.return_new {
            params.insert(PARAM_RETURN_NEW, return_new);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.updates)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteDocument {
    id: DocumentId,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl DeleteDocument {
    pub fn new<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        DeleteDocument::with_id(DocumentId::new(
            collection_name,
            document_key.deconstruct()
        ))
    }

    pub fn with_key<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        DeleteDocument::new(collection_name, document_key)
    }

    pub fn with_id(id: DocumentId) -> Self {
        DeleteDocument {
            id,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync<S>(mut self, force_wait_for_sync: S) -> Self
        where S: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
        where IfMatch: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl Method for DeleteDocument {
    type Result = DocumentHeader;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DeleteDocument {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT)
            + "/" + self.id.collection_name()
            + "/" + self.id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_OLD, false);
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
pub struct DeleteDocumentReturnOld<T> {
    id: DocumentId,
    old_content: PhantomData<T>,
    force_wait_for_sync: Option<bool>,
    if_match: Option<String>,
}

impl<T> DeleteDocumentReturnOld<T> {
    pub fn new<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        DeleteDocumentReturnOld::with_id(DocumentId::new(
            collection_name,
            document_key.deconstruct()
        ))
    }

    pub fn with_key<Coll>(collection_name: Coll, document_key: DocumentKey) -> Self
        where Coll: Into<String>
    {
        DeleteDocumentReturnOld::with_id(DocumentId::new(
            collection_name,
            document_key.deconstruct()
        ))
    }

    pub fn with_id(id: DocumentId) -> Self {
        DeleteDocumentReturnOld {
            id,
            old_content: PhantomData,
            force_wait_for_sync: None,
            if_match: None,
        }
    }

    pub fn with_force_wait_for_sync<S>(mut self, force_wait_for_sync: S) -> Self
        where S: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_if_match<IfMatch>(mut self, if_match: IfMatch) -> Self
        where IfMatch: Into<Option<String>>
    {
        self.if_match = if_match.into();
        self
    }

    pub fn id(&self) -> &DocumentId {
        &self.id
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn if_match(&self) -> Option<&String> {
        self.if_match.as_ref()
    }
}

impl<T> Method for DeleteDocumentReturnOld<T>
    where T: DeserializeOwned
{
    type Result = Document<T>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for DeleteDocumentReturnOld<T> {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT)
            + "/" + self.id.collection_name()
            + "/" + self.id.document_key()
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_OLD, true);
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
pub struct DeleteDocuments {
    collection_name: String,
    selectors: Vec<DocumentSelector>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
}

impl DeleteDocuments {
    pub fn new<Coll, Selectors>(collection_name: Coll, selectors: Selectors) -> Self
        where Coll: Into<String>, Selectors: IntoIterator<Item=DocumentSelector>
    {
        DeleteDocuments {
            collection_name: collection_name.into(),
            selectors: Vec::from_iter(selectors.into_iter()),
            force_wait_for_sync: None,
            ignore_revisions: None,
        }
    }

    pub fn with_ids<Coll, Ids>(collection_name: Coll, ids: Ids) -> Self
        where Coll: Into<String>, Ids: IntoIterator<Item=DocumentId>
    {
        DeleteDocuments::new(collection_name, ids.into_iter()
            .map(DocumentSelector::Id))
    }

    pub fn with_keys<Coll, Keys>(collection_name: Coll, keys: Keys) -> Self
        where Coll: Into<String>, Keys: IntoIterator<Item=DocumentKey>
    {
        DeleteDocuments::new(collection_name, keys.into_iter()
            .map(DocumentSelector::Key))
    }

    pub fn with_headers<Coll, Headers>(collection_name: Coll, headers: Headers) -> Self
        where Coll: Into<String>, Headers: IntoIterator<Item=DocumentHeader>
    {
        DeleteDocuments::new(collection_name, headers.into_iter()
            .map(DocumentSelector::Header))
    }

    pub fn with_force_wait_for_sync<S>(mut self, force_wait_for_sync: S) -> Self
        where S: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn selectors(&self) -> &[DocumentSelector] {
        &self.selectors
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }
}

impl Method for DeleteDocuments {
    type Result = ResultList<DocumentHeader>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl Prepare for DeleteDocuments {
    type Content = Vec<DocumentSelector>;

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_OLD, false);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.selectors)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteDocumentsReturnOld<T> {
    collection_name: String,
    selectors: Vec<DocumentSelector>,
    old_content: PhantomData<T>,
    force_wait_for_sync: Option<bool>,
    ignore_revisions: Option<bool>,
}

impl<T> DeleteDocumentsReturnOld<T> {
    pub fn new<Coll, Selectors>(collection_name: Coll, selectors: Selectors) -> Self
        where Coll: Into<String>, Selectors: IntoIterator<Item=DocumentSelector>
    {
        DeleteDocumentsReturnOld {
            collection_name: collection_name.into(),
            selectors: Vec::from_iter(selectors.into_iter()),
            old_content: PhantomData,
            force_wait_for_sync: None,
            ignore_revisions: None,
        }
    }

    pub fn with_ids<Coll, Ids>(collection_name: Coll, ids: Ids) -> Self
        where Coll: Into<String>, Ids: IntoIterator<Item=DocumentId>
    {
        DeleteDocumentsReturnOld::new(collection_name, ids.into_iter()
            .map(DocumentSelector::Id))
    }

    pub fn with_keys<Coll, Keys>(collection_name: Coll, keys: Keys) -> Self
        where Coll: Into<String>, Keys: IntoIterator<Item=DocumentKey>
    {
        DeleteDocumentsReturnOld::new(collection_name, keys.into_iter()
            .map(DocumentSelector::Key))
    }

    pub fn with_headers<Coll, Headers>(collection_name: Coll, headers: Headers) -> Self
        where Coll: Into<String>, Headers: IntoIterator<Item=DocumentHeader>
    {
        DeleteDocumentsReturnOld::new(collection_name, headers.into_iter()
            .map(DocumentSelector::Header))
    }

    pub fn with_force_wait_for_sync<S>(mut self, force_wait_for_sync: S) -> Self
        where S: Into<Option<bool>>
    {
        self.force_wait_for_sync = force_wait_for_sync.into();
        self
    }

    pub fn with_ignore_revisions<R>(mut self, ignore_revisions: R) -> Self
        where R: Into<Option<bool>>
    {
        self.ignore_revisions = ignore_revisions.into();
        self
    }

    pub fn collection_name(&self) -> &str {
        &self.collection_name
    }

    pub fn selectors(&self) -> &[DocumentSelector] {
        &self.selectors
    }

    pub fn force_wait_for_sync(&self) -> Option<bool> {
        self.force_wait_for_sync
    }

    pub fn ignore_revisions(&self) -> Option<bool> {
        self.ignore_revisions
    }
}

impl<T> Method for DeleteDocumentsReturnOld<T>
    where T: DeserializeOwned
{
    type Result = ResultList<Document<T>>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some(FIELD_CODE),
    };
}

impl<T> Prepare for DeleteDocumentsReturnOld<T> {
    type Content = Vec<DocumentSelector>;

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from(PATH_API_DOCUMENT) + "/" + &self.collection_name
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::new();
        params.insert(PARAM_RETURN_OLD, true);
        if let Some(force_wait_for_sync) = self.force_wait_for_sync {
            params.insert(PARAM_WAIT_FOR_SYNC, force_wait_for_sync);
        }
        if let Some(ignore_revisions) = self.ignore_revisions {
            params.insert(PARAM_IGNORE_REVISIONS, ignore_revisions);
        }
        params
    }

    fn header(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.selectors)
    }
}
