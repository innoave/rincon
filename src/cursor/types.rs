
use std::collections::HashMap;
use std::mem;

use api::query::{Query, Value};
use api::types::JsonValue;

/// A temporary cursor for retrieving query results.
///
/// If the result set contains more documents than should be transferred in a
/// single round-trip (i.e. as set via the `batch_size` attribute), the server
/// will return the first few documents and create a temporary cursor. The
/// cursor identifier will also be returned to the client. The server will put
/// the cursor identifier in the id attribute of the response object.
/// Furthermore, the `has_more` attribute of the response object will be set to
/// true. This is an indication for the client that there are additional
/// results to fetch from the server.
///
/// The cursor will automatically be destroyed on the server when the client
/// has retrieved all documents from it. The client can also explicitly destroy
/// the cursor at any earlier time using the `DeleteCursor` method.
///
/// **Note**: the server will also destroy abandoned cursors automatically
/// after a certain server-controlled timeout to avoid resource leakage.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cursor<T> {
    /// The id of the cursor created on the server.
    id: Option<String>,

    /// An array of result documents (might be empty if query has no results).
    result: Vec<T>,

    /// A boolean indicator whether there are more results available for the
    /// cursor on the server.
    has_more: bool,

    /// The total number of result documents available (only available if the
    /// query was executed with the count attribute set).
    count: Option<u64>,

    /// A boolean flag indicating whether the query result was served from the
    /// query cache or not. If the query result is served from the query cache,
    /// the extra return attribute will not contain any stats sub-attribute and
    /// no profile sub-attribute.
    cached: bool,

    /// An optional JSON object with extra information about the query result
    /// contained in its stats sub-attribute. For data-modification queries,
    /// the extra.stats sub-attribute will contain the number of modified
    /// documents and the number of documents that could not be modified due
    /// to an error (if ignoreErrors query option is specified).
    extra: Option<CursorExtra>,
}

impl<T> Cursor<T> {
    /// Returns the id of the cursor created on the server.
    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Returns an array of result documents (might be empty if query has no
    /// results).
    pub fn result(&self) -> &[T] {
        &self.result
    }

    /// Returns whether there are more results available for this cursor on
    /// the server.
    pub fn has_more(&self) -> bool {
        self.has_more
    }

    /// Returns the total number of result documents available (only available
    /// if the query was executed with the count attribute set).
    pub fn count(&self) -> Option<u64> {
        self.count
    }

    /// Returns whether the query result was served from the query cache or not.
    ///
    /// If the query result is served from the query cache, the extra return
    /// attribute will not contain any stats sub-attribute and no profile
    /// sub-attribute.
    pub fn is_cached(&self) -> bool {
        self.cached
    }

    /// Returns an optional JSON object with extra information about the query
    /// result contained in its `stats` sub-attribute. For data-modification
    /// queries, the `extra.stats` sub-attribute will contain the number of
    /// modified documents and the number of documents that could not be
    /// modified due to an error (if `ignoreErrors` query option is specified).
    pub fn extra(&self) -> Option<&CursorExtra> {
        self.extra.as_ref()
    }
}

//TODO find a suitable type for warnings
pub type Warning = JsonValue;

/// Holds extra information about the query execution.
#[derive(Clone, Debug, Deserialize)]
pub struct CursorExtra {
    /// Statistics about the query execution.
    stats: CursorStatistics,
    /// Warnings that occurred during query execution.
    warnings: Vec<Warning>,
}

impl CursorExtra {
    /// Returns the statistics about the query execution.
    pub fn stats(&self) -> &CursorStatistics {
        &self.stats
    }

    /// Returns warnings that occurred during query execution.
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }
}

/// Holds statistics information about the query execution.
#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorStatistics {
    /// The execution time of the query.
    execution_time: f64,
    /// The number of results that have been filtered out.
    filtered: u64,
    /// The total number of results that have been found.
    full_count: Option<u64>,
    /// The number of http requests.
    http_requests: u64,
    /// The number of full scans.
    scanned_full: u64,
    /// The number of index scans.
    scanned_index: u64,
    /// The number of write operations that have been executed.
    writes_executed: u64,
    /// The number of write operations that have been ignored.
    writes_ignored: u64,
}

