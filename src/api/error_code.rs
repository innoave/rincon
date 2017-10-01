//! Error codes and their meanings
//!
//! This module defines the `ErrorCode` enum for all possible error codes as
//! listed in the official documentation of *ArangoDB*.
//!
//! Source: [https://docs.arangodb.com/devel/Manual/Appendix/ErrorCodes.html]
//!
//! The `ErrorCode` enum is defined by means of the macro `error_code_enum`.
//! This macro defines the enum variants as well as the functions `from_u16`
//! and `description`. The function `from_u16` can be used to convert an u16
//! value into the corresponding enum variant. The function `description`
//! returns a `&str` that contains a short description of an error code.
//!
//! Last updated: 09/17/2017

/// The `error_code_enum` macro defines an `ErrorCode` enum with the given
/// variants. In addition it implements the `description` function that returns
/// a short description of the error code as well as the `from_u16` function
/// that maps `u16` values to `ErrorCode` variants.
macro_rules! error_code_enum {
    ( $($i:ident($c:expr, $d:expr)),*, ) => {
        /// An enumeration of all error codes that are defined for *ArangoDB*.
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum ErrorCode {
            $(
                /// $d
                $i,
            )*
            /// ArangoDB returned an error code that has not been added
            /// to this enum yet.
            UnknownError,
        }

        impl ErrorCode {
            /// Returns the `ErrorCode` variant that is assigned to the
            /// given `u16` value.
            pub fn from_u16(value: u16) -> Self {
                match value {
                    $(
                        $c => ErrorCode::$i,
                    )*
                    _ => ErrorCode::UnknownError,
                }
            }

            /// Returns an integer that is assigned to the enum variant of
            /// this error code.
            pub fn as_u16(&self) -> u16 {
                match *self {
                    $(
                        ErrorCode::$i => $c,
                    )*
                    _ => ::std::u16::MAX,
                }
            }

            /// Returns a short description of the error code. It defines the meaning
            /// of the an error code and when it can occur.
            pub fn description(&self) -> &str {
                match *self {
                    $(
                        ErrorCode::$i => $d,
                    )*
                    ErrorCode::UnknownError => "An error occurred that is not known by the driver.",
                }
            }
        }

        impl ::std::fmt::Debug for ErrorCode {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&format!("ErrorCode({}, {})", self.as_u16(), self.description()))
            }
        }

        impl ::std::fmt::Display for ErrorCode {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&format!("Error Code {}: {}", self.as_u16(), self.description()))
            }
        }

        impl ::std::cmp::PartialOrd for ErrorCode {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.as_u16().partial_cmp(&other.as_u16())
            }
        }

        impl ::std::cmp::Ord for ErrorCode {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                self.as_u16().cmp(&other.as_u16())
            }
        }
    }
}

