
use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio_core::reactor::Core;

use rincon_client::collection::methods::*;
use rincon_client::collection::types::{Collection, CollectionProperties,
    CollectionPropertiesUpdate, CollectionRevision, RenameTo};
use rincon_client::document::methods::*;
use rincon_client::document::types::{Document, DocumentHeader, DocumentId,
    DocumentKey, DocumentModifyOptions, DocumentReplaceOptions, DocumentUpdate,
    NewDocument, UpdatedDocument};
use rincon_core::api::connector::{Connector, Execute};
use rincon_core::api::method::{Method, Prepare, ResultList};
use rincon_core::api::types::Entity;

use super::Result;

/// A session for operating with a specific collection.
#[derive(Debug)]
pub struct CollectionSession<C> {
    entity: Entity<Collection>,
    database_name: String,
    connector: Rc<C>,
    core: Rc<RefCell<Core>>,
}

impl<C> CollectionSession<C>
    where C: 'static + Connector
{
    /// Instantiates a new `CollectionSession` for the given collection entity.
    pub(crate) fn new(
        entity: Entity<Collection>,
        database_name: String,
        connector: Rc<C>,
        core: Rc<RefCell<Core>>,
    ) -> Self {
        CollectionSession {
            entity,
            database_name,
            connector,
            core,
        }
    }

    /// Executes an API method applied to the database of this session.
    fn execute<M>(&self, method: M) -> Result<<M as Method>::Result>
        where M: 'static + Method + Prepare
    {
        self.core.borrow_mut().run(
            self.connector.connection(&self.database_name)
                .execute(method)
        )
    }

    /// Returns the name of the database this collection is located in.
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Returns the name of the collection this `CollectionSession` operates
    /// with.
    pub fn name(&self) -> &str {
        match self.entity {
            Entity::Name(ref name) => name,
            Entity::Object(ref obj) => obj.name(),
        }
    }

    /// Returns the `Collection` entity this `CollectionSession` operates with.
    ///
    /// It returns `Some(&Collection)` if this session holds a loaded collection
    /// entity or `None` otherwise.
    ///
    /// If the collection entity is not loaded the `load()` function can be
    /// used to get a session with a loaded collection entity.
    pub fn entity(&self) -> Option<&Collection> {
        match self.entity {
            Entity::Name(_) => None,
            Entity::Object(ref collection) => Some(collection),
        }
    }

    /// Unwraps the collection entity out of this session which is either the
    /// name of the collection or the `Collection` struct.
    pub fn unwrap_entity(self) -> Entity<Collection> {
        self.entity
    }

    /// Returns whether this session holds a loaded collection entity.
    ///
    /// It returns true if this session holds the collection entity or false
    /// otherwise. If this function returns true the `entity()` function will
    /// return `Some(&Collection)` otherwise that function returns `None`.
    pub fn is_entity(&self) -> bool {
        match self.entity {
            Entity::Name(_) => false,
            Entity::Object(_) => true,
        }
    }

    /// Fetches the entity of the collection represented by this session and
    /// returns a new `CollectionSession` with the entity set in the session.
    pub fn fetch(self) -> Result<CollectionSession<C>> {
        self.execute(GetCollection::with_name(self.name().clone()))
            .map(|collection|
                CollectionSession {
                    entity: Entity::Object(collection),
                    database_name: self.database_name,
                    connector: self.connector,
                    core: self.core,
                }
            )
    }

    /// Renames the collection represented by this session and returns the
    /// renamed collection as a new `CollectionSession`.
    pub fn rename<N>(self, new_name: N) -> Result<CollectionSession<C>>
        where N: Into<String>
    {
        self.execute(RenameCollection::new(self.name().into(), RenameTo::new(new_name)))
            .map(|collection| CollectionSession {
                entity: Entity::Object(collection),
                database_name: self.database_name,
                connector: self.connector,
                core: self.core,
            })
    }

    /// Gets the revision of the collection represented by this session.
    pub fn get_revision(&self) -> Result<CollectionRevision> {
        self.execute(GetCollectionRevision::new(self.name().into()))
    }

    /// Gets the properties of the collection represented by this session.
    pub fn get_properties(&self) -> Result<CollectionProperties> {
        self.execute(GetCollectionProperties::with_name(self.name().clone()))
    }

    /// Changes the properties of the collection represented by this session
    /// and returns the updated collection properties.
    pub fn change_properties(&self, properties: CollectionPropertiesUpdate) -> Result<CollectionProperties> {
        self.execute(ChangeCollectionProperties::new(self.name().into(), properties))
    }

    /// Inserts a new document into this collection.
    pub fn insert_document<D, T>(&self, document: D) -> Result<DocumentHeader>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocument::new(self.name(), document.into()))
    }

    /// Inserts a new document into this collection with forced waiting for the
    /// document being synced to disk.
    pub fn insert_document_synced<D, T>(&self, document: D) -> Result<DocumentHeader>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocument::new(self.name(), document.into())
            .with_force_wait_for_sync(true))
    }

    /// Inserts a new document into this collection and returns the newly
    /// created document.
    pub fn insert_document_return_new<D, T>(&self, document: D) -> Result<Document<T>>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentReturnNew::new(self.name(), document.into()))
    }

    /// Inserts a new document into this collection with forced waiting for the
    /// document being synced to disk and returns the newly created document
    pub fn insert_document_return_new_synced<D, T>(&self, document: D) -> Result<Document<T>>
        where
            D: Into<NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentReturnNew::new(self.name(), document.into())
            .with_force_wait_for_sync(true))
    }

    /// Inserts multiple documents into this collection.
    pub fn insert_documents<D, T>(&self, documents: D) -> Result<ResultList<DocumentHeader>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocuments::new(self.name(), documents))
    }

    /// Inserts multiple documents into this collection with forced waiting for
    /// the documents being synced to disk.
    pub fn insert_documents_synced<D, T>(&self, documents: D) -> Result<ResultList<DocumentHeader>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocuments::new(self.name(), documents)
            .with_force_wait_for_sync(true))
    }

    /// Inserts multiple documents into this collection and returns the newly
    /// created documents.
    pub fn insert_documents_return_new<D, T>(&self, documents: D) -> Result<ResultList<Document<T>>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentsReturnNew::new(self.name(), documents))
    }

    /// Inserts multiple documents into this collection with forced waiting for
    /// the documents being synced to disk and returns the newly created
    /// documents.
    pub fn insert_documents_return_new_synced<D, T>(&self, documents: D) -> Result<ResultList<Document<T>>>
        where
            D: IntoIterator<Item=NewDocument<T>>,
            T: 'static + Serialize + DeserializeOwned + Debug,
    {
        self.execute(InsertDocumentsReturnNew::new(self.name(), documents)
            .with_force_wait_for_sync(true))
    }

    /// Fetches the document with the given key from this collection.
    pub fn get_document<T>(&self, key: DocumentKey) -> Result<Document<T>>
        where T: 'static + DeserializeOwned
    {
        self.execute(GetDocument::new(self.name(), key))
    }

    /// Fetches the document with the given key from this collection if the
    /// revision matches the given predicate.
    pub fn get_document_if_match<IfMatch, T>(&self, key: DocumentKey, if_match: IfMatch) -> Result<Document<T>>
        where
            IfMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocument::new(self.name(), key).with_if_match(if_match))
    }

    /// Fetches the document with the given key from this collection if the
    /// revision does not match the given predicate.
    pub fn get_document_if_non_match<IfNonMatch, T>(&self, key: DocumentKey, if_non_match: IfNonMatch) -> Result<Document<T>>
        where
            IfNonMatch: Into<String>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocument::new(self.name(), key).with_if_non_match(if_non_match))
    }

    /// Fetches multiple documents with the given keys from this collection.
    pub fn get_documents<Keys, T>(&self, keys: Keys) -> Result<ResultList<Document<T>>>
        where
            Keys: IntoIterator<Item=DocumentKey>,
            T: 'static + DeserializeOwned,
    {
        self.execute(GetDocuments::with_keys(self.name(), keys))
    }

    /// Replaces an existing document with new content.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    pub fn replace_document<Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document))
    }

    /// Replaces an existing document with new content if the match condition
    /// is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    pub fn replace_document_if_match<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        if_match: IfMatch,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document)
            .with_if_match(if_match)
        )
    }

    /// Replaces an existing document with new content if the match condition
    /// is met. This function allows to specify detailed options for the
    /// replace method.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    /// * `options` : Additional options for the replace method
    pub fn replace_document_if_match_opt<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        if_match: IfMatch,
        options: DocumentReplaceOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document)
            .with_if_match(if_match)
            .with_options(options)
        )
    }

    /// Replaces an existing document with new content. This function allows
    /// to specify detailed options for the replace method.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `new_document` : The new content of the document
    /// * `options` : Additional options for the replace method
    pub fn replace_document_opt<Old, New>(
        &self,
        key: DocumentKey,
        new_document: DocumentUpdate<New>,
        options: DocumentReplaceOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ReplaceDocument::new(id, new_document).with_options(options))
    }

    /// Partially modifies an existing document.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    pub fn modify_document<Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications))
    }

    /// Partially modifies an existing document if the match condition is met.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    pub fn modify_document_if_match<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        if_match: IfMatch,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_if_match(if_match)
        )
    }

    /// Partially modifies an existing document if the match condition is met.
    /// This function allows to specify detailed options for the modify method.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `if_match` : The match condition that must be met to replace a
    ///   document
    /// * `options` : Additional options for the modify method
    pub fn modify_document_if_match_opt<IfMatch, Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        if_match: IfMatch,
        options: DocumentModifyOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_if_match(if_match)
            .with_options(options)
        )
    }

    /// Partially modifies an existing document. This function allows to
    /// specify detailed options for the modify method.
    ///
    /// The modifications argument must contain a document with the attributes
    /// to patch (the patch document). All attributes from the patch document
    /// will be added to the existing document if they do not exist yet or
    /// overwritten in the existing document if they already exist there.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be replaced
    /// * `modifications` : A document with the content to be modified
    ///   (patch document)
    /// * `options` : Additional options for the modify method
    pub fn modify_document_opt<Old, New>(
        &self,
        key: DocumentKey,
        modifications: DocumentUpdate<New>,
        options: DocumentModifyOptions,
    ) -> Result<UpdatedDocument<Old, New>>
        where
            Old: 'static + DeserializeOwned,
            New: 'static + Serialize + DeserializeOwned + Debug,
    {
        let id = DocumentId::new(self.name(), key.deconstruct());
        self.execute(ModifyDocument::new(id, modifications)
            .with_options(options)
        )
    }

    /// Deletes the document with the given key from this collection.
    pub fn delete_document(&self, key: DocumentKey) -> Result<DocumentHeader> {
        self.execute(DeleteDocument::new(self.name(), key))
    }

    /// Deletes the document with the given key from this collection if the
    /// match condition is met.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to delete the
    ///   document
    pub fn delete_document_if_match<IfMatch>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<DocumentHeader>
        where
            IfMatch: Into<String>,
    {
        self.execute(DeleteDocument::new(self.name(), key)
            .with_if_match(if_match)
        )
    }

    /// Deletes the document with the given key from this collection if the
    /// match condition is met and waits until the changes are synced to disk.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to delete the
    ///   document
    pub fn delete_document_if_match_synced<IfMatch>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<DocumentHeader>
        where
            IfMatch: Into<String>,
    {
        self.execute(DeleteDocument::new(self.name(), key)
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    /// Deletes the document with the given key from this collection and waits
    /// until the changes are synced to disk.
    pub fn delete_document_synced(&self, key: DocumentKey) -> Result<DocumentHeader> {
        self.execute(DeleteDocument::new(self.name(), key)
            .with_force_wait_for_sync(true)
        )
    }

    /// Deletes the document with the given key from this collection and returns
    /// the deleted document.
    pub fn delete_document_return_old<Old>(&self, key: DocumentKey) -> Result<Document<Old>>
        where
            Old: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), key))
    }

    /// Deletes the document with the given key from this collection if the
    /// match condition is met and returns the deleted document.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to delete the
    ///   document
    pub fn delete_document_if_match_return_old<IfMatch, Old>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<Document<Old>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), key)
            .with_if_match(if_match)
        )
    }

    /// Deletes the document with the given key from this collection if the
    /// match condition is met, waits until the changes are synced to disk and
    /// returns the deleted document.
    ///
    /// # Arguments
    ///
    /// * `key` : The key of the document to be deleted
    /// * `if_match` : The match condition that must be met to delete the
    ///   document
    pub fn delete_document_if_match_return_old_synced<IfMatch, Old>(
        &self,
        key: DocumentKey,
        if_match: IfMatch,
    ) -> Result<Document<Old>>
        where
            IfMatch: Into<String>,
            Old: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), key)
            .with_if_match(if_match)
            .with_force_wait_for_sync(true)
        )
    }

    /// Deletes the document with the given key from this collection, waits
    /// until the changes are synced to disk and returns the deleted document.
    pub fn delete_document_return_old_synced<Old>(&self, key: DocumentKey) -> Result<Document<Old>>
        where
            Old: 'static + DeserializeOwned,
    {
        self.execute(DeleteDocumentReturnOld::new(self.name(), key)
            .with_force_wait_for_sync(true))
    }
}