impl CursorStatistics {
    /// Returns the time the execution of the query took.
    pub fn execution_time(&self) -> f64 {
        self.execution_time
    }

    /// Returns the number of results that have been filtered out.
    pub fn filtered(&self) -> u64 {
        self.filtered
    }

    /// Returns the total number of results that have been found.
    ///
    /// This property is only available if the option `CursorOptions.full_count`
    /// parameter has been set to `true` for this query.
    pub fn full_count(&self) -> Option<u64> {
        self.full_count
    }

    /// Returns the number of http request.
    pub fn http_requests(&self) -> u64 {
        self.http_requests
    }

    /// Returns the number of full scans.
    pub fn scanned_full(&self) -> u64 {
        self.scanned_full
    }

    /// Returns the number of index scans.
    pub fn scanned_index(&self) -> u64 {
        self.scanned_index
    }

    /// Returns the number of write operation that have been executed.
    pub fn writes_executed(&self) -> u64 {
        self.writes_executed
    }

    /// Returns the number of write operations that have been ignored.
    pub fn writes_ignored(&self) -> u64 {
        self.writes_ignored
    }
}

/// This struct defines the parameters of a cursor for an AQL query that is
/// to be created.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCursor {
    /// Contains the query string to be executed.
    query: String,

    /// key/value pairs representing the bind parameters.
    bind_vars: HashMap<String, Value>,

    /// Indicates whether the number of documents in the result set should be
    /// returned in the "count" attribute of the result. Calculating the
    /// 'count' attribute might have a performance impact for some queries in
    /// the future so this option is turned off by default, and 'count' is only
    /// returned when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<bool>,

    /// Maximum number of result documents to be transferred from the server to
    /// the client in one round-trip. If this attribute is not set, a
    /// server-controlled default value will be used. A batchSize value of 0 is
    /// disallowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<u32>,

    /// A flag to determine whether the AQL query cache shall be used. If set to
    /// false, then any query cache lookup will be skipped for the query. If set
    /// to true, it will lead to the query cache being checked for the query if
    /// the query cache mode is either on or demand.
    #[serde(skip_serializing_if = "Option::is_none")]
    cache: Option<bool>,

    /// The maximum number of memory (measured in bytes) that the query is
    /// allowed to use. If set, then the query will fail with error 'resource
    /// limit exceeded' in case it allocates too much memory. A value of 0
    /// indicates that there is no memory limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_limit: Option<u64>,

    /// The time-to-live for the cursor (in seconds). The cursor will be removed
    /// on the server automatically after the specified amount of time. This is
    /// useful to ensure garbage collection of cursors that are not fully
    /// fetched by clients. If not set, a server-defined value will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u32>,

    /// Optional parameters for tweaking query execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<CursorOptions>,
}

impl NewCursor {
    /// Constructs a new instance of `NewCursor` from the given `Query`.
    pub fn new(query: Query) -> Self {
        let (query, bind_vars) = query.deconstruct();
        NewCursor {
            query,
            bind_vars,
            count: None,
            batch_size: None,
            cache: None,
            memory_limit: None,
            ttl: None,
            options: None,
        }
    }

    /// Returns the query string to be executed.
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Returns the bind parameters of this `Query`.
    pub fn bind_vars(&self) -> &HashMap<String, Value> {
        &self.bind_vars
    }

    /// Sets the flag whether number of documents in the result set should be
    /// returned.
    ///
    /// Calculating the 'count' attribute might have a performance impact for
    /// some queries in the future so this option is turned off by default, and
    /// 'count' is only returned when requested.
    pub fn set_count(&mut self, count: Option<bool>) {
        self.count = count;
    }

    /// Returns whether the number of documents in the result set should be
    /// returned in the "count" attribute of the result.
    pub fn is_count(&self) -> Option<bool> {
        self.count
    }

    /// Sets the maximum number of result documents to be transferred from the
    /// server to the client in one round-trip.
    ///
    /// If this attribute is not set, a server-controlled default value will be
    /// used. A batchSize value of 0 is disallowed.
    pub fn set_batch_size(&mut self, batch_size: Option<u32>) {
        self.batch_size = batch_size;
    }

    /// Returns the maximum number of result documents to be transferred from
    /// the server to the client in one round-trip.
    pub fn batch_size(&self) -> Option<u32> {
        self.batch_size
    }

