
use api::{Method, Operation, Parameters, Prepare, RpcReturnType};
use super::types::*;

/// Retrieves a list of existing collections.
#[derive(Clone, Debug, PartialEq)]
pub struct ListOfCollections {
    /// Whether or not to exclude system collections from the response.
    exclude_system: bool
}

impl ListOfCollections {
    /// Constructs a new instance of the `ListOfCollections` method with
    /// the `exclude_system` parameter set to `true`.
    pub fn new() -> Self {
        ListOfCollections {
            exclude_system: true,
        }
    }

    /// Constructs a new instance of the `ListOfCollections` method with
    /// the `exclude_system` parameter set to `false`.
    pub fn including_system() -> Self {
        ListOfCollections {
            exclude_system: false,
        }
    }

    /// Sets whether system collections shall be excluded from the response.
    pub fn set_exclude_system(&mut self, exclude: bool) {
        self.exclude_system = exclude;
    }

    /// Returns whether system collections are going to be excluded from the
    /// response.
    pub fn is_exclude_system(&self) -> bool {
        self.exclude_system
    }
}

impl Method for ListOfCollections {
    type Result = Vec<Collection>;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("result"),
        code_field: Some("code"),
    };
}

impl Prepare for ListOfCollections {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Read
    }

    fn path(&self) -> String {
        String::from("/_api/collection")
    }

    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if self.exclude_system {
            params.push("excludeSystem", "true");
        }
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}

/// Creates a new collection with the given name.
#[derive(Clone, Debug, PartialEq)]
pub struct CreateCollection {
    collection: NewCollection,
    #[cfg(feature = "cluster")]
    wait_for_sync_replication: bool,
}

impl CreateCollection {
    /// Constructs a new instance of the `CreateCollection` method with the
    /// given `NewCollection` parameters.
    pub fn new(collection: NewCollection) -> Self {
        CreateCollection {
            collection,
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new collection with the given name and the default collection
    /// type. The default collection type is defined by the ArangoDB server.
    ///
    /// All other parameters will be set to their default values.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new documents collection with the given name.
    ///
    /// All other parameters will be set to their default values.
    pub fn documents_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::edges_with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Constructs a new instance of the `CreateCollection` method that will
    /// create a new edge collection with the given name.
    ///
    /// All other parameters will be set to their default values.
    pub fn edges_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        CreateCollection {
            collection: NewCollection::edges_with_name(name),
            #[cfg(feature = "cluster")]
            wait_for_sync_replication: true,
        }
    }

    /// Set whether the server shall wait until the new collection has been
    /// created at all replications before it returns the response.
    #[cfg(feature = "cluster")]
    pub fn set_wait_for_sync_replication(&mut self, wait_for_sync_replication: bool) {
        self.wait_for_sync_replication = wait_for_sync_replication;
    }

    /// Returns the parameters that are going to be used to create the new
    /// collection.
    pub fn collection(&self) -> &NewCollection {
        &self.collection
    }

    /// Returns whether the request will wait until the new collection has
    /// been created at all replications.
    #[cfg(feature = "cluster")]
    pub fn is_wait_for_sync_replication(&self) -> bool {
        &self.wait_for_sync_replication
    }
}

impl Method for CreateCollection {
    type Result = Collection;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: None,
        code_field: Some("code"),
    };
}

impl Prepare for CreateCollection {
    type Content = NewCollection;

    fn operation(&self) -> Operation {
        Operation::Create
    }

    fn path(&self) -> String {
        String::from("/_api/collection")
    }

    #[cfg(not(feature = "cluster"))]
    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    #[cfg(feature = "cluster")]
    fn parameters(&self) -> Parameters {
        let mut params = Parameters::with_capacity(1);
        if !self.wait_for_sync_replication {
            params.push("waitForSyncReplication", "0");
        }
        params
    }

    fn content(&self) -> Option<&Self::Content> {
        Some(&self.collection)
    }
}

/// Drops the collection identified by the given name.
///
/// If the collection was successfully dropped, the identifier of the dropped
/// collection is returned.
#[derive(Clone, Debug, PartialEq)]
pub struct DropCollection {
    name: String,
    system: bool,
}

impl DropCollection {
    /// Constructs a new instance of the `DropCollection` method that is
    /// going to be drop the user collection identified by the given name.
    ///
    /// **Note**: This method returns a `DropCollection` instance that drops
    /// user collections only. To drop a system collection either use the
    /// constructor method `DropCollection::system_with_name` or set the
    /// is_system property to `true` explicitly by calling the function
    /// `DropCollection::set_system`.
    pub fn with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        DropCollection {
            name: name.into(),
            system: false,
        }
    }

    /// Constructs a new instance of the `DropCollection` method that is
    /// going to be drop the system collection identified by the given name.
    ///
    /// **Note**: This method returns a `DropCollection` instance that drops
    /// system collections only. To drop a user collection either use the
    /// constructor method `DropCollection::with_name` or set the
    /// is_system property to `false` explicitly by calling the function
    /// `DropCollection::set_system`.
    pub fn system_with_name<N>(name: N) -> Self
        where N: Into<String>
    {
        DropCollection {
            name: name.into(),
            system: true,
        }
    }

    /// Set whether the collection to be dropped is a system collection.
    ///
    /// The collection is dropped only when this property is reflects the
    /// type of the collection to be dropped.
    pub fn set_system(&mut self, system: bool) {
        self.system = system;
    }

    /// Returns the name of the collection to be dropped.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the collection to be dropped is a system collection.
    pub fn is_system(&self) -> bool {
        self.system
    }
}

impl Method for DropCollection {
    type Result = String;
    const RETURN_TYPE: RpcReturnType = RpcReturnType {
        result_field: Some("id"),
        code_field: Some("code"),
    };
}

impl Prepare for DropCollection {
    type Content = ();

    fn operation(&self) -> Operation {
        Operation::Delete
    }

    fn path(&self) -> String {
        String::from("/_api/collection/") + &self.name
    }

    fn parameters(&self) -> Parameters {
        Parameters::empty()
    }

    fn content(&self) -> Option<&Self::Content> {
        None
    }
}
