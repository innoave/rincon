
use std::collections::{HashMap, HashSet};
use std::iter::{ExactSizeIterator, Iterator};
use std::collections::hash_set::Iter;
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
    pub fn with_count<C>(&mut self, count: C) -> &mut Self
        where C: Into<Option<bool>>
    {
        self.count = count.into();
        self
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
    pub fn with_batch_size<S>(&mut self, batch_size: S) -> &mut Self
        where S: Into<Option<u32>>
    {
        self.batch_size = batch_size.into();
        self
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
    pub fn with_cache<C>(&mut self, cache: C) -> &mut Self
        where C: Into<Option<bool>>
    {
        self.cache = cache.into();
        self
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
    pub fn with_memory_limit<L>(&mut self, memory_limit: L) -> &mut Self
        where L: Into<Option<u64>>
    {
        self.memory_limit = memory_limit.into();
        self
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
    pub fn with_ttl<T>(&mut self, ttl: T) -> &mut Self
        where T: Into<Option<u32>>
    {
        self.ttl = ttl.into();
        self
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

    /// Removes the currently set options from this struct and returns them.
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
    optimizer_rules: Option<HashSet<String>>,

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
    fn new() -> Self {
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
    pub fn with_fail_on_warning<W>(&mut self, fail_on_warning: W) -> &mut Self
        where W: Into<Option<bool>>
    {
        self.fail_on_warning = fail_on_warning.into();
        self
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
    pub fn with_profile<P>(&mut self, profile: P) -> &mut Self
        where P: Into<Option<bool>>
    {
        self.profile = profile.into();
        self
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
    pub fn with_max_warning_count<C>(&mut self, max_warning_count: C) -> &mut Self
        where C: Into<Option<u32>>
    {
        self.max_warning_count = max_warning_count.into();
        self
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
    pub fn with_full_count<C>(&mut self, full_count: C) -> &mut Self
        where C: Into<Option<bool>>
    {
        self.full_count = full_count.into();
        self
    }

    /// Returns whether full count and stats should be returned.
    pub fn is_full_count(&self) -> Option<bool> {
        self.full_count
    }

    /// Sets the maximum number of plans that are created by the AQL query
    /// optimizer.
    pub fn with_max_plans<P>(&mut self, max_plans: P) -> &mut Self
        where P: Into<Option<u32>>
    {
        self.max_plans = max_plans.into();
        self
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
    pub fn optimizer_rules_mut(&mut self) -> OptimizerRuleSetMut {
        OptimizerRuleSetMut::new(self.optimizer_rules
            .get_or_insert_with(|| HashSet::new()))
    }

    /// Removes the optimizer rules from this instance.
    pub fn remove_optimizer_rules(&mut self) {
        self.optimizer_rules = None;
    }

    /// Returns the list of to-be-included or to-be-excluded optimizer rules,
    /// that are telling the optimizer to include or exclude specific rules.
    pub fn optimizer_rules(&self) -> Option<OptimizerRuleSet> {
        self.optimizer_rules.as_ref().map(OptimizerRuleSet::new)
    }

    /// Sets the maximum number of operations after which an intermediate
    /// commit is performed automatically.
    ///
    /// Honored by the RocksDB storage engine only.
    #[cfg(feature = "rocksdb")]
    pub fn with_intermediate_commit_count<C>(&mut self, intermediate_commit_count: C) -> &mut Self
        where C: Into<Option<u32>>
    {
        self.intermediate_commit_count = intermediate_commit_count.into();
        self
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
    pub fn with_intermediate_commit_size<S>(&mut self, intermediate_commit_size: S) -> &mut Self
        where S: Into<Option<u32>>
    {
        self.intermediate_commit_size = intermediate_commit_size.into();
        self
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
    pub fn with_max_transaction_size<S>(&mut self, max_transaction_size: S) -> &mut Self
        where S: Into<Option<u32>>
    {
        self.max_transaction_size = max_transaction_size.into();
        self
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
    pub fn with_satellite_sync_wait<W>(&mut self, satellite_sync_wait: W) -> &mut Self
        where W: Into<Option<bool>>
    {
        self.satellite_sync_wait = satellite_sync_wait.into();
        self
    }

    /// Returns the enterprise parameter that configures how long a DBServer
    /// will have time to bring the satellite collections involved in the query
    /// into sync.
    #[cfg(feature = "enterprise")]
    pub fn satellite_sync_wait(&self) -> Option<bool> {
        self.satellite_sync_wait
    }
}

#[derive(Debug)]
pub struct OptimizerRuleSet<'a> {
    rules: &'a HashSet<String>,
}

impl<'a> OptimizerRuleSet<'a> {
    fn new(rules: &'a HashSet<String>) -> Self {
        OptimizerRuleSet {
            rules,
        }
    }

    fn included(rule: &OptimizerRule) -> String {
       String::from("+") + rule.as_api_str()
    }

    fn excluded(rule: &OptimizerRule) -> String {
        String::from("-") + rule.as_api_str()
    }

    pub fn includes(&self, rule: &OptimizerRule) -> bool {
        self.rules.contains(&OptimizerRuleSet::included(&rule))
    }

    pub fn excludes(&self, rule: &OptimizerRule) -> bool {
        self.rules.contains(&OptimizerRuleSet::excluded(&rule))
    }

    pub fn into_iter(self) -> OptimizerRuleIntoIter<'a> {
        OptimizerRuleIntoIter {
            inner: self.rules.into_iter(),
        }
    }
}

#[derive(Debug)]
pub struct OptimizerRuleSetMut<'a> {
    rules: &'a mut HashSet<String>,
}

impl<'a> OptimizerRuleSetMut<'a> {
    fn new(rules: &'a mut HashSet<String>) -> Self {
        OptimizerRuleSetMut {
            rules,
        }
    }

    pub fn include(&mut self, rule: OptimizerRule) -> &mut Self {
        self.rules.remove(&OptimizerRuleSet::excluded(&rule));
        self.rules.insert(OptimizerRuleSet::included(&rule));
        self
    }

    pub fn exclude(&mut self, rule: OptimizerRule) -> &mut Self {
        self.rules.remove(&OptimizerRuleSet::included(&rule));
        self.rules.insert(OptimizerRuleSet::excluded(&rule));
        self
    }

    pub fn remove(&mut self, rule: &OptimizerRule) -> &mut Self {
        self.rules.remove(&OptimizerRuleSet::included(&rule));
        self.rules.remove(&OptimizerRuleSet::excluded(&rule));
        self
    }
}

#[derive(Debug)]
pub struct OptimizerRuleIntoIter<'a> {
    inner: Iter<'a, String>,
}

impl<'a> Iterator for OptimizerRuleIntoIter<'a> {
    type Item = (IncludedExcluded, OptimizerRule);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| match (&v[..1], &v[1..]) {
            ("+", rule) => (IncludedExcluded::Included, OptimizerRule::from_api_str(rule)),
            ("-", rule) => (IncludedExcluded::Excluded, OptimizerRule::from_api_str(rule)),
            _ => unreachable!(),
        })
    }
}

impl<'a> ExactSizeIterator for OptimizerRuleIntoIter<'a> {}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum IncludedExcluded {
    Included,
    Excluded,
}

/// Represents the rules for the AQL query optimizer.
///
/// This enum defines all possible optimizer rules as listed in the official
/// documentation of *ArangoDB*.
///
/// Source: [https://docs.arangodb.com/devel/AQL/ExecutionAndPerformance/Optimizer.html#list-of-optimizer-rules]
///
/// Last updated: 10/08/2017
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OptimizerRule {
    /// Pseudo-rule that matches all rules.
    All,
    /// will appear if a CalculationNode was moved up in a plan. The intention of this rule is to move calculations up in the processing pipeline as far as possible (ideally out of enumerations) so they are not executed in loops if not required. It is also quite common that this rule enables further optimizations to kick in.
    MoveCalculationsUp,
    /// will appear if a FilterNode was moved up in a plan. The intention of this rule is to move filters up in the processing pipeline as far as possible (ideally out of inner loops) so they filter results as early as possible.
    MoveFiltersUp,
    /// will appear when the values used as right-hand side of an IN operator will be pre-sorted using an extra function call. Pre-sorting the comparison array allows using a binary search in-list lookup with a logarithmic complexity instead of the default linear complexity in-list lookup.
    SortInValues,
    /// will appear if a FilterNode was removed or replaced. FilterNodes whose filter condition will always evaluate to true will be removed from the plan, whereas FilterNode that will never let any results pass will be replaced with a NoResultsNode.
    RemoveUnnecessaryFilters,
    /// will appear if redundant calculations (expressions with the exact same result) were found in the query. The optimizer rule will then replace references to the redundant expressions with a single reference, allowing other optimizer rules to remove the then-unneeded CalculationNodes.
    RemoveRedundantCalculations,
    /// will appear if CalculationNodes were removed from the query. The rule will removed all calculations whose result is not referenced in the query (note that this may be a consequence of applying other optimizations).
    RemoveUnnecessaryCalculations,
    /// will appear if multiple SORT statements can be merged into fewer sorts.
    RemoveRedundantSorts,
    /// will appear if a query contains multiple FOR statements whose order were permuted. Permutation of FOR statements is performed because it may enable further optimizations by other rules.
    InterchangeAdjacentEnumerations,
    /// will appear if an INTO clause was removed from a COLLECT statement because the result of INTO is not used. May also appear if a result of a COLLECT statement's AGGREGATE variables is not used.
    RemoveCollectVariables,
    /// will appear when a constant value was inserted into a filter condition, replacing a dynamic attribute value.
    PropagateConstantAttributes,
    /// will appear if multiple OR-combined equality conditions on the same variable or attribute were replaced with an IN condition.
    ReplaceOrWithIn,
    /// will appear if multiple OR conditions for the same variable or attribute were combined into a single condition.
    RemoveRedundantOr,
    /// will appear when an index is used to iterate over a collection. As a consequence, an EnumerateCollectionNode was replaced with an IndexNode in the plan.
    UseIndexes,
    /// will appear if a FilterNode was removed or replaced because the filter condition is already covered by an IndexNode.
    RemoveFilterCoveredByIndex,
    /// will appear if a FilterNode was removed or replaced because the filter condition is already covered by an TraversalNode.
    RemoveFilterCoveredByTraversal,
    /// will appear if an index can be used to avoid a SORT operation. If the rule was applied, a SortNode was removed from the plan.
    UseIndexForSort,
    /// will appear if a CalculationNode was moved down in a plan. The intention of this rule is to move calculations down in the processing pipeline as far as possible (below FILTER, LIMIT and SUBQUERY nodes) so they are executed as late as possible and not before their results are required.
    MoveCalculationsDown,
    /// will appear if an UpdateNode was patched to not buffer its input completely, but to process it in smaller batches. The rule will fire for an UPDATE query that is fed by a full collection scan, and that does not use any other indexes and sub-queries.
    PatchUpdateStatements,
    /// will appear if either the edge or path output variable in an AQL traversal was optimized away, or if a FILTER condition from the query was moved in the TraversalNode for early pruning of results.
    OptimizeTraversals,
    /// will appear when a sub query was pulled out in its surrounding scope, e.g. FOR x IN (FOR y IN collection FILTER y.value >= 5 RETURN y.test) RETURN x.a would become FOR tmp IN collection FILTER tmp.value >= 5 LET x = tmp.test RETURN x.a
    InlineSubQueries,
    /// will appear when a geo index is utilized.
    GeoIndexOptimizer,
    /// will appear when a SORT RAND() expression is removed by moving the random iteration into an EnumerateCollectionNode. This optimizer rule is specific for the MMFiles storage engine.
    RemoveSortRand,
    /// will appear when an EnumerationCollectionNode that would have extracted an entire document was modified to return only a projection of each document. This optimizer rule is specific for the RocksDB storage engine.
    ReduceExtractionToProjection,
    #[cfg(feature = "cluster")]
    /// will appear when query parts get distributed in a cluster. This is not an optimization rule, and it cannot be turned off.
    DistributeInCluster,
    #[cfg(feature = "cluster")]
    /// will appear when scatter, gather, and remote nodes are inserted into a distributed query. This is not an optimization rule, and it cannot be turned off.
    ScatterInCluster,
    #[cfg(feature = "cluster")]
    /// will appear when filters are moved up in a distributed execution plan. Filters are moved as far up in the plan as possible to make result sets as small as possible as early as possible.
    DistributeFilterCalcToCluster,
    #[cfg(feature = "cluster")]
    /// will appear if sorts are moved up in a distributed query. Sorts are moved as far up in the plan as possible to make result sets as small as possible as early as possible.
    DistributeSortToCluster,
    #[cfg(feature = "cluster")]
    /// will appear if a RemoteNode is followed by a ScatterNode, and the ScatterNode is only followed by calculations or the SingletonNode. In this case, there is no need to distribute the calculation, and it will be handled centrally.
    RemoveUnnecessaryRemoteScatter,
    #[cfg(feature = "cluster")]
    /// will appear if a RemoveNode can be pushed into the same query part that enumerates over the documents of a collection. This saves inter-cluster round-trips between the EnumerateCollectionNode and the RemoveNode.
    UnDistributeRemoveAfterEnumColl,
    /// Can be used to specify a rule that has not been added to this enum yet.
    Custom(String),
}

impl OptimizerRule {
    /// Constructs an optimizer rule from the string slice as used in the
    /// *ArangoDB* API.
    pub fn from_api_str(api_str: &str) -> Self {
        use self::OptimizerRule::*;
        match api_str {
            "all" => All,
            "move-calculations-up" => MoveCalculationsUp,
            "move-filters-up" => MoveFiltersUp,
            "sort-in-values" => SortInValues,
            "remove-unnecessary-filters" => RemoveUnnecessaryFilters,
            "remove-redundant-calculations" => RemoveRedundantCalculations,
            "remove-unnecessary-calculations" => RemoveUnnecessaryCalculations,
            "remove-redundant-sorts" => RemoveRedundantSorts,
            "interchange-adjacent-enumerations" => InterchangeAdjacentEnumerations,
            "remove-collect-variables" => RemoveCollectVariables,
            "propagate-constant-attributes" => PropagateConstantAttributes,
            "replace-or-with-in" => ReplaceOrWithIn,
            "remove-redundant-or" => RemoveRedundantOr,
            "use-indexes" => UseIndexes,
            "remove-filter-covered-by-index" => RemoveFilterCoveredByIndex,
            "remove-filter-covered-by-traversal" => RemoveFilterCoveredByTraversal,
            "use-index-for-sort" => UseIndexForSort,
            "move-calculations-down" => MoveCalculationsDown,
            "patch-update-statements" => PatchUpdateStatements,
            "optimize-traversals" => OptimizeTraversals,
            "inline-subqueries" => InlineSubQueries,
            "geo-index-optimizer" => GeoIndexOptimizer,
            "remove-sort-rand" => RemoveSortRand,
            "reduce-extraction-to-projection" => ReduceExtractionToProjection,
            #[cfg(feature = "cluster")]
            "distribute-in-cluster" => DistributeInCluster,
            #[cfg(feature = "cluster")]
            "scatter-in-cluster" => ScatterInCluster,
            #[cfg(feature = "cluster")]
            "distribute-filtercalc-to-cluster" => DistributeFilterCalcToCluster,
            #[cfg(feature = "cluster")]
            "distribute-sort-to-cluster" => DistributeSortToCluster,
            #[cfg(feature = "cluster")]
            "remove-unnecessary-remote-scatter" => RemoveUnnecessaryRemoteScatter,
            #[cfg(feature = "cluster")]
            "undistribute-remove-after-enum-coll" => UnDistributeRemoveAfterEnumColl,
            rule => Custom(rule.to_owned()),
        }
    }

    /// Returns this optimizer rule as a string slice to be used with the
    /// *ArangoDB* API.
    pub fn as_api_str(&self) -> &str {
        use self::OptimizerRule::*;
        match *self {
            All => "all",
            MoveCalculationsUp => "move-calculations-up",
            MoveFiltersUp => "move-filters-up",
            SortInValues => "sort-in-values",
            RemoveUnnecessaryFilters => "remove-unnecessary-filters",
            RemoveRedundantCalculations => "remove-redundant-calculations",
            RemoveUnnecessaryCalculations => "remove-unnecessary-calculations",
            RemoveRedundantSorts => "remove-redundant-sorts",
            InterchangeAdjacentEnumerations => "interchange-adjacent-enumerations",
            RemoveCollectVariables => "remove-collect-variables",
            PropagateConstantAttributes => "propagate-constant-attributes",
            ReplaceOrWithIn => "replace-or-with-in",
            RemoveRedundantOr => "remove-redundant-or",
            UseIndexes => "use-indexes",
            RemoveFilterCoveredByIndex => "remove-filter-covered-by-index",
            RemoveFilterCoveredByTraversal => "remove-filter-covered-by-traversal",
            UseIndexForSort => "use-index-for-sort",
            MoveCalculationsDown => "move-calculations-down",
            PatchUpdateStatements => "patch-update-statements",
            OptimizeTraversals => "optimize-traversals",
            InlineSubQueries => "inline-subqueries",
            GeoIndexOptimizer => "geo-index-optimizer",
            RemoveSortRand => "remove-sort-rand",
            ReduceExtractionToProjection => "reduce-extraction-to-projection",
            #[cfg(feature = "cluster")]
            DistributeInCluster => "distribute-in-cluster",
            #[cfg(feature = "cluster")]
            ScatterInCluster => "scatter-in-cluster",
            #[cfg(feature = "cluster")]
            DistributeFilterCalcToCluster => "distribute-filtercalc-to-cluster",
            #[cfg(feature = "cluster")]
            DistributeSortToCluster => "distribute-sort-to-cluster",
            #[cfg(feature = "cluster")]
            RemoveUnnecessaryRemoteScatter => "remove-unnecessary-remote-scatter",
            #[cfg(feature = "cluster")]
            UnDistributeRemoveAfterEnumColl => "undistribute-remove-after-enum-coll",
            Custom(ref rule) => rule,
        }
    }
}

/// The execution node types will appear in the execution plan as output of
/// the explain method.
///
/// This enum defines all possible execution nodes as listed in the official
/// documentation of *ArangoDB*.
///
/// Source: [https://docs.arangodb.com/devel/AQL/ExecutionAndPerformance/Optimizer.html#list-of-execution-nodes]
///
/// Last update: 10/08/2017
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExecutionNode {
    /// the purpose of a SingletonNode is to produce an empty document that is used as input for other processing steps. Each execution plan will contain exactly one SingletonNode as its top node.
    SingletonNode,
    /// enumeration over documents of a collection (given in its collection attribute) without using an index.
    EnumerateCollectionNode,
    /// enumeration over one or many indexes (given in its indexes attribute) of a collection. The index ranges are specified in the condition attribute of the node.
    IndexNode,
    /// enumeration over a list of (non-collection) values.
    EnumerateListNode,
    /// only lets values pass that satisfy a filter condition. Will appear once per FILTER statement.
    FilterNode,
    /// limits the number of results passed to other processing steps. Will appear once per LIMIT statement.
    LimitNode,
    /// evaluates an expression. The expression result may be used by other nodes, e.g. FilterNode, EnumerateListNode, SortNode etc.
    CalculationNode,
    /// executes a sub-query.
    SubQueryNode,
    /// performs a sort of its input values.
    SortNode,
    /// aggregates its input and produces new output variables. This will appear once per COLLECT statement.
    AggregateNode,
    /// returns data to the caller. Will appear in each read-only query at least once. Sub-queries will also contain ReturnNodes.
    ReturnNode,
    /// inserts documents into a collection (given in its collection attribute).Will appear exactly once in a query that contains an INSERT statement.
    InsertNode,
    /// removes documents from a collection (given in its collection attribute).Will appear exactly once in a query that contains a REMOVE statement.
    RemoveNode,
    /// replaces documents in a collection (given in its collection attribute).Will appear exactly once in a query that contains a REPLACE statement.
    ReplaceNode,
    /// updates documents in a collection (given in its collection attribute).Will appear exactly once in a query that contains an UPDATE statement.
    UpdateNode,
    /// upserts documents in a collection (given in its collection attribute).Will appear exactly once in a query that contains an UPSERT statement.
    UpsertNode,
    /// will be inserted if FILTER statements turn out to be never satisfiable. The NoResultsNode will pass an empty result set into the processing pipeline.
    NoResultsNode,
    #[cfg(feature = "cluster")]
    /// used on a coordinator to fan-out data to one or multiple shards.
    ScatterNode,
    #[cfg(feature = "cluster")]
    /// used on a coordinator to aggregate results from one or many shards into a combined stream of results.
    GatherNode,
    #[cfg(feature = "cluster")]
    /// used on a coordinator to fan-out data to one or multiple shards, taking into account a collection's shard key.
    DistributeNode,
    #[cfg(feature = "cluster")]
    /// a RemoteNode will perform communication with another ArangoDB instances in the cluster. For example, the cluster coordinator will need to communicate with other servers to fetch the actual data from the shards. It will do so via RemoteNodes. The data servers themselves might again pull further data from the coordinator, and thus might also employ RemoteNodes. So, all of the above cluster relevant nodes will be accompanied by a RemoteNode.
    RemoteNode,
    /// Can be used to specify a execution node that has not been added to this enum yet.
    Unlisted(String),
}

impl ExecutionNode {
    /// Constructs an execution node from the string slice as used in the
    /// *ArangoDB* API.
    pub fn from_api_str(api_str: &str) -> Self {
        use self::ExecutionNode::*;
        match api_str {
            "SingletonNode" => SingletonNode,
            "EnumerateCollectionNode" => EnumerateCollectionNode,
            "IndexNode" => IndexNode,
            "EnumerateListNode" => EnumerateListNode,
            "FilterNode" => FilterNode,
            "LimitNode" => LimitNode,
            "CalculationNode" => CalculationNode,
            "SubqueryNode" => SubQueryNode,
            "SortNode" => SortNode,
            "AggregateNode" => AggregateNode,
            "ReturnNode" => ReturnNode,
            "InsertNode" => InsertNode,
            "RemoveNode" => RemoveNode,
            "ReplaceNode" => ReplaceNode,
            "UpdateNode" => UpdateNode,
            "UpsertNode" => UpsertNode,
            "NoResultsNode" => NoResultsNode,
            #[cfg(feature = "cluster")]
            "ScatterNode" => ScatterNode,
            #[cfg(feature = "cluster")]
            "GatherNode" => GatherNode,
            #[cfg(feature = "cluster")]
            "DistributeNode" => DistributeNode,
            #[cfg(feature = "cluster")]
            "RemoteNode" => RemoteNode,
            node => Unlisted(node.to_owned()),
        }
    }

    /// Returns this execution node as a string slice to be used with the
    /// *ArangoDB* API.
    pub fn as_api_str(&self) -> &str {
        use self::ExecutionNode::*;
        match *self {
            SingletonNode => "SingletonNode",
            EnumerateCollectionNode => "EnumerateCollectionNode",
            IndexNode => "IndexNode",
            EnumerateListNode => "EnumerateListNode",
            FilterNode => "FilterNode",
            LimitNode => "LimitNode",
            CalculationNode => "CalculationNode",
            SubQueryNode => "SubqueryNode",
            SortNode => "SortNode",
            AggregateNode => "AggregateNode",
            ReturnNode => "ReturnNode",
            InsertNode => "InsertNode",
            RemoveNode => "RemoveNode",
            ReplaceNode => "ReplaceNode",
            UpdateNode => "UpdateNode",
            UpsertNode => "UpsertNode",
            NoResultsNode => "NoResultsNode",
            #[cfg(feature = "cluster")]
            ScatterNode => "ScatterNode",
            #[cfg(feature = "cluster")]
            GatherNode => "GatherNode",
            #[cfg(feature = "cluster")]
            DistributeNode => "DistributeNode",
            #[cfg(feature = "cluster")]
            RemoteNode => "RemoteNode",
            Unlisted(ref node) => node,
        }
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
    fn set_optimizer_rule_cursor_option_on_a_newly_initialized_new_cursor() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());
        let query = query;

        let mut new_cursor = NewCursor::from(query);
        assert!(new_cursor.options().is_none());

        new_cursor.options_mut().optimizer_rules_mut()
            .include(OptimizerRule::UseIndexes)
            .exclude(OptimizerRule::MoveFiltersUp);
        let new_cursor = new_cursor;

        assert!(new_cursor.options().is_some());
        assert!(new_cursor.options().unwrap().optimizer_rules().is_some());

        let optimizer_rules = new_cursor.options().unwrap().optimizer_rules().unwrap();
        assert!(optimizer_rules.includes(&OptimizerRule::UseIndexes));
        assert!(optimizer_rules.excludes(&OptimizerRule::MoveFiltersUp));
    }

    #[test]
    fn set_cursor_options_on_newly_initialized_new_cursor() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());
        let query = query;

        let mut new_cursor = NewCursor::from(query);
        assert!(new_cursor.options().is_none());

        new_cursor.options_mut()
            .with_fail_on_warning(true)
            .with_full_count(Some(false))
            .with_max_warning_count(None)
            .with_max_plans(5)
        ;
        #[cfg(feature = "rocksdb")] {
            new_cursor.options_mut()
                .with_intermediate_commit_count(1)
            ;
        }
        #[cfg(feature = "enterprise")] {
            new_cursor.options_mut()
                .with_satellite_sync_wait(false)
            ;
        }
        let new_cursor = new_cursor;
        assert!(new_cursor.options().is_some());
        let cursor_options = new_cursor.options().unwrap();

        assert_eq!(Some(true), cursor_options.is_fail_on_warning());
        assert_eq!(None, cursor_options.is_profile());
        assert_eq!(None, cursor_options.max_warning_count());
        assert_eq!(Some(false), cursor_options.is_full_count());
        assert_eq!(Some(5), cursor_options.max_plans());

        #[cfg(feature = "rocksdb")] {
            assert_eq!(Some(1), cursor_options.intermediate_commit_count());
            assert_eq!(None, cursor_options.intermediate_commit_size());
            assert_eq!(None, cursor_options.max_transaction_size());
        }
        #[cfg(feature = "enterprise")] {
            assert_eq!(Some(false), cursor_options.satellite_sync_wait());
        }
    }

    #[test]
    fn set_options_on_newly_initialized_new_cursor() {
        let mut query = Query::new("FOR u IN users FILTER u.name = @name RETURN u.name");
        query.set_parameter("name".to_owned(), "simone".to_owned());
        let query = query;

        let mut new_cursor = NewCursor::from(query);
        new_cursor.with_batch_size(42)
            .with_cache(false)
            .with_count(None)
            .with_memory_limit(32 * 1024)
            .with_ttl(Some(30))
        ;
        let new_cursor = new_cursor;

        assert_eq!(Some(42), new_cursor.batch_size());
        assert_eq!(Some(false), new_cursor.is_cache());
        assert_eq!(None, new_cursor.is_count());
        assert_eq!(Some(32768), new_cursor.memory_limit());
        assert_eq!(Some(30), new_cursor.ttl());
    }
}