    /// Sets the flag that indicates whether the AQL query cache shall be used.
    ///
    /// If set to false, then any query cache lookup will be skipped for the
    /// query. If set to true, it will lead to the query cache being checked
    /// for the query if the query cache mode is either on or demand.
    pub fn set_cache(&mut self, cache: Option<bool>) {
        self.cache = cache
    }

    /// Returns whether the AQL query cache shall be used.
    pub fn is_cache(&self) -> Option<bool> {
        self.cache
    }

    /// Sets the maximum number of memory (measured in bytes) that the query
    /// is allowed to use.
    ///
    /// If set, then the query will fail with error 'resource limit exceeded'
    /// in case it allocates too much memory. A value of 0 indicates that there
    /// is no memory limit.
    pub fn set_memory_limit(&mut self, memory_limit: Option<u64>) {
        self.memory_limit = memory_limit;
    }

    /// Returns the maximum number of memory (measured in bytes) that the query
    /// is allowed to use.
    pub fn memory_limit(&self) -> Option<u64> {
        self.memory_limit
    }

    /// Sets the time-to-live for the cursor (in seconds).
    ///
    /// The cursor will be removed on the server automatically after the
    /// specified amount of time. This is useful to ensure garbage collection
    /// of cursors that are not fully fetched by clients. If not set, a
    /// server-defined value will be used.
    pub fn set_ttl(&mut self, ttl: Option<u32>) {
        self.ttl = ttl;
    }

    /// Returns the time-to-live for the cursor (in seconds).
    pub fn ttl(&self) -> Option<u32> {
        self.ttl
    }

    /// Returns the optional cursor options as mutable reference for changing
    /// the optional cursor options to tweak query execution.
    pub fn options_mut(&mut self) -> &mut CursorOptions {
        self.options.get_or_insert_with(|| CursorOptions::new())
    }

    /// Removes the currently set options from this instance and returns them.
    pub fn remove_options(&mut self) -> Option<CursorOptions> {
        mem::replace(&mut self.options, None)
    }

    /// Returns the optional cursor options for tweaking query execution.
    pub fn options(&self) -> Option<&CursorOptions> {
        self.options.as_ref()
    }
}

impl From<Query> for NewCursor {
    fn from(query: Query) -> Self {
        NewCursor::new(query)
    }
}

/// Optional parameters for tweaking query execution.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorOptions {
    /// When set to true, the query will throw an exception and abort instead of
    /// producing a warning. This option should be used during development to
    /// catch potential issues early. When the attribute is set to false,
    /// warnings will not be propagated to exceptions and will be returned with
    /// the query result. There is also a server configuration option
    /// `--query.fail-on-warning` for setting the default value for
    /// `fail_on_warning` so it does not need to be set on a per-query level.
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_on_warning: Option<bool>,

    /// If set to true, then the additional query profiling information will
    /// be returned in the sub-attribute profile of the extra return attribute
    /// if the query result is not served from the query cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<bool>,

    /// Limits the maximum number of warnings a query will return. The number
    /// of warnings a query will return is limited to 10 by default, but that
    /// number can be increased or decreased by setting this attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_warning_count: Option<u32>,

    /// If set to true and the query contains a LIMIT clause, then the result
    /// will have an extra attribute with the sub-attributes stats and
    /// fullCount, `{ ... , "extra": { "stats": { "fullCount": 123 } } }`. The
    /// fullCount attribute will contain the number of documents in the result
    /// before the last LIMIT in the query was applied. It can be used to count
    /// the number of documents that match certain filter criteria, but only
    /// return a subset of them, in one go. It is thus similar to MySQL's
    /// `SQL_CALC_FOUND_ROWS` hint. Note that setting the option will disable a
    /// few LIMIT optimizations and may lead to more documents being processed,
    /// and thus make queries run longer. Note that the fullCount attribute will
    /// only be present in the result if the query has a LIMIT clause and the
    /// LIMIT clause is actually used in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    full_count: Option<bool>,

    /// Limits the maximum number of plans that are created by the AQL query
    /// optimizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_plans: Option<u32>,

    /// A list of to-be-included or to-be-excluded optimizer rules can be put
    /// into this attribute, telling the optimizer to include or exclude
    /// specific rules. To disable a rule, prefix its name with a `-`, to
    /// enable a rule, prefix it with a `+`. There is also a pseudo-rule `all`,
    /// which will match all optimizer rules.
    #[serde(rename = "optimizer.rules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    optimizer_rules: Option<Vec<String>>,

    /// Maximum number of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediate_commit_count: Option<u32>,

    /// Maximum total size of operations after which an intermediate commit is
    /// performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediate_commit_size: Option<u32>,

    /// Transaction size limit in bytes.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_transaction_size: Option<u32>,

    /// This enterprise parameter allows to configure how long a DBServer will
    /// have time to bring the satellite collections involved in the query into
    /// sync. The default value is 60.0 (seconds). When the max time has been
    /// reached the query will be stopped.
    #[cfg(feature = "enterprise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    satellite_sync_wait: Option<bool>,
}