error_code_enum! {
    // General errors
    NoError(0, "No error has occurred."),
    Failed(1, "Will be raised when a general error occurred."),
    SysError(2, "Will be raised when operating system error occurred."),
    OutOfMemory(3, "Will be raised when there is a memory shortage."),
    Internal(4, "Will be raised when an internal error occurred."),
    IllegalNumber(5, "Will be raised when an illegal representation of a number was given."),
    NumericOverflow(6, "Will be raised when a numeric overflow occurred."),
    IllegalOption(7, "Will be raised when an unknown option was supplied by the user."),
    DeadPid(8, "Will be raised when a PID without a living process was found."),
    NotImplemented(9, "Will be raised when hitting an unimplemented feature."),
    BadParameter(10, "Will be raised when the parameter does not fulfill the requirements."),
    Forbidden(11, "Will be raised when you are missing permission for the operation."),
    OutOfMemoryMmap(12, "Will be raised when there is a memory shortage."),
    CorruptedCsv(13, "Will be raised when encountering a corrupt csv line."),
    FileNotFound(14, "Will be raised when a file is not found."),
    CannotWriteFile(15, "Will be raised when a file cannot be written."),
    CannotOverwriteFile(16, "Will be raised when an attempt is made to overwrite an existing file."),
    TypeError(17, "Will be raised when a type error is encountered."),
    LockTimeout(18, "Will be raised when there's a timeout waiting for a lock."),
    CannotCreateDirectory(19, "Will be raised when an attempt to create a directory fails."),
    CannotCreateTempFile(20, "Will be raised when an attempt to create a temporary file fails."),
    RequestCanceled(21, "Will be raised when a request is canceled by the user."),
    Debug(22, "Will be raised intentionally during debugging."),
    IpAddressInvalid(25, "Will be raised when the structure of an IP address is invalid."),
    FileExists(27, "Will be raised when a file already exists."),
    Locked(28, "Will be raised when a resource or an operation is locked."),
    Deadlock(29, "Will be raised when a deadlock is detected when accessing collections."),
    ShuttingDown(30, "Will be raised when a call cannot succeed because a server shutdown is already in progress."),
    OnlyEnterprise(31, "Will be raised when an enterprise-feature is requested from the community edition."),
    ResourceLimit(32, "Will be raised when the resources used by an operation exceed the configured maximum value."),

    // HTTP error status codes

    HttpBadParameter(400, "Will be raised when the HTTP request does not fulfill the requirements."),
    HttpUnauthorized(401, "Will be raised when authorization is required but the user is not authorized."),
    HttpForbidden(403, "Will be raised when the operation is forbidden."),
    HttpNotFound(404, "Will be raised when an URI is unknown."),
    HttpMethodNotAllowed(405, "Will be raised when an unsupported HTTP method is used for an operation."),
    HttpNotAcceptable(406, "Will be raised when an unsupported HTTP content type is used for an operation, or if a request is not acceptable for a leader or follower."),
    HttpConflict(409, "Will be raised when a conflict with the current state of a resource is found."),
    HttpPreconditionFailed(412, "Will be raised when a precondition for an HTTP request is not met."),
    HttpServerError(500, "Will be raised when an internal server is encountered."),
    HttpServiceUnavailable(503, "Will be raised when a service is temporarily unavailable."),

    // HTTP processing errors

    HttpCorruptedJson(600, "Will be raised when a string representation of a JSON object is corrupt."),
    HttpSuperfluousSuffices(601, "Will be raised when the URL contains superfluous suffices."),

    // Internal ArangoDB storage errors
    // For errors that occur because of a programming error.

    ArangoIllegalState(1000, "Internal error that will be raised when the datafile is not in the required state."),
    ArangoDatafileSealed(1002, "Internal error that will be raised when trying to write to a datafile."),
    ArangoReadOnly(1004, "Internal error that will be raised when trying to write to a read-only datafile or collection."),
    ArangoDuplicateIdentifier(1005, "Internal error that will be raised when a identifier duplicate is detected."),
    ArangoDatafileUnreadable(1006, "Internal error that will be raised when a datafile is unreadable."),
    ArangoDatafileEmpty(1007, "Internal error that will be raised when a datafile is empty."),
    ArangoRecovery(1008, "Will be raised when an error occurred during WAL log file recovery."),
    ArangoDatafileStatisticsNotFound(1009, "Will be raised when a required datafile statistics object was not found."),

    // External ArangoDB storage errors
    // For errors that occur because of an outside event.

    ArangoCorruptedDatafile(1100, "Will be raised when a corruption is detected in a datafile."),
    ArangoIllegalParameterFile(1101, "Will be raised if a parameter file is corrupted or cannot be read."),
    ArangoCorruptedCollection(1102, "Will be raised when a collection contains one or more corrupted data files."),
    ArangoMmapFailed(1103, "Will be raised when the system call mmap failed."),
    ArangoFilesystemFull(1104, "Will be raised when the filesystem is full."),
    ArangoNoJournal(1105, "Will be raised when a journal cannot be created."),
    ArangoDatafileAlreadyExists(1106, "Will be raised when the datafile cannot be created or renamed because a file of the same name already exists."),
    ArangoDatadirLocked(1107, "Will be raised when the database directory is locked by a different process."),
    ArangoCollectionDirectoryAlreadyExists(1108, "Will be raised when the collection cannot be created because a directory of the same name already exists."),
    ArangoMsyncFailed(1109, "Will be raised when the system call msync failed."),
    ArangoDatadirUnlockable(1110, "Will be raised when the server cannot lock the database directory on startup."),
    ArangoSyncTimeout(1111, "Will be raised when the server waited too long for a datafile to be synced to disk."),

    // General ArangoDB storage errors
    // For errors that occur when fulfilling a user request.

    ArangoConflict(1200, "Will be raised when updating or deleting a document and a conflict has been detected."),
    ArangoDatadirInvalid(1201, "Will be raised when a non-existing database directory was specified when starting the database."),
    ArangoDocumentNotFound(1202, "Will be raised when a document with a given identifier or handle is unknown."),
    ArangoCollectionNotFound(1203, "Will be raised when a collection with the given identifier or name is unknown."),
    ArangoCollectionParameterMissing(1204, "Will be raised when the collection parameter is missing."),
    ArangoDocumentHandleBad(1205, "Will be raised when a document handle is corrupt."),
    ArangoMaximalSizeTooSmall(1206, "Will be raised when the maximal size of the journal is too small."),
    ArangoDuplicateName(1207, "Will be raised when a name duplicate is detected."),
    ArangoIllegalName(1208, "Will be raised when an illegal name is detected."),
    ArangoNoIndex(1209, "Will be raised when no suitable index for the query is known."),
    ArangoUniqueConstraintViolated(1210, "Will be raised when there is a unique constraint violation."),
    ArangoViewNotFound(1211, "Will be raised when a view with the given identifier or name is unknown."),
    ArangoIndexNotFound(1212, "Will be raised when an index with a given identifier is unknown."),
    ArangoCrossCollectionRequest(1213, "Will be raised when a cross-collection is requested."),
    ArangoIndexHandleBad(1214, "Will be raised when a index handle is corrupt."),
    ArangoDocumentTooLarge(1216, "Will be raised when the document cannot fit into any datafile because of it is too large."),
    ArangoCollectionNotUnloaded(1217, "Will be raised when a collection should be unloaded, but has a different status."),
    ArangoCollectionTypeInvalid(1218, "Will be raised when an invalid collection type is used in a request."),
    ArangoValidationFailed(1219, "Will be raised when the validation of an attribute of a structure failed."),
    ArangoAttributeParserFailed(1220, "Will be raised when parsing an attribute name definition failed."),
    ArangoDocumentKeyBad(1221, "Will be raised when a document key is corrupt."),
    ArangoDocumentKeyUnexpected(1222, "Will be raised when a user-defined document key is supplied for collections with auto key generation."),
    ArangoDatadirNotWritable(1224, "Will be raised when the server's database directory is not writable for the current user."),
    ArangoOutOfKeys(1225, "Will be raised when a key generator runs out of keys."),
    ArangoDocumentKeyMissing(1226, "Will be raised when a document key is missing."),
    ArangoDocumentTypeInvalid(1227, "Will be raised when there is an attempt to create a document with an invalid type."),
    ArangoDatabaseNotFound(1228, "Will be raised when a non-existing database is accessed."),
    ArangoDatabaseNameInvalid(1229, "Will be raised when an invalid database name is used."),
    ArangoUseSystemDatabase(1230, "Will be raised when an operation is requested in a database other than the system database."),
    ArangoEndpointNotFound(1231, "Will be raised when there is an attempt to delete a non-existing endpoint."),
    ArangoInvalidKeyGenerator(1232, "Will be raised when an invalid key generator description is used."),
    ArangoInvalidEdgeAttribute(1233, "will be raised when the _from or _to values of an edge are undefined or contain an invalid value."),
    ArangoIndexDocumentAttributeMissing(1234, "Will be raised when an attempt to insert a document into an index is caused by in the document not having one or more attributes which the index is built on."),
    ArangoIndexCreationFailed(1235, "Will be raised when an attempt to create an index has failed."),
    ArangoWriteThrottleTimeout(1236, "Will be raised when the server is write-throttled and a write operation has waited too long for the server to process queued operations."),
    ArangoCollectionTypeMismatch(1237, "Will be raised when a collection has a different type from what has been expected."),
    ArangoCollectionNotLoaded(1238, "Will be raised when a collection is accessed that is not yet loaded."),
    ArangoDocumentRevBad(1239, "Will be raised when a document revision is corrupt or is missing where needed."),

    // Checked ArangoDB storage errors
    // For errors that occur but are anticipated.

    ArangoDatafileFull(1300, "Will be raised when the datafile reaches its limit."),
    ArangoEmptyDatadir(1301, "Will be raised when encountering an empty server database directory."),
    ArangoTryAgain(1302, "Will be raised when an operation should be retried."),
    ArangoBusy(1303, "Will be raised when storage engine is busy."),
    ArangoMergeInProgress(1304, "Will be raised when storage engine has a datafile merge in progress and cannot complete the operation."),
    ArangoIoError(1305, "Will be raised when storage engine encounters an I/O error."),

    // ArangoDB replication errors

    ReplicationNoResponse(1400, "Will be raised when the replication applier does not receive any or an incomplete response from the master."),
    ReplicationInvalidResponse(1401, "Will be raised when the replication applier receives an invalid response from the master."),
    ReplicationMasterError(1402, "Will be raised when the replication applier receives a server error from the master."),
    ReplicationMasterIncompatible(1403, "Will be raised when the replication applier connects to a master that has an incompatible version."),
    ReplicationMasterChange(1404, "Will be raised when the replication applier connects to a different master than before."),
    ReplicationLoop(1405, "Will be raised when the replication applier is asked to connect to itself for replication."),
    ReplicationUnexpectedMarker(1406, "Will be raised when an unexpected marker is found in the replication log stream."),
    ReplicationInvalidApplierState(1407, "Will be raised when an invalid replication applier state file is found."),
    ReplicationUnexpectedTransaction(1408, "Will be raised when an unexpected transaction id is found."),
    ReplicationInvalidApplierConfiguration(1410, "Will be raised when the configuration for the replication applier is invalid."),
    ReplicationRunning(1411, "Will be raised when there is an attempt to perform an operation while the replication applier is running."),
    ReplicationApplierStopped(1412, "Special error code used to indicate the replication applier was stopped by a user."),
    ReplicationNoStartTick(1413, "Will be raised when the replication applier is started without a known start tick value."),
    ReplicationStartTickNotPresent(1414, "Will be raised when the replication applier fetches data using a start tick, but that start tick is not present on the logger server anymore."),
    ReplicationWrongChecksumFormat(1415, "Will be raised when the format of the checksum is wrong"),
    ReplicationWrongChecksum(1416, "Will be raised when a new born follower submits a wrong checksum"),

    // ArangoDB cluster errors

    ClusterNoAgency(1450, "Will be raised when none of the agency servers can be connected to."),
    ClusterNoCoordinatorHeader(1451, "Will be raised when a DB server in a cluster receives a HTTP request without a coordinator header."),
    ClusterCouldNotLockPlan(1452, "Will be raised when a coordinator in a cluster cannot lock the Plan hierarchy in the agency."),
    ClusterCollectionIdExists(1453, "Will be raised when a coordinator in a cluster tries to create a collection and the collection ID already exists."),
    ClusterCouldNotCreateCollectionInPlan(1454, "Will be raised when a coordinator in a cluster cannot create an entry for a new collection in the Plan hierarchy in the agency."),
    ClusterCouldNotReadCurrentVersion(1455, "Will be raised when a coordinator in a cluster cannot read the Version entry in the Current hierarchy in the agency."),
    ClusterCouldNotCreateCollection(1456, "Will be raised when a coordinator in a cluster notices that some DBServers report problems when creating shards for a new collection."),
    ClusterTimeout(1457, "Will be raised when a coordinator in a cluster runs into a timeout for some cluster wide operation."),
    ClusterCouldNotRemoveCollectionInPlan(1458, "Will be raised when a coordinator in a cluster cannot remove an entry for a collection in the Plan hierarchy in the agency."),
    ClusterCouldNotRemoveCollectionInCurrent(1459, "Will be raised when a coordinator in a cluster cannot remove an entry for a collection in the Current hierarchy in the agency."),
    ClusterCouldNotCreateDatabaseInPlan(1460, "Will be raised when a coordinator in a cluster cannot create an entry for a new database in the Plan hierarchy in the agency."),
    ClusterCouldNotCreateDatabase(1461, "Will be raised when a coordinator in a cluster notices that some DBServers report problems when creating databases for a new cluster wide database."),
    ClusterCouldNotRemoveDatabaseInPlan(1462, "Will be raised when a coordinator in a cluster cannot remove an entry for a database in the Plan hierarchy in the agency."),
    ClusterCouldNotRemoveDatabaseInCurrent(1463, "Will be raised when a coordinator in a cluster cannot remove an entry for a database in the Current hierarchy in the agency."),
    ClusterShardGone(1464, "Will be raised when a coordinator in a cluster cannot determine the shard that is responsible for a given document."),
    ClusterConnectionLost(1465, "Will be raised when a coordinator in a cluster loses an HTTP connection to a DBserver in the cluster whilst transferring data."),
    ClusterMustNotSpecifyKey(1466, "Will be raised when a coordinator in a cluster finds that the _key attribute was specified in a sharded collection the uses not only _key as sharding attribute."),
    ClusterGotContradictingAnswers(1467, "Will be raised if a coordinator in a cluster gets conflicting results from different shards, which should never happen."),
    ClusterNotAllShardingAttributesGiven(1468, "Will be raised if a coordinator tries to find out which shard is responsible for a partial document, but cannot do this because not all sharding attributes are specified."),
    ClusterMustNotChangeShardingAttributes(1469, "Will be raised if there is an attempt to update the value of a shard attribute."),
    ClusterUnsupported(1470, "Will be raised when there is an attempt to carry out an operation that is not supported in the context of a sharded collection."),
    ClusterOnlyOnCoordinator(1471, "Will be raised if there is an attempt to run a coordinator-only operation on a different type of node."),
    ClusterReadingPlanAgency(1472, "Will be raised if a coordinator or DBserver cannot read the Plan in the agency."),
    ClusterCouldNotTruncateCollection(1473, "Will be raised if a coordinator cannot truncate all shards of a cluster collection."),
    ClusterAqlCommunication(1474, "Will be raised if the internal communication of the cluster for AQL produces an error."),
    ArangoDocumentNotFoundOrShardingAttributesChanged(1475, "Will be raised when a document with a given identifier or handle is unknown, or if the sharding attributes have been changed in a REPLACE operation in the cluster."),
    ClusterCouldNotDetermineId(1476, "Will be raised if a cluster server at startup could not determine its own ID from the local info provided."),
    ClusterOnlyOnDbserver(1477, "Will be raised if there is an attempt to run a DBserver-only operation on a different type of node."),
    ClusterBackendUnavailable(1478, "Will be raised if a required db server can't be reached."),
    ClusterUnknownCallbackEndpoint(1479, "An endpoint couldn't be found"),
    ClusterAgencyStructureInvalid(1480, "The structure in the agency is invalid"),
    ClusterAqlCollectionOutOfSync(1481, "Will be raised if a collection needed during query execution is out of sync. This currently can only happen when using satellite collections"),
    ClusterCouldNotCreateIndexInPlan(1482, "Will be raised when a coordinator in a cluster cannot create an entry for a new index in the Plan hierarchy in the agency."),
    ClusterCouldNotDropIndexInPlan(1483, "Will be raised when a coordinator in a cluster cannot remove an index from the Plan hierarchy in the agency."),
    ClusterChainOfDistributeshardslike(1484, "Will be raised if one tries to create a collection with a distributeShardsLike attribute which points to another collection that also has one."),
    ClusterMustNotDropCollOtherDistributeshardslike(1485, "Will be raised if one tries to drop a collection to which another collection points with its distributeShardsLike attribute."),
    ClusterUnknownDistributeshardslike(1486, "Will be raised if one tries to create a collection which points to an unknown collection in its distributeShardsLike attribute."),
    ClusterInsufficientDbservers(1487, "Will be raised if one tries to create a collection with a replicationFactor greater than the available number of DBServers."),
    ClusterCouldNotDropFollower(1488, "Will be raised if a follower that ought to be dropped could not be dropped in the agency (under Current)."),
    ClusterShardLeaderRefusesReplication(1489, "Will be raised if a replication operation is refused by a shard leader."),
    ClusterShardFollowerRefusesOperation(1490, "Will be raised if a non-replication operation is refused by a shard follower."),
    ClusterShardLeaderResigned(1491, "Will be raised if a non-replication operation is refused by a former shard leader that has found out that it is no longer the leader."),
    ClusterAgencyCommunicationFailed(1492, "Will be raised if after various retries an agency operation could not be performed successfully."),
    ClusterDistributeShardsLikeReplicationFactor(1493, "Will be raised if intended replication factor does not match that of the prototype shard given in ditributeShardsLike parameter."),
    ClusterDistributeShardsLikeNumberOfShards(1494, "Will be raised if intended number of shards does not match that of the prototype shard given in ditributeShardsLike parameter."),

    // ArangoDB query errors

    QueryKilled(1500, "Will be raised when a running query is killed by an explicit admin command."),
    QueryParse(1501, "Will be raised when query is parsed and is found to be syntactically invalid."),
    QueryEmpty(1502, "Will be raised when an empty query is specified."),
    QueryScript(1503, "Will be raised when a runtime error is caused by the query."),
    QueryNumberOutOfRange(1504, "Will be raised when a number is outside the expected range."),
    QueryVariableNameInvalid(1510, "Will be raised when an invalid variable name is used."),
    QueryVariableRedeclared(1511, "Will be raised when a variable gets re-assigned in a query."),
    QueryVariableNameUnknown(1512, "Will be raised when an unknown variable is used or the variable is undefined the context it is used."),
    QueryCollectionLockFailed(1521, "Will be raised when a read lock on the collection cannot be acquired."),
    QueryTooManyCollections(1522, "Will be raised when the number of collections in a query is beyond the allowed value."),
    QueryDocumentAttributeRedeclared(1530, "Will be raised when a document attribute is re-assigned."),
    QueryFunctionNameUnknown(1540, "Will be raised when an undefined function is called."),
    QueryFunctionArgumentNumberMismatch(1541, "Will be raised when the number of arguments used in a function call does not match the expected number of arguments for the function."),
    QueryFunctionArgumentTypeMismatch(1542, "Will be raised when the type of an argument used in a function call does not match the expected argument type."),
    QueryInvalidRegex(1543, "Will be raised when an invalid regex argument value is used in a call to a function that expects a regex."),
    QueryBindParametersInvalid(1550, "Will be raised when the structure of bind parameters passed has an unexpected format."),
    QueryBindParameterMissing(1551, "Will be raised when a bind parameter was declared in the query but the query is being executed with no value for that parameter."),
    QueryBindParameterUndeclared(1552, "Will be raised when a value gets specified for an undeclared bind parameter."),
    QueryBindParameterType(1553, "Will be raised when a bind parameter has an invalid value or type."),
    QueryInvalidLogicalValue(1560, "Will be raised when a non-boolean value is used in a logical operation."),
    QueryInvalidArithmeticValue(1561, "Will be raised when a non-numeric value is used in an arithmetic operation."),
    QueryDivisionByZero(1562, "Will be raised when there is an attempt to divide by zero."),
    QueryArrayExpected(1563, "Will be raised when a non-array operand is used for an operation that expects an array argument operand."),
    QueryFailCalled(1569, "Will be raised when the function FAIL() is called from inside a query."),
    QueryGeoIndexMissing(1570, "Will be raised when a geo restriction was specified but no suitable geo index is found to resolve it."),
    QueryFulltextIndexMissing(1571, "Will be raised when a fulltext query is performed on a collection without a suitable fulltext index."),
    QueryInvalidDateValue(1572, "Will be raised when a value cannot be converted to a date."),
    QueryMultiModify(1573, "Will be raised when an AQL query contains more than one data-modifying operation."),
    QueryInvalidAggregateExpression(1574, "Will be raised when an AQL query contains an invalid aggregate expression."),
    QueryCompileTimeOptions(1575, "Will be raised when an AQL data-modification query contains options that cannot be figured out at query compile time."),
    QueryExceptionOptions(1576, "Will be raised when an AQL data-modification query contains an invalid options specification."),
    QueryCollectionUsedInExpression(1577, "Will be raised when a collection is used as an operand in an AQL expression."),
    QueryDisallowedDynamicCall(1578, "Will be raised when a dynamic function call is made to a function that cannot be called dynamically."),
    QueryAccessAfterModification(1579, "Will be raised when collection data are accessed after a data-modification operation."),

    // AQL user function errors

    QueryFunctionInvalidName(1580, "Will be raised when a user function with an invalid name is registered."),
    QueryFunctionInvalidCode(1581, "Will be raised when a user function is registered with invalid code."),
    QueryFunctionNotFound(1582, "Will be raised when a user function is accessed but not found."),
    QueryFunctionRuntimeError(1583, "Will be raised when a user function throws a runtime exception."),

    // AQL query registry errors

    QueryBadJsonPlan(1590, "Will be raised when an HTTP API for a query got an invalid JSON object."),
    QueryNotFound(1591, "Will be raised when an Id of a query is not found by the HTTP API."),
    QueryInUse(1592, "Will be raised when an Id of a query is found by the HTTP API but the query is in use."),

    // ArangoDB cursor errors

    CursorNotFound(1600, "Will be raised when a cursor is requested via its id but a cursor with that id cannot be found."),
    CursorBusy(1601, "Will be raised when a cursor is requested via its id but a concurrent request is still using the cursor."),

    // ArangoDB transaction errors

    TransactionInternal(1650, "Will be raised when a wrong usage of transactions is detected. this is an internal error and indicates a bug in ArangoDB."),
    TransactionNested(1651, "Will be raised when transactions are nested."),
    TransactionUnregisteredCollection(1652, "Will be raised when a collection is used in the middle of a transaction but was not registered at transaction start."),
    TransactionDisallowedOperation(1653, "Will be raised when a disallowed operation is carried out in a transaction."),
    TransactionAborted(1654, "Will be raised when a transaction was aborted."),

    // User management errors

    UserInvalidName(1700, "Will be raised when an invalid user name is used."),
    UserInvalidPassword(1701, "Will be raised when an invalid password is used."),
    UserDuplicate(1702, "Will be raised when a user name already exists."),
    UserNotFound(1703, "Will be raised when a user name is updated that does not exist."),
    UserChangePassword(1704, "Will be raised when the user must change his password."),

    // Service management errors (legacy)
    // These have been superceded by the Foxx management errors in public APIs.

    ServiceInvalidName(1750, "Will be raised when an invalid service name is specified."),
    ServiceInvalidMount(1751, "Will be raised when an invalid mount is specified."),
    ServiceDownloadFailed(1752, "Will be raised when a service download from the central repository failed."),
    ServiceUploadFailed(1753, "Will be raised when a service upload from the client to the ArangoDB server failed."),

    // LDAP errors

    LdapCannotInit(1800, "can not init a LDAP connection"),
    LdapCannotSetOption(1801, "can not set a LDAP option"),
    LdapCannotBind(1802, "can not bind to a LDAP server"),
    LdapCannotUnbind(1803, "can not unbind from a LDAP server"),
    LdapCannotSearch(1804, "can not search the LDAP server"),
    LdapCannotStartTls(1805, "can not star a TLS LDAP session"),
    LdapFoundNoObjects(1806, "LDAP didn't found any objects with the specified search query"),
    LdapNotOneUserFound(1807, "LDAP found zero ore more than one user"),
    LdapUserNotIdentified(1808, "LDAP found a user, but its not the desired one"),
    LdapInvalidMode(1820, "cant distinguish a valid mode for provided ldap configuration"),

    // Task errors

    TaskInvalidId(1850, "Will be raised when a task is created with an invalid id."),
    TaskDuplicateId(1851, "Will be raised when a task id is created with a duplicate id."),
    TaskNotFound(1852, "Will be raised when a task with the specified id could not be found."),

    // Graph / traversal errors

    GraphInvalidGraph(1901, "Will be raised when an invalid name is passed to the server."),
    GraphCouldNotCreateGraph(1902, "Will be raised when an invalid name, vertices or edges is passed to the server."),
    GraphInvalidVertex(1903, "Will be raised when an invalid vertex id is passed to the server."),
    GraphCouldNotCreateVertex(1904, "Will be raised when the vertex could not be created."),
    GraphCouldNotChangeVertex(1905, "Will be raised when the vertex could not be changed."),
    GraphInvalidEdge(1906, "Will be raised when an invalid edge id is passed to the server."),
    GraphCouldNotCreateEdge(1907, "Will be raised when the edge could not be created."),
    GraphCouldNotChangeEdge(1908, "Will be raised when the edge could not be changed."),
    GraphTooManyIterations(1909, "Will be raised when too many iterations are done in a graph traversal."),
    GraphInvalidFilterResult(1910, "Will be raised when an invalid filter result is returned in a graph traversal."),
    GraphCollectionMultiUse(1920, "An edge collection may only be used once in one edge definition of a graph."),
    GraphCollectionUseInMultiGraphs(1921, "Is already used by another graph in a different edge definition."),
    GraphCreateMissingName(1922, "A graph name is required to create a graph."),
    GraphCreateMalformedEdgeDefinition(1923, "The edge definition is malformed. It has to be an array of objects."),
    GraphNotFound(1924, "A graph with this name could not be found."),
    GraphDuplicate(1925, "A graph with this name already exists."),
    GraphVertexColDoesNotExist(1926, "The specified vertex collection does not exist or is not part of the graph."),
    GraphWrongCollectionTypeVertex(1927, "The collection is not a vertex collection."),
    GraphNotInOrphanCollection(1928, "Vertex collection not in orphan collection of the graph."),
    GraphCollectionUsedInEdgeDef(1929, "The collection is already used in an edge definition of the graph."),
    GraphEdgeCollectionNotUsed(1930, "The edge collection is not used in any edge definition of the graph."),
    GraphNotAnArangoCollection(1931, "The collection is not an ArangoCollection."),
    GraphNoGraphCollection(1932, "Collection _graphs does not exist."),
    GraphInvalidExampleArrayObjectString(1933, "Invalid example type. Has to be String, Array or Object."),
    GraphInvalidExampleArrayObject(1934, "Invalid example type. Has to be Array or Object."),
    GraphInvalidNumberOfArguments(1935, "Invalid number of arguments. Expected: "),
    GraphInvalidParameter(1936, "Invalid parameter type."),
    GraphInvalidId(1937, "Invalid id"),
    GraphCollectionUsedInOrphans(1938, "The collection is already used in the orphans of the graph."),
    GraphEdgeColDoesNotExist(1939, "The specified edge collection does not exist or is not part of the graph."),
    GraphEmpty(1940, "The requested graph has no edge collections."),

    // Session errors

    SessionUnknown(1950, "Will be raised when an invalid/unknown session id is passed to the server."),
    SessionExpired(1951, "Will be raised when a session is expired."),

    // Simple Client errors

    SimpleClientUnknownError(2000, "This error should not happen."),
    SimpleClientCouldNotConnect(2001, "Will be raised when the client could not connect to the server."),
    SimpleClientCouldNotWrite(2002, "Will be raised when the client could not write data."),
    SimpleClientCouldNotRead(2003, "Will be raised when the client could not read data."),

    // Communicator errors

    CommunicatorRequestAborted(2100, "Request was aborted."),
    CommunicatorDisabled(2101, "Communication was disabled."),

    // Foxx management errors

    MalformedManifestFile(3000, "The service manifest file is not well-formed JSON."),
    InvalidServiceManifest(3001, "The service manifest contains invalid values."),
    ServiceFilesMissing(3002, "The service folder or bundle does not exist on this server."),
    ServiceFilesOutdated(3003, "The local service bundle does not match the checksum in the database."),
    InvalidFoxxOptions(3004, "The service options contain invalid values."),
    InvalidMountpoint(3007, "The service mountpath contains invalid characters."),
    ServiceNotFound(3009, "No service found at the given mountpath."),
    ServiceNeedsConfiguration(3010, "The service is missing configuration or dependencies."),
    ServiceMountpointConflict(3011, "A service already exists at the given mountpath."),
    ServiceManifestNotFound(3012, "The service directory does not contain a manifest file."),
    ServiceOptionsMalformed(3013, "The service options are not well-formed JSON."),
    ServiceSourceNotFound(3014, "The source path does not match a file or directory."),
    ServiceSourceError(3015, "The source path could not be resolved."),
    ServiceUnknownScript(3016, "The service does not have a script with this name."),

    // JavaScript module loader errors

    ModuleNotFound(3100, "The module path could not be resolved."),
    ModuleSyntaxError(3101, "The module could not be parsed because of a syntax error."),
    ModuleFailure(3103, "Failed to invoke the module in its context."),

    // Enterprise errors

    NoSmartCollection(4000, "The requested collection needs to be smart, but it ain't"),
    NoSmartGraphAttribute(4001, "The given document does not have the smart graph attribute set."),
    CannotDropSmartCollection(4002, "This smart collection cannot be dropped, it dictates sharding in the graph."),
    KeyMustBePrefixedWithSmartGraphAttribute(4003, "In a smart vertex collection _key must be prefixed with the value of the smart graph attribute."),
    IllegalSmartGraphAttribute(4004, "The given smartGraph attribute is illegal and connot be used for sharding. All system attributes are forbidden."),

    // Agency errors

    AgencyInquirySyntax(20001, "Inquiry handles a list of string clientIds: [,...]."),
    AgencyInformMustBeObject(20011, "The inform message in the agency must be an object."),
    AgencyInformMustContainTerm(20012, "The inform message in the agency must contain a uint parameter 'term'."),
    AgencyInformMustContainId(20013, "The inform message in the agency must contain a string parameter 'id'."),
    AgencyInformMustContainActive(20014, "The inform message in the agency must contain an array 'active'."),
    AgencyInformMustContainPool(20015, "The inform message in the agency must contain an object 'pool'."),
    AgencyInformMustContainMinPing(20016, "The inform message in the agency must contain an object 'min ping'."),
    AgencyInformMustContainMaxPing(20017, "The inform message in the agency must contain an object 'max ping'."),
    AgencyInformMustContainTimeoutMult(20018, "The inform message in the agency must contain an object 'timeoutMult'."),
    AgencyInquireClientIdMustBeString(20020, "Inquiry by clientId failed"),
    AgencyCannotRebuildDbs(20021, "Will be raised if the readDB or the spearHead cannot be rebuilt from the replicated log."),

    // Supervision errors

    SupervisionGeneralFailure(20501, "General supervision failure."),

    // Dispatcher errors

    DispatcherIsStopping(21001, "Will be returned if a shutdown is in progress."),
    QueueUnknown(21002, "Will be returned if a queue with this name does not exist."),
    QueueFull(21003, "Will be returned if a queue with this name is full."),
}