impl CursorOptions {
    /// Constructs a new instance of an empty `CursorOptions` struct.
    ///
    /// All fields are set to `None`.
    pub fn new() -> Self {
        CursorOptions {
            fail_on_warning: None,
            profile: None,
            max_warning_count: None,
            full_count: None,
            max_plans: None,
            optimizer_rules: None,
            #[cfg(feature = "rocksdb")]
            intermediate_commit_count: None,
            #[cfg(feature = "rocksdb")]
            intermediate_commit_size: None,
            #[cfg(feature = "rocksdb")]
            max_transaction_size: None,
            #[cfg(feature = "enterprise")]
            satellite_sync_wait: None,
        }
    }

    /// Sets the flag indicating whether the query shall fail on warnings.
    ///
    /// When set to true, the query will throw an exception and abort instead of
    /// producing a warning. This option should be used during development to
    /// catch potential issues early. When the attribute is set to false,
    /// warnings will not be propagated to exceptions and will be returned with
    /// the query result. There is also a server configuration option
    /// `--query.fail-on-warning` for setting the default value for
    /// `fail_on_warning` so it does not need to be set on a per-query level.
    pub fn set_fail_on_warning(&mut self, fail_on_warning: Option<bool>) {
        self.fail_on_warning = fail_on_warning;
    }

    /// Returns whether the query shall fail on warnings.
    pub fn is_fail_on_warning(&self) -> Option<bool> {
        self.fail_on_warning
    }

    /// Sets the flag indicating whether additional query profiling information
    /// shall be returned.
    ///
    /// If set to true, then the additional query profiling information will
    /// be returned in the sub-attribute profile of the extra return attribute
    /// if the query result is not served from the query cache.
    pub fn set_profile(&mut self, profile: Option<bool>) {
        self.profile = profile;
    }

    /// Returns whether additional query profiling information shall be
    /// returned.
    pub fn is_profile(&self) -> Option<bool> {
        self.profile
    }

    /// Sets the maximum number of warnings a query will return.
    ///
    /// The number of warnings a query will return is limited to 10 by default,
    /// but that number can be increased or decreased by setting this attribute.
    pub fn set_max_warning_count(&mut self, max_warning_count: Option<u32>) {
        self.max_warning_count = max_warning_count;
    }

    /// Returns the maximum number of warnings a query will return.
    pub fn max_warning_count(&self) -> Option<u32> {
        self.max_warning_count
    }

    /// Set the flag indicating whether full count and stats should be returned.
    ///
    /// If set to true and the query contains a LIMIT clause, then the result
    /// will have an extra attribute with the sub-attributes stats and
    /// fullCount, `{ ... , "extra": { "stats": { "fullCount": 123 } } }`. The
    /// fullCount attribute will contain the number of documents in the result
    /// before the last LIMIT in the query was applied. It can be used to count
    /// the number of documents that match certain filter criteria, but only
    /// return a subset of them, in one go. It is thus similar to MySQL's
    /// `SQL_CALC_FOUND_ROWS` hint. Note that setting the option will disable a
    /// few LIMIT optimizations and may lead to more documents being processed,
    /// and thus make queries run longer. Note that the fullCount attribute will
    /// only be present in the result if the query has a LIMIT clause and the
    /// LIMIT clause is actually used in the query.
    pub fn set_full_count(&mut self, full_count: Option<bool>) {
        self.full_count = full_count;
    }

    /// Returns whether full count and stats should be returned.
    pub fn is_full_count(&self) -> Option<bool> {
        self.full_count
    }

    /// Sets the maximum number of plans that are created by the AQL query
    /// optimizer.
    pub fn set_max_plans(&mut self, max_plans: Option<u32>) {
        self.max_plans = max_plans;
    }

    /// Returns the maximum number of plans that are created by the AQL query
    /// optimizer.
    pub fn max_plans(&self) -> Option<u32> {
        self.max_plans
    }

    /// Returns a mutable reference to the list of to-be-included or
    /// to-be-excluded optimizer rules.
    ///
    /// To disable a rule, prefix its name with a `-`, to enable a rule, prefix
    /// it with a `+`. There is also a pseudo-rule `all`, which will match all
    /// optimizer rules.
    pub fn optimizer_rules_mut(&mut self) -> &mut Vec<String> {
        self.optimizer_rules.get_or_insert_with(|| Vec::new())
    }

    /// Removes the optimizer rules from this instance and returns them.
    pub fn remove_optimizer_rules(&mut self) -> Option<Vec<String>> {
        mem::replace(&mut self.optimizer_rules, None)
    }

    /// Returns the list of to-be-included or to-be-excluded optimizer rules,
    /// that are telling the optimizer to include or exclude specific rules.
    pub fn optimizer_rules(&self) -> Option<&Vec<String>> {
        self.optimizer_rules.as_ref()
    }

    /// Sets the maximum number of operations after which an intermediate
    /// commit is performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn set_intermediate_commit_count(&mut self, intermediate_commit_count: Option<u32>) {
        self.intermediate_commit_count = intermediate_commit_count;
    }

    /// Returns the maximum number of operations after which an intermediate
    /// commit is performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn intermediate_commit_count(&self) -> Option<u32> {
        self.intermediate_commit_count
    }

    /// Sets the maximum total size of operations after which an intermediate
    /// commit is performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn set_intermediate_commit_size(&mut self, intermediate_commit_size: Option<u32>) {
        self.intermediate_commit_size = intermediate_commit_size
    }

    /// Returns the maximum total size of operations after which an intermediate
    /// commit is performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn intermediate_commit_size(&self) -> Option<u32> {
        self.intermediate_commit_size
    }

    /// Sets the transaction size limit in bytes.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn set_max_transaction_size(&mut self, max_transaction_size: Option<u32>) {
        self.max_transaction_size = max_transaction_size;
    }

    /// Returns the transaction size limit in bytes.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn max_transaction_size(&self) -> Option<u32> {
        self.max_transaction_size
    }

    /// Sets the enterprise parameter that configures how long a DBServer will
    /// have time to bring the satellite collections involved in the query into
    /// sync.
    ///
    /// The default value is 60.0 (seconds). When the max time has been reached
    /// the query will be stopped.
    #[cfg(feature = "enterprise")]
    pub fn set_satellite_sync_wait(&mut self, satellite_sync_wait: Option<bool>) {
        self.satellite_sync_wait = satellite_sync_wait
    }

    /// Returns the enterprise parameter that configures how long a DBServer
    /// will have time to bring the satellite collections involved in the query
    /// into sync.
    #[cfg(feature = "enterprise")]
    pub fn satellite_sync_wait(&self) -> Option<bool> {
        self.satellite_sync_wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_query_into_new_cursor_to_be_created() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());
        let query = query;

        let new_cursor: NewCursor = query.into();

        assert_eq!("FOR u IN users FILTER u.name = @name RETURN u.name", new_cursor.query);
        assert_eq!(Some(&Value::String("simone".to_owned())), new_cursor.bind_vars.get("name"));
    }

    #[test]
    fn set_optional_cursor_options_of_a_newly_initialized_new_cursor() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());
        let query = query;

        let mut new_cursor = NewCursor::from(query);
        assert!(new_cursor.options().is_none());
        {
            let cursor_options = new_cursor.options_mut();
            cursor_options.optimizer_rules_mut().push("+use-indexes".to_owned());
        }
        let new_cursor = new_cursor;

        assert!(new_cursor.options().is_some());
        assert!(new_cursor.options().unwrap().optimizer_rules().unwrap().contains(&"+use-indexes".to_owned()));
    }
}
