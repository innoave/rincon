
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::{IntoIter, Iter};
use std::iter::{FromIterator, ExactSizeIterator, Iterator};
use std::mem;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use rincon_core::api::query::Query;
use rincon_core::api::types::{JsonValue, Value};
use index::types::Index;

const COLLECT_METHOD_SORTED: &str = "sorted";
const COLLECT_METHOD_HASH: &str = "hash";

const EXECUTION_NODE_TYPE_SINGLETON_NODE: &str = "SingletonNode";
const EXECUTION_NODE_TYPE_ENUMERATE_COLLECTION_NODE: &str = "EnumerateCollectionNode";
const EXECUTION_NODE_TYPE_INDEX_NODE: &str = "IndexNode";
const EXECUTION_NODE_TYPE_ENUMERATE_LIST_NODE: &str = "EnumerateListNode";
const EXECUTION_NODE_TYPE_FILTER_NODE: &str = "FilterNode";
const EXECUTION_NODE_TYPE_LIMIT_NODE: &str = "LimitNode";
const EXECUTION_NODE_TYPE_CALCULATION_NODE: &str = "CalculationNode";
const EXECUTION_NODE_TYPE_SUBQUERY_NODE: &str = "SubqueryNode";
const EXECUTION_NODE_TYPE_SORT_NODE: &str = "SortNode";
const EXECUTION_NODE_TYPE_COLLECT_NODE: &str = "CollectNode";
const EXECUTION_NODE_TYPE_RETURN_NODE: &str = "ReturnNode";
const EXECUTION_NODE_TYPE_INSERT_NODE: &str = "InsertNode";
const EXECUTION_NODE_TYPE_REMOVE_NODE: &str = "RemoveNode";
const EXECUTION_NODE_TYPE_REPLACE_NODE: &str = "ReplaceNode";
const EXECUTION_NODE_TYPE_UPDATE_NODE: &str = "UpdateNode";
const EXECUTION_NODE_TYPE_UPSERT_NODE: &str = "UpsertNode";
const EXECUTION_NODE_TYPE_NO_RESULTS_NODE: &str = "NoResultsNode";
#[cfg(feature = "cluster")]
const EXECUTION_NODE_TYPE_SCATTER_NODE: &str = "ScatterNode";
#[cfg(feature = "cluster")]
const EXECUTION_NODE_TYPE_GATHER_NODE: &str = "GatherNode";
#[cfg(feature = "cluster")]
const EXECUTION_NODE_TYPE_DISTRIBUTE_NODE: &str = "DistributeNode";
#[cfg(feature = "cluster")]
const EXECUTION_NODE_TYPE_REMOTE_NODE: &str = "RemoteNode";

const OPTIMIZER_RULE_ALL: &str = "all";
const OPTIMIZER_RULE_MOVE_CALCULATIONS_UP: &str = "move-calculations-up";
const OPTIMIZER_RULE_MOVE_FILTERS_UP: &str = "move-filters-up";
const OPTIMIZER_RULE_SORT_IN_VALUES: &str = "sort-in-values";
const OPTIMIZER_RULE_REMOVE_UNNECESSARY_FILTERS: &str = "remove-unnecessary-filters";
const OPTIMIZER_RULE_REMOVE_REDUNDANT_CALCULATIONS: &str = "remove-redundant-calculations";
const OPTIMIZER_RULE_REMOVE_UNNECESSARY_CALCULATIONS: &str = "remove-unnecessary-calculations";
const OPTIMIZER_RULE_REMOVE_REDUNDANT_SORTS: &str = "remove-redundant-sorts";
const OPTIMIZER_RULE_INTERCHANGE_ADJACENT_ENUMERATIONS: &str = "interchange-adjacent-enumerations";
const OPTIMIZER_RULE_REMOVE_COLLECT_VARIABLES: &str = "remove-collect-variables";
const OPTIMIZER_RULE_PROPAGATE_CONSTANT_ATTRIBUTES: &str = "propagate-constant-attributes";
const OPTIMIZER_RULE_REPLACE_OR_WITH_IN: &str = "replace-or-with-in";
const OPTIMIZER_RULE_REMOVE_REDUNDANT_OR: &str = "remove-redundant-or";
const OPTIMIZER_RULE_USE_INDEXES: &str = "use-indexes";
const OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_INDEX: &str = "remove-filter-covered-by-index";
const OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_TRAVERSAL: &str = "remove-filter-covered-by-traversal";
const OPTIMIZER_RULE_USE_INDEX_FOR_SORT: &str = "use-index-for-sort";
const OPTIMIZER_RULE_MOVE_CALCULATIONS_DOWN: &str = "move-calculations-down";
const OPTIMIZER_RULE_PATCH_UPDATE_STATEMENTS: &str = "patch-update-statements";
const OPTIMIZER_RULE_OPTIMIZE_TRAVERSALS: &str = "optimize-traversals";
const OPTIMIZER_RULE_INLINE_SUBQUERIES: &str = "inline-subqueries";
const OPTIMIZER_RULE_GEO_INDEX_OPTIMIZER: &str = "geo-index-optimizer";
const OPTIMIZER_RULE_REMOVE_SORT_RAND: &str = "remove-sort-rand";
const OPTIMIZER_RULE_REDUCE_EXTRACTION_TO_PROJECTION: &str = "reduce-extraction-to-projection";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_DISTRIBUTE_IN_CLUSTER: &str = "distribute-in-cluster";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_SCATTER_IN_CLUSTER: &str = "scatter-in-cluster";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_DISTRIBUTE_FILTERCALC_TO_CLUSTER: &str = "distribute-filtercalc-to-cluster";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_DISTRIBUTE_SORT_TO_CLUSTER: &str = "distribute-sort-to-cluster";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_REMOVE_UNNECESSARY_REMOTE_SCATTER: &str = "remove-unnecessary-remote-scatter";
#[cfg(feature = "cluster")]
const OPTIMIZER_RULE_UNDISTRIBUTE_REMOVE_AFTER_ENUM_COLL: &str = "undistribute-remove-after-enum-coll";

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedQuery {
    collections: Vec<String>,
    bind_vars: Vec<String>,
    ast: Vec<ParsedAstNode>,
}

impl ParsedQuery {
    pub fn new<C, CI, B, BI, A>(
        collections: C,
        bind_vars: B,
        ast: A,
    ) -> Self
        where
            C: IntoIterator<Item=CI>,
            B: IntoIterator<Item=BI>,
            A: IntoIterator<Item=ParsedAstNode>,
            CI: Into<String>,
            BI: Into<String>,
    {
        ParsedQuery {
            collections: Vec::from_iter(collections.into_iter().map(|i| i.into())),
            bind_vars: Vec::from_iter(bind_vars.into_iter().map(|i| i.into())),
            ast: Vec::from_iter(ast.into_iter()),
        }
    }

    pub fn collections(&self) -> &[String] {
        &self.collections
    }

    pub fn bind_vars(&self) -> &[String] {
        &self.bind_vars
    }

    pub fn ast(&self) -> &[ParsedAstNode] {
        &self.ast
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedAstNode {
    #[serde(rename = "type")]
    kind: String,
    name: Option<String>,
    id: Option<AstNodeId>,
    value: Option<JsonValue>,
    #[serde(default = "Vec::new")]
    sub_nodes: Vec<ParsedAstNode>,
}

impl ParsedAstNode {
    pub fn new<K, N, I, V, S>(
        kind: K,
        name: N,
        id: I,
        value: V,
        sub_nodes: S,
    ) -> Self
        where
            K: Into<String>,
            N: Into<Option<String>>,
            I: Into<Option<AstNodeId>>,
            V: Into<Option<JsonValue>>,
            S: IntoIterator<Item=ParsedAstNode>,
    {
        ParsedAstNode {
            kind: kind.into(),
            name: name.into(),
            id: id.into(),
            value: value.into(),
            sub_nodes: Vec::from_iter(sub_nodes.into_iter()),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn id(&self) -> Option<&AstNodeId> {
        self.id.as_ref()
    }

    pub fn value(&self) -> Option<&JsonValue> {
        self.value.as_ref()
    }

    pub fn sub_nodes(&self) -> &[ParsedAstNode] {
        &self.sub_nodes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub struct AstNodeId(pub i64);

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainedQuery {
    plan: Option<ExecutionPlan>,
    #[serde(default = "Vec::new")]
    plans: Vec<ExecutionPlan>,
    warnings: Vec<String>,
    stats: ExecutionStats,
    cacheable: Option<bool>,
}

impl ExplainedQuery {
    pub fn from_best_plan<Pln, Wrns, Wrn, Cch>(
        plan: Pln,
        warnings: Wrns,
        stats: ExecutionStats,
        cacheable: Cch,
    ) -> Self
        where
            Pln: Into<Option<ExecutionPlan>>,
            Wrns: IntoIterator<Item=Wrn>,
            Wrn: Into<String>,
            Cch: Into<Option<bool>>,
    {
        ExplainedQuery {
            plan: plan.into(),
            plans: Vec::new(),
            warnings: Vec::from_iter(warnings.into_iter().map(|w| w.into())),
            stats,
            cacheable: cacheable.into(),
        }
    }

    pub fn from_multiple_plans<Plns, Wrns, Wrn>(
        plans: Plns,
        warnings: Wrns,
        stats: ExecutionStats,
    ) -> Self
        where
            Plns: IntoIterator<Item=ExecutionPlan>,
            Wrns: IntoIterator<Item=Wrn>,
            Wrn: Into<String>
    {
        ExplainedQuery {
            plan: None,
            plans: Vec::from_iter(plans.into_iter()),
            warnings: Vec::from_iter(warnings.into_iter().map(|w| w.into())),
            stats,
            cacheable: None,
        }
    }

    pub fn plan(&self) -> Option<&ExecutionPlan> {
        self.plan.as_ref()
    }

    pub fn plans(&self) -> &[ExecutionPlan] {
        &self.plans
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn stats(&self) -> &ExecutionStats {
        &self.stats
    }

    pub fn is_cacheable(&self) -> Option<bool> {
        self.cacheable
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPlan {
    nodes: Vec<ExecutionNode>,
    rules: Vec<String>,
    collections: Vec<ExecutionCollection>,
    variables: Vec<ExecutionVariable>,
    estimated_cost: f64,
    estimated_nr_items: u64,
}

impl ExecutionPlan {
    pub fn new<Nds, Rls, Rl, Cols, Vars>(
        nodes: Nds,
        rules: Rls,
        collections: Cols,
        variables: Vars,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Nds: IntoIterator<Item=ExecutionNode>,
            Rls: IntoIterator<Item=Rl>,
            Rl: Into<String>,
            Cols: IntoIterator<Item=ExecutionCollection>,
            Vars: IntoIterator<Item=ExecutionVariable>,
    {
        ExecutionPlan {
            nodes: Vec::from_iter(nodes.into_iter()),
            rules: Vec::from_iter(rules.into_iter().map(|r| r.into())),
            collections: Vec::from_iter(collections.into_iter()),
            variables: Vec::from_iter(variables.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }

    pub fn nodes(&self) -> &[ExecutionNode] {
        &self.nodes
    }

    pub fn rules(&self) -> &[String] {
        &self.rules
    }

    pub fn collections(&self) -> &[ExecutionCollection] {
        &self.collections
    }

    pub fn variables(&self) -> &[ExecutionVariable] {
        &self.variables
    }

    pub fn estimated_cost(&self) -> f64 {
        self.estimated_cost
    }

    pub fn estimated_nr_items(&self) -> u64 {
        self.estimated_nr_items
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
#[derive(Clone, Debug, PartialEq)]
pub enum ExecutionNode {
    /// The purpose of a SingletonNode is to produce an empty document that is
    /// used as input for other processing steps. Each execution plan will
    /// contain exactly one SingletonNode as its top node.
    Singleton(SingletonNode),
    /// Enumeration over documents of a collection (given in its collection
    /// attribute) without using an index.
    EnumerateCollection(EnumerateCollectionNode),
    /// Enumeration over one or many indexes (given in its indexes attribute) of
    /// a collection. The index ranges are specified in the condition attribute
    /// of the node.
    Index(IndexNode),
    /// Enumeration over a list of (non-collection) values.
    EnumerateList(EnumerateListNode),
    /// Only lets values pass that satisfy a filter condition. Will appear once
    /// per FILTER statement.
    Filter(FilterNode),
    /// Limits the number of results passed to other processing steps. Will
    /// appear once per LIMIT statement.
    Limit(LimitNode),
    /// Evaluates an expression. The expression result may be used by other
    /// nodes, e.g. FilterNode, EnumerateListNode, SortNode etc.
    Calculation(CalculationNode),
    /// Executes a sub-query.
    SubQuery(SubQueryNode),
    /// Performs a sort of its input values.
    Sort(SortNode),
    /// Aggregates its input and produces new output variables. This will appear
    /// once per COLLECT statement.
    Aggregate(AggregateNode),
    /// Returns data to the caller. Will appear in each read-only query at least
    /// once. Sub-queries will also contain ReturnNodes.
    Return(ReturnNode),
    /// Inserts documents into a collection (given in its collection attribute).
    /// Will appear exactly once in a query that contains an INSERT statement.
    Insert(InsertNode),
    /// Removes documents from a collection (given in its collection attribute).
    /// Will appear exactly once in a query that contains a REMOVE statement.
    Remove(RemoveNode),
    /// Replaces documents in a collection (given in its collection attribute).
    /// Will appear exactly once in a query that contains a REPLACE statement.
    Replace(ReplaceNode),
    /// Updates documents in a collection (given in its collection attribute).
    /// Will appear exactly once in a query that contains an UPDATE statement.
    Update(UpdateNode),
    /// Upserts documents in a collection (given in its collection attribute).
    /// Will appear exactly once in a query that contains an UPSERT statement.
    Upsert(UpsertNode),
    /// Will be inserted if FILTER statements turn out to be never satisfiable.
    /// The NoResultsNode will pass an empty result set into the processing
    /// pipeline.
    NoResults(NoResultsNode),
    #[cfg(feature = "cluster")]
    /// Used on a coordinator to fan-out data to one or multiple shards.
    Scatter(ScatterNode),
    #[cfg(feature = "cluster")]
    /// Used on a coordinator to aggregate results from one or many shards into
    /// a combined stream of results.
    Gather(GatherNode),
    #[cfg(feature = "cluster")]
    /// Used on a coordinator to fan-out data to one or multiple shards, taking
    /// into account a collection's shard key.
    Distribute(DistributeNode),
    #[cfg(feature = "cluster")]
    /// A RemoteNode will perform communication with another ArangoDB instances
    /// in the cluster. For example, the cluster coordinator will need to
    /// communicate with other servers to fetch the actual data from the shards.
    /// It will do so via RemoteNodes. The data servers themselves might again
    /// pull further data from the coordinator, and thus might also employ
    /// RemoteNodes. So, all of the above cluster relevant nodes will be
    /// accompanied by a RemoteNode.
    Remote(RemoteNode),
    /// A generic node that is used for node types that have not been added to
    /// this enum yet. This should not happen until a new node type is added
    /// to a newer version of *ArangoDB*.
    ///
    /// If your application get this node type returned please file an issue
    /// for this crate. Add the query and if possible the debug output of this
    /// ExecutionNode to that issue.
    Unlisted(GenericExecutionNode),
}

/// The purpose of a SingletonNode is to produce an empty document that is used
/// as input for other processing steps. Each execution plan will contain
/// exactly one SingletonNode as its top node.
#[derive(Clone, Debug, PartialEq)]
pub struct SingletonNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
}

impl SingletonNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        SingletonNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

/// Enumeration over documents of a collection (given in its collection
/// attribute) without using an index.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumerateCollectionNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    out_variable: ExecutionVariable,
    random: bool,
}

impl EnumerateCollectionNode {
    pub fn new<Deps, Db, Col>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Col,
        out_variable: ExecutionVariable,
        random: bool,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Col: Into<String>,
    {
        EnumerateCollectionNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            out_variable,
            random,
        }
    }
}

/// Enumeration over one or many indexes (given in its indexes attribute) of a
/// collection. The index ranges are specified in the condition attribute of the
/// node.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    out_variable: ExecutionVariable,
    indexes: Vec<Index>,
    condition: ExecutionExpression,
    reverse: bool,
}

impl IndexNode {
    pub fn new<Deps, Db, Col, Idxs>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Col,
        out_variable: ExecutionVariable,
        indexes: Idxs,
        condition: ExecutionExpression,
        reverse: bool,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Col: Into<String>,
            Idxs: IntoIterator<Item=Index>,
    {
        IndexNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            out_variable,
            indexes: Vec::from_iter(indexes.into_iter()),
            condition,
            reverse,
        }
    }
}

/// Enumeration over a list of (non-collection) values.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumerateListNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    in_variable: ExecutionVariable,
    out_variable: ExecutionVariable,
}

impl EnumerateListNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        in_variable: ExecutionVariable,
        out_variable: ExecutionVariable,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        EnumerateListNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            in_variable,
            out_variable,
        }
    }
}

/// Only lets values pass that satisfy a filter condition. Will appear once per
/// FILTER statement.
#[derive(Clone, Debug, PartialEq)]
pub struct FilterNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    in_variable: ExecutionVariable,
}

impl FilterNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        in_variable: ExecutionVariable,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        FilterNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            in_variable,
        }
    }
}

/// Limits the number of results passed to other processing steps. Will appear
/// once per LIMIT statement.
#[derive(Clone, Debug, PartialEq)]
pub struct LimitNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    offset: u64,
    limit: u64,
    full_count: bool,
}

impl LimitNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        offset: u64,
        limit: u64,
        full_count: bool,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        LimitNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            offset,
            limit,
            full_count,
        }
    }
}

/// Evaluates an expression. The expression result may be used by other nodes,
/// e.g. FilterNode, EnumerateListNode, SortNode etc.
#[derive(Clone, Debug, PartialEq)]
pub struct CalculationNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    out_variable: ExecutionVariable,
    expression_type: String,
    expression: ExecutionExpression,
    can_throw: bool,
}

impl CalculationNode {
    pub fn new<Deps, Etp>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        out_variable: ExecutionVariable,
        expression_type: Etp,
        expression: ExecutionExpression,
        can_throw: bool,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Etp: Into<String>,
    {
        CalculationNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            out_variable,
            expression_type: expression_type.into(),
            expression,
            can_throw,
        }
    }
}

/// Executes a sub-query.
#[derive(Clone, Debug, PartialEq)]
pub struct SubQueryNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    sub_query: ExplainedSubQuery,
    out_variable: ExecutionVariable,
    is_const: bool,
}

impl SubQueryNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        sub_query: ExplainedSubQuery,
        out_variable: ExecutionVariable,
        is_const: bool,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        SubQueryNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            sub_query,
            out_variable,
            is_const,
        }
    }
}

/// Performs a sort of its input values.
#[derive(Clone, Debug, PartialEq)]
pub struct SortNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
}

impl SortNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        SortNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

/// Aggregates its input and produces new output variables. This will appear
/// once per COLLECT statement.
#[derive(Clone, Debug, PartialEq)]
pub struct AggregateNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    out_variable: Option<ExecutionVariable>,
    groups: Vec<ExecutionGroup>,
    aggregates: Vec<ExecutionAggregate>,
    collect_options: CollectOptions,
}

impl AggregateNode {
    pub fn new<Deps, Out>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        out_variable: Out,
        groups: Vec<ExecutionGroup>,
        aggregates: Vec<ExecutionAggregate>,
        collect_options: CollectOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Out: Into<Option<ExecutionVariable>>
    {
        AggregateNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            out_variable: out_variable.into(),
            groups,
            aggregates,
            collect_options,
        }
    }
}

/// Returns data to the caller. Will appear in each read-only query at least
/// once. Sub-queries will also contain ReturnNodes.
#[derive(Clone, Debug, PartialEq)]
pub struct ReturnNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    in_variable: ExecutionVariable,
}

impl ReturnNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        in_variable: ExecutionVariable,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        ReturnNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            in_variable,
        }
    }
}

/// Inserts documents into a collection (given in its collection attribute).
/// Will appear exactly once in a query that contains an INSERT statement.
#[derive(Clone, Debug, PartialEq)]
pub struct InsertNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    in_variable: ExecutionVariable,
    out_variable_new: Option<ExecutionVariable>,
    modification_flags: ModificationOptions,
}

impl InsertNode {
    pub fn new<Deps, Db, Cll, OvN>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Cll,
        in_variable: ExecutionVariable,
        out_variable_new: OvN,
        modification_flags: ModificationOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Cll: Into<String>,
            OvN: Into<Option<ExecutionVariable>>,
    {
        InsertNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            in_variable,
            out_variable_new: out_variable_new.into(),
            modification_flags,
        }
    }
}

/// Removes documents from a collection (given in its collection attribute).
/// Will appear exactly once in a query that contains a REMOVE statement.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    in_variable: ExecutionVariable,
    out_variable_old: Option<ExecutionVariable>,
    modification_flags: ModificationOptions,
}

impl RemoveNode {
    pub fn new<Deps, Db, Cll, OvO>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Cll,
        in_variable: ExecutionVariable,
        out_variable_old: OvO,
        modification_flags: ModificationOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Cll: Into<String>,
            OvO: Into<Option<ExecutionVariable>>,
    {
        RemoveNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            in_variable,
            out_variable_old: out_variable_old.into(),
            modification_flags,
        }
    }
}

/// Replaces documents in a collection (given in its collection attribute).
/// Will appear exactly once in a query that contains a REPLACE statement.
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    in_doc_variable: ExecutionVariable,
    in_key_variable: Option<ExecutionVariable>,
    out_variable_old: Option<ExecutionVariable>,
    out_variable_new: Option<ExecutionVariable>,
    modification_flags: ModificationOptions,
}

impl ReplaceNode {
    pub fn new<Deps, Db, Cll, IkV, OvO, OvN>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Cll,
        in_doc_variable: ExecutionVariable,
        in_key_variable: IkV,
        out_variable_old: OvO,
        out_variable_new: OvN,
        modification_flags: ModificationOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Cll: Into<String>,
            IkV: Into<Option<ExecutionVariable>>,
            OvO: Into<Option<ExecutionVariable>>,
            OvN: Into<Option<ExecutionVariable>>,
    {
        ReplaceNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            in_doc_variable,
            in_key_variable: in_key_variable.into(),
            out_variable_old: out_variable_old.into(),
            out_variable_new: out_variable_new.into(),
            modification_flags,
        }
    }
}

/// Updates documents in a collection (given in its collection attribute).
/// Will appear exactly once in a query that contains an UPDATE statement.
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    in_doc_variable: ExecutionVariable,
    in_key_variable: Option<ExecutionVariable>,
    out_variable_old: Option<ExecutionVariable>,
    out_variable_new: Option<ExecutionVariable>,
    modification_flags: ModificationOptions,
}

impl UpdateNode {
    pub fn new<Deps, Db, Cll, IkV, OvO, OvN>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Cll,
        in_doc_variable: ExecutionVariable,
        in_key_variable: IkV,
        out_variable_old: OvO,
        out_variable_new: OvN,
        modification_flags: ModificationOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Cll: Into<String>,
            IkV: Into<Option<ExecutionVariable>>,
            OvO: Into<Option<ExecutionVariable>>,
            OvN: Into<Option<ExecutionVariable>>,
    {
        UpdateNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            in_doc_variable,
            in_key_variable: in_key_variable.into(),
            out_variable_old: out_variable_old.into(),
            out_variable_new: out_variable_new.into(),
            modification_flags,
        }
    }
}

/// Upserts documents in a collection (given in its collection attribute).
/// Will appear exactly once in a query that contains an UPSERT statement.
#[derive(Clone, Debug, PartialEq)]
pub struct UpsertNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    database: String,
    collection: String,
    in_doc_variable: ExecutionVariable,
    in_key_variable: Option<ExecutionVariable>,
    insert_variable: ExecutionVariable,
    update_variable: ExecutionVariable,
    is_replace: bool,
    modification_flags: ModificationOptions,
}

impl UpsertNode {
    pub fn new<Deps, Db, Cll, IkV>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
        database: Db,
        collection: Cll,
        in_doc_variable: ExecutionVariable,
        in_key_variable: IkV,
        insert_variable: ExecutionVariable,
        update_variable: ExecutionVariable,
        is_replace: bool,
        modification_flags: ModificationOptions,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
            Db: Into<String>,
            Cll: Into<String>,
            IkV: Into<Option<ExecutionVariable>>,
    {
        UpsertNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
            database: database.into(),
            collection: collection.into(),
            in_doc_variable,
            in_key_variable: in_key_variable.into(),
            insert_variable,
            update_variable,
            is_replace,
            modification_flags,
        }
    }
}

/// Will be inserted if FILTER statements turn out to be never satisfiable. The
/// NoResultsNode will pass an empty result set into the processing pipeline.
#[derive(Clone, Debug, PartialEq)]
pub struct NoResultsNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
}

impl NoResultsNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        NoResultsNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

#[cfg(feature = "cluster")]
//TODO add node specific fields
/// Used on a coordinator to fan-out data to one or multiple shards.
#[derive(Clone, Debug, PartialEq)]
pub struct ScatterNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,

}

#[cfg(feature = "cluster")]
impl ScatterNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        ScatterNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

#[cfg(feature = "cluster")]
//TODO add node specific fields
/// Used on a coordinator to aggregate results from one or many shards into a
/// combined stream of results.
#[derive(Clone, Debug, PartialEq)]
pub struct GatherNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,

}

#[cfg(feature = "cluster")]
impl GatherNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        GatherNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

#[cfg(feature = "cluster")]
//TODO add node specific fields
/// Used on a coordinator to fan-out data to one or multiple shards, taking
/// into account a collection's shard key.
#[derive(Clone, Debug, PartialEq)]
pub struct DistributeNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,

}

#[cfg(feature = "cluster")]
impl DistributeNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        DistributeNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

#[cfg(feature = "cluster")]
//TODO add node specific fields
/// A RemoteNode will perform communication with another ArangoDB instances in
/// the cluster. For example, the cluster coordinator will need to communicate
/// with other servers to fetch the actual data from the shards. It will do so
/// via RemoteNodes. The data servers themselves might again pull further data
/// from the coordinator, and thus might also employ RemoteNodes. So, all of the
/// above cluster relevant nodes will be accompanied by a RemoteNode.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoteNode {
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,

}

#[cfg(feature = "cluster")]
impl RemoteNode {
    pub fn new<Deps>(
        id: ExecutionNodeId,
        dependencies: Deps,
        estimated_cost: f64,
        estimated_nr_items: u64,
    ) -> Self
        where
            Deps: IntoIterator<Item=ExecutionNodeId>,
    {
        RemoteNode {
            id,
            dependencies: Vec::from_iter(dependencies.into_iter()),
            estimated_cost,
            estimated_nr_items,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericExecutionNode {
    #[serde(rename = "type")]
    kind: ExecutionNodeType,
    id: ExecutionNodeId,
    dependencies: Vec<ExecutionNodeId>,
    estimated_cost: f64,
    estimated_nr_items: u64,
    depth: Option<u64>,
    database: Option<String>,
    collection: Option<String>,
    in_variable: Option<ExecutionVariable>,
    out_variable: Option<ExecutionVariable>,
    condition_variable: Option<ExecutionVariable>,
    random: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
    full_count: Option<bool>,
    #[serde(rename = "subquery")]
    sub_query: Option<ExplainedSubQuery>,
    is_const: Option<bool>,
    can_throw: Option<bool>,
    expression_type: Option<String>,
    indexes: Option<Vec<Index>>,
    expression: Option<ExecutionExpression>,
    condition: Option<ExecutionExpression>,
    reverse: Option<bool>,
    groups: Option<Vec<ExecutionGroup>>,
    aggregates: Option<Vec<ExecutionAggregate>>,
    collect_options: Option<CollectOptions>,
    modification_flags: Option<ModificationOptions>,
    in_doc_variable: Option<ExecutionVariable>,
    in_key_variable: Option<ExecutionVariable>,
    out_variable_old: Option<ExecutionVariable>,
    out_variable_new: Option<ExecutionVariable>,
    insert_variable: Option<ExecutionVariable>,
    update_variable: Option<ExecutionVariable>,
    is_replace: Option<bool>,
}

impl GenericExecutionNode {
    pub fn new(
        kind: ExecutionNodeType,
        id: ExecutionNodeId,
        dependencies: Vec<ExecutionNodeId>,
        estimated_cost: f64,
        estimated_nr_items: u64,
        depth: Option<u64>,
        database: Option<String>,
        collection: Option<String>,
        in_variable: Option<ExecutionVariable>,
        out_variable: Option<ExecutionVariable>,
        condition_variable: Option<ExecutionVariable>,
        random: Option<bool>,
        offset: Option<u64>,
        limit: Option<u64>,
        full_count: Option<bool>,
        sub_query: Option<ExplainedSubQuery>,
        is_const: Option<bool>,
        can_throw: Option<bool>,
        expression_type: Option<String>,
        indexes: Option<Vec<Index>>,
        expression: Option<ExecutionExpression>,
        condition: Option<ExecutionExpression>,
        reverse: Option<bool>,
        groups: Option<Vec<ExecutionGroup>>,
        aggregates: Option<Vec<ExecutionAggregate>>,
        collect_options: Option<CollectOptions>,
        modification_flags: Option<ModificationOptions>,
        in_doc_variable: Option<ExecutionVariable>,
        in_key_variable: Option<ExecutionVariable>,
        out_variable_old: Option<ExecutionVariable>,
        out_variable_new: Option<ExecutionVariable>,
        insert_variable: Option<ExecutionVariable>,
        update_variable: Option<ExecutionVariable>,
        is_replace: Option<bool>,
    ) -> Self {
        GenericExecutionNode {
            kind,
            id,
            dependencies,
            estimated_cost,
            estimated_nr_items,
            depth,
            database,
            collection,
            in_variable,
            out_variable,
            condition_variable,
            random,
            offset,
            limit,
            full_count,
            sub_query,
            is_const,
            can_throw,
            expression_type,
            indexes,
            expression,
            condition,
            reverse,
            groups,
            aggregates,
            collect_options,
            modification_flags,
            in_doc_variable,
            in_key_variable,
            out_variable_old,
            out_variable_new,
            insert_variable,
            update_variable,
            is_replace,
        }
    }

    pub fn kind(&self) -> &ExecutionNodeType {
        &self.kind
    }

    pub fn id(&self) -> ExecutionNodeId {
        self.id
    }

    pub fn dependencies(&self) -> &[ExecutionNodeId] {
        &self.dependencies
    }

    pub fn estimated_cost(&self) -> f64 {
        self.estimated_cost
    }

    pub fn estimated_nr_items(&self) -> u64 {
        self.estimated_nr_items
    }

    pub fn depth(&self) -> Option<u64> {
        self.depth
    }

    pub fn database(&self) -> Option<&String> {
        self.database.as_ref()
    }

    pub fn collection(&self) -> Option<&String> {
        self.collection.as_ref()
    }

    pub fn in_variable(&self) -> Option<&ExecutionVariable> {
        self.in_variable.as_ref()
    }

    pub fn out_variable(&self) -> Option<&ExecutionVariable> {
        self.out_variable.as_ref()
    }

    pub fn condition_variable(&self) -> Option<&ExecutionVariable> {
        self.condition_variable.as_ref()
    }

    pub fn is_random(&self) -> Option<bool> {
        self.random
    }

    pub fn offset(&self) -> Option<u64> {
        self.offset
    }

    pub fn limit(&self) -> Option<u64> {
        self.limit
    }

    pub fn is_full_count(&self) -> Option<bool> {
        self.full_count
    }

    pub fn sub_query(&self) -> Option<&ExplainedSubQuery> {
        self.sub_query.as_ref()
    }

    pub fn is_const(&self) -> Option<bool> {
        self.is_const
    }

    pub fn can_throw(&self) -> Option<bool> {
        self.can_throw
    }

    pub fn expression_type(&self) -> Option<&String> {
        self.expression_type.as_ref()
    }

    pub fn indexes(&self) -> Option<&Vec<Index>> {
        self.indexes.as_ref()
    }

    pub fn expression(&self) -> Option<&ExecutionExpression> {
        self.expression.as_ref()
    }

    pub fn condition(&self) -> Option<&ExecutionExpression> {
        self.condition.as_ref()
    }

    pub fn is_reverse(&self) -> Option<bool> {
        self.reverse
    }

    pub fn groups(&self) -> Option<&Vec<ExecutionGroup>> {
        self.groups.as_ref()
    }

    pub fn aggregates(&self) -> Option<&Vec<ExecutionAggregate>> {
        self.aggregates.as_ref()
    }

    pub fn collect_options(&self) -> Option<&CollectOptions> {
        self.collect_options.as_ref()
    }

    pub fn modification_flags(&self) -> Option<&ModificationOptions> {
        self.modification_flags.as_ref()
    }

    pub fn in_doc_variable(&self) -> Option<&ExecutionVariable> {
        self.in_doc_variable.as_ref()
    }

    pub fn in_key_variable(&self) -> Option<&ExecutionVariable> {
        self.in_key_variable.as_ref()
    }

    pub fn out_variable_old(&self) -> Option<&ExecutionVariable> {
        self.out_variable_old.as_ref()
    }

    pub fn out_variable_new(&self) -> Option<&ExecutionVariable> {
        self.out_variable_new.as_ref()
    }

    pub fn insert_variable(&self) -> Option<&ExecutionVariable> {
        self.insert_variable.as_ref()
    }

    pub fn update_variable(&self) -> Option<&ExecutionVariable> {
        self.update_variable.as_ref()
    }

    pub fn is_replace(&self) -> Option<bool> {
        self.is_replace
    }
}

impl<'de> Deserialize<'de> for ExecutionNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        use serde::de::Error;
        use self::ExecutionNode::*;
        let GenericExecutionNode {
            kind,
            id,
            dependencies,
            estimated_cost,
            estimated_nr_items,
            depth,
            database,
            collection,
            in_variable,
            out_variable,
            condition_variable,
            random,
            offset,
            limit,
            full_count,
            sub_query,
            is_const,
            can_throw,
            expression_type,
            indexes,
            expression,
            condition,
            reverse,
            groups,
            aggregates,
            collect_options,
            modification_flags,
            in_doc_variable,
            in_key_variable,
            out_variable_old,
            out_variable_new,
            insert_variable,
            update_variable,
            is_replace,
        } = GenericExecutionNode::deserialize(deserializer)?;
        match kind {
            ExecutionNodeType::SingletonNode =>
                Ok(Singleton(SingletonNode {
                    id,
                    dependencies,
                    estimated_cost,
                    estimated_nr_items,
                })),
            ExecutionNodeType::EnumerateCollectionNode => match (database, collection, out_variable, random) {
                (Some(database), Some(collection), Some(out_variable), Some(random)) =>
                    Ok(EnumerateCollection(EnumerateCollectionNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        out_variable,
                        random,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::IndexNode => match (database, collection, out_variable, indexes, condition, reverse) {
                (Some(database), Some(collection), Some(out_variable), Some(indexes), Some(condition), Some(reverse)) =>
                    Ok(Index(IndexNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        out_variable,
                        indexes,
                        condition,
                        reverse,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::EnumerateListNode => match (in_variable, out_variable) {
                (Some(in_variable), Some(out_variable)) =>
                    Ok(EnumerateList(EnumerateListNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        in_variable,
                        out_variable,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::FilterNode => match in_variable {
                Some(in_variable) =>
                    Ok(Filter(FilterNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        in_variable,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::LimitNode => match (offset, limit, full_count) {
                (Some(offset), Some(limit), Some(full_count)) =>
                    Ok(Limit(LimitNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        offset,
                        limit,
                        full_count,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::CalculationNode => match (out_variable, expression_type, expression, can_throw) {
                (Some(out_variable), Some(expression_type), Some(expression), Some(can_throw)) =>
                    Ok(Calculation(CalculationNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        out_variable,
                        expression_type,
                        expression,
                        can_throw,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::SubQueryNode => match (sub_query, out_variable, is_const) {
                (Some(sub_query), Some(out_variable), Some(is_const)) =>
                    Ok(SubQuery(SubQueryNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        sub_query,
                        out_variable,
                        is_const,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::SortNode =>
                Ok(Sort(SortNode {
                    id,
                    dependencies,
                    estimated_cost,
                    estimated_nr_items,
                })),
            ExecutionNodeType::AggregateNode => match (groups, aggregates, collect_options) {
                (Some(groups), Some(aggregates), Some(collect_options)) =>
                    Ok(Aggregate(AggregateNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        out_variable,
                        groups,
                        aggregates,
                        collect_options,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            }
            ExecutionNodeType::ReturnNode => match in_variable {
                Some(in_variable) =>
                    Ok(Return(ReturnNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        in_variable,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::InsertNode => match (database, collection, in_variable, modification_flags) {
                (Some(database), Some(collection), Some(in_variable), Some(modification_flags)) =>
                    Ok(Insert(InsertNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        in_variable,
                        out_variable_new,
                        modification_flags,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::RemoveNode => match (database, collection, in_variable, modification_flags) {
                (Some(database), Some(collection), Some(in_variable), Some(modification_flags)) =>
                    Ok(Remove(RemoveNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        in_variable,
                        out_variable_old,
                        modification_flags,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::ReplaceNode => match (database, collection, in_doc_variable, modification_flags) {
                (Some(database), Some(collection), Some(in_doc_variable), Some(modification_flags)) =>
                    Ok(Replace(ReplaceNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        in_doc_variable,
                        in_key_variable,
                        out_variable_old,
                        out_variable_new,
                        modification_flags,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::UpdateNode => match (database, collection, in_doc_variable, modification_flags) {
                (Some(database), Some(collection), Some(in_doc_variable), Some(modification_flags)) =>
                    Ok(Update(UpdateNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        in_doc_variable,
                        in_key_variable,
                        out_variable_old,
                        out_variable_new,
                        modification_flags,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::UpsertNode => match (database, collection, in_doc_variable, insert_variable, update_variable, is_replace, modification_flags) {
                (Some(database), Some(collection), Some(in_doc_variable), Some(insert_variable), Some(update_variable), Some(is_replace), Some(modification_flags)) =>
                    Ok(Upsert(UpsertNode {
                        id,
                        dependencies,
                        estimated_cost,
                        estimated_nr_items,
                        database,
                        collection,
                        in_doc_variable,
                        in_key_variable,
                        insert_variable,
                        update_variable,
                        is_replace,
                        modification_flags,
                    })),
                _ => Err(D::Error::custom("Unsupported type/fields combination")),
            },
            ExecutionNodeType::NoResultsNode =>
                Ok(NoResults(NoResultsNode {
                    id,
                    dependencies,
                    estimated_cost,
                    estimated_nr_items,
                })),
            //ExecutionNodeType::Unlisted(_) =>
            _ =>
                Ok(Unlisted(GenericExecutionNode {
                    kind,
                    id,
                    dependencies,
                    estimated_cost,
                    estimated_nr_items,
                    depth,
                    database,
                    collection,
                    in_variable,
                    out_variable,
                    condition_variable,
                    random,
                    offset,
                    limit,
                    full_count,
                    sub_query,
                    is_const,
                    can_throw,
                    expression_type,
                    indexes,
                    expression,
                    condition,
                    reverse,
                    groups,
                    aggregates,
                    collect_options,
                    modification_flags,
                    in_doc_variable,
                    in_key_variable,
                    out_variable_old,
                    out_variable_new,
                    insert_variable,
                    update_variable,
                    is_replace,
                })),
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
pub enum ExecutionNodeType {
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

impl ExecutionNodeType {
    /// Constructs an execution node from the string slice as used in the
    /// *ArangoDB* API.
    pub fn from_api_str(api_str: &str) -> Self {
        use self::ExecutionNodeType::*;
        match api_str {
            EXECUTION_NODE_TYPE_SINGLETON_NODE => SingletonNode,
            EXECUTION_NODE_TYPE_ENUMERATE_COLLECTION_NODE => EnumerateCollectionNode,
            EXECUTION_NODE_TYPE_INDEX_NODE => IndexNode,
            EXECUTION_NODE_TYPE_ENUMERATE_LIST_NODE => EnumerateListNode,
            EXECUTION_NODE_TYPE_FILTER_NODE => FilterNode,
            EXECUTION_NODE_TYPE_LIMIT_NODE => LimitNode,
            EXECUTION_NODE_TYPE_CALCULATION_NODE => CalculationNode,
            EXECUTION_NODE_TYPE_SUBQUERY_NODE => SubQueryNode,
            EXECUTION_NODE_TYPE_SORT_NODE => SortNode,
            EXECUTION_NODE_TYPE_COLLECT_NODE => AggregateNode,
            EXECUTION_NODE_TYPE_RETURN_NODE => ReturnNode,
            EXECUTION_NODE_TYPE_INSERT_NODE => InsertNode,
            EXECUTION_NODE_TYPE_REMOVE_NODE => RemoveNode,
            EXECUTION_NODE_TYPE_REPLACE_NODE => ReplaceNode,
            EXECUTION_NODE_TYPE_UPDATE_NODE => UpdateNode,
            EXECUTION_NODE_TYPE_UPSERT_NODE => UpsertNode,
            EXECUTION_NODE_TYPE_NO_RESULTS_NODE => NoResultsNode,
            #[cfg(feature = "cluster")]
            EXECUTION_NODE_TYPE_SCATTER_NODE => ScatterNode,
            #[cfg(feature = "cluster")]
            EXECUTION_NODE_TYPE_GATHER_NODE => GatherNode,
            #[cfg(feature = "cluster")]
            EXECUTION_NODE_TYPE_DISTRIBUTE_NODE => DistributeNode,
            #[cfg(feature = "cluster")]
            EXECUTION_NODE_TYPE_REMOTE_NODE => RemoteNode,
            node => Unlisted(node.to_owned()),
        }
    }

    /// Returns this execution node as a string slice to be used with the
    /// *ArangoDB* API.
    pub fn as_api_str(&self) -> &str {
        use self::ExecutionNodeType::*;
        match *self {
            SingletonNode => EXECUTION_NODE_TYPE_SINGLETON_NODE,
            EnumerateCollectionNode => EXECUTION_NODE_TYPE_ENUMERATE_COLLECTION_NODE,
            IndexNode => EXECUTION_NODE_TYPE_INDEX_NODE,
            EnumerateListNode => EXECUTION_NODE_TYPE_ENUMERATE_LIST_NODE,
            FilterNode => EXECUTION_NODE_TYPE_FILTER_NODE,
            LimitNode => EXECUTION_NODE_TYPE_LIMIT_NODE,
            CalculationNode => EXECUTION_NODE_TYPE_CALCULATION_NODE,
            SubQueryNode => EXECUTION_NODE_TYPE_SUBQUERY_NODE,
            SortNode => EXECUTION_NODE_TYPE_SORT_NODE,
            AggregateNode => EXECUTION_NODE_TYPE_COLLECT_NODE,
            ReturnNode => EXECUTION_NODE_TYPE_RETURN_NODE,
            InsertNode => EXECUTION_NODE_TYPE_INSERT_NODE,
            RemoveNode => EXECUTION_NODE_TYPE_REMOVE_NODE,
            ReplaceNode => EXECUTION_NODE_TYPE_REPLACE_NODE,
            UpdateNode => EXECUTION_NODE_TYPE_UPDATE_NODE,
            UpsertNode => EXECUTION_NODE_TYPE_UPSERT_NODE,
            NoResultsNode => EXECUTION_NODE_TYPE_NO_RESULTS_NODE,
            #[cfg(feature = "cluster")]
            ScatterNode => EXECUTION_NODE_TYPE_SCATTER_NODE,
            #[cfg(feature = "cluster")]
            GatherNode => EXECUTION_NODE_TYPE_GATHER_NODE,
            #[cfg(feature = "cluster")]
            DistributeNode => EXECUTION_NODE_TYPE_DISTRIBUTE_NODE,
            #[cfg(feature = "cluster")]
            RemoteNode => EXECUTION_NODE_TYPE_REMOTE_NODE,
            Unlisted(ref node) => node,
        }
    }
}

impl<'de> Deserialize<'de> for ExecutionNodeType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = String::deserialize(deserializer)?;
        Ok(ExecutionNodeType::from_api_str(&value))
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainedSubQuery {
    nodes: Vec<ExecutionNode>,
}

impl ExplainedSubQuery {
    pub fn new(
        nodes: Vec<ExecutionNode>,
    ) -> Self {
        ExplainedSubQuery {
            nodes,
        }
    }

    pub fn nodes(&self) -> &[ExecutionNode] {
        &self.nodes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub struct ExecutionNodeId(pub i64);

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionVariable {
    id: ExecutionVariableId,
    name: String,
}

impl ExecutionVariable {
    pub fn new<N>(id: ExecutionVariableId, name: N) -> Self
        where N: Into<String>
    {
        ExecutionVariable {
            id,
            name: name.into(),
        }
    }

    pub fn id(&self) -> ExecutionVariableId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub struct ExecutionVariableId(pub i64);

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionCollection {
    #[serde(rename = "type")]
    kind: String,
    name: String,
}

impl ExecutionCollection {
    pub fn new<K, N>(kind: K, name: N) -> Self
        where K: Into<String>, N: Into<String>
    {
        ExecutionCollection {
            kind: kind.into(),
            name: name.into(),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionExpression {
    #[serde(rename = "type")]
    kind: String,
    name: Option<String>,
    id: Option<ExecutionExpressionId>,
    value: Option<JsonValue>,
    sorted: Option<bool>,
    quantifier: Option<String>,
    #[serde(default = "Vec::new")]
    levels: Vec<u64>,
    #[serde(default = "Vec::new")]
    sub_nodes: Vec<ExecutionExpression>,
}

impl ExecutionExpression {
    pub fn new<Kd, Nm, Id, Val, Srt, Qnt, Levs, Subs>(
        kind: Kd,
        name: Nm,
        id: Id,
        value: Val,
        sorted: Srt,
        quantifier: Qnt,
        levels: Levs,
        sub_nodes: Subs,
    ) -> Self
        where
            Kd: Into<String>,
            Nm: Into<Option<String>>,
            Id: Into<Option<ExecutionExpressionId>>,
            Val: Into<Option<JsonValue>>,
            Srt: Into<Option<bool>>,
            Qnt: Into<Option<String>>,
            Levs: IntoIterator<Item=u64>,
            Subs: IntoIterator<Item=ExecutionExpression>,
    {
        ExecutionExpression {
            kind: kind.into(),
            name: name.into(),
            id: id.into(),
            value: value.into(),
            sorted: sorted.into(),
            quantifier: quantifier.into(),
            levels: Vec::from_iter(levels.into_iter()),
            sub_nodes: Vec::from_iter(sub_nodes.into_iter()),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn id(&self) -> Option<ExecutionExpressionId> {
        self.id
    }

    pub fn value(&self) -> Option<&JsonValue> {
        self.value.as_ref()
    }

    pub fn is_sorted(&self) -> Option<bool> {
        self.sorted
    }

    pub fn quantifier(&self) -> Option<&String> {
        self.quantifier.as_ref()
    }

    pub fn levels(&self) -> &[u64] {
        &self.levels
    }

    pub fn sub_nodes(&self) -> &[ExecutionExpression] {
        &self.sub_nodes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub struct ExecutionExpressionId(pub i64);

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionStats {
    rules_executed: u32,
    rules_skipped: u32,
    plans_created: u32,
}

impl ExecutionStats {
    pub fn new(
        rules_executed: u32,
        rules_skipped: u32,
        plans_created: u32,
    ) -> Self {
        ExecutionStats {
            rules_executed,
            rules_skipped,
            plans_created,
        }
    }

    pub fn rules_executed(&self) -> u32 {
        self.rules_executed
    }

    pub fn rules_skipped(&self) -> u32 {
        self.rules_skipped
    }

    pub fn plans_created(&self) -> u32 {
        self.plans_created
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionGroup {
    in_variable: ExecutionVariable,
    out_variable: ExecutionVariable,
}

impl ExecutionGroup {
    pub fn new(
        in_variable: ExecutionVariable,
        out_variable: ExecutionVariable,
    ) -> Self {
        ExecutionGroup {
            in_variable,
            out_variable,
        }
    }

    pub fn in_variable(&self) -> &ExecutionVariable {
        &self.in_variable
    }

    pub fn out_variable(&self) -> &ExecutionVariable {
        &self.out_variable
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionAggregate {
    #[serde(rename = "type")]
    kind: String,
    in_variable: ExecutionVariable,
    out_variable: ExecutionVariable,
}

impl ExecutionAggregate {
    pub fn new<Knd>(
        kind: Knd,
        in_variable: ExecutionVariable,
        out_variable: ExecutionVariable,
    ) -> Self
        where
            Knd: Into<String>,
    {
        ExecutionAggregate {
            kind: kind.into(),
            in_variable,
            out_variable,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn in_variable(&self) -> &ExecutionVariable {
        &self.in_variable
    }

    pub fn out_variable(&self) -> &ExecutionVariable {
        &self.out_variable
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectOptions {
    method: CollectMethod,
}

impl CollectOptions {
    pub fn new(method: CollectMethod) -> Self {
        CollectOptions {
            method,
        }
    }

    pub fn method(&self) -> &CollectMethod {
        &self.method
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CollectMethod {
    Sorted,
    Hash,
    Custom(String),
}

impl CollectMethod {
    pub fn from_api_str(value: &str) -> Self {
        use self::CollectMethod::*;
        match value {
            COLLECT_METHOD_SORTED => Sorted,
            COLLECT_METHOD_HASH => Hash,
            _ => Custom(value.to_owned()),
        }
    }

    pub fn as_api_str(&self) -> &str {
        use self::CollectMethod::*;
        match *self {
            Sorted => COLLECT_METHOD_SORTED,
            Hash => COLLECT_METHOD_HASH,
            Custom(ref method) => method,
        }
    }
}

impl Serialize for CollectMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.as_api_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CollectMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let value = String::deserialize(deserializer)?;
        Ok(CollectMethod::from_api_str(&value))
    }
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModificationOptions {
    ignore_errors: bool,
    wait_for_sync: bool,
    null_means_remove: bool,
    merge_objects: bool,
    ignore_document_not_found: bool,
    read_complete_input: bool,
    use_is_restore: bool,
    consult_aql_write_filter: bool,
}

impl ModificationOptions {
    pub fn new(
        ignore_errors: bool,
        wait_for_sync: bool,
        null_means_remove: bool,
        merge_objects: bool,
        ignore_document_not_found: bool,
        read_complete_input: bool,
        use_is_restore: bool,
        consult_aql_write_filter: bool,
    ) -> Self {
        ModificationOptions {
            ignore_errors,
            wait_for_sync,
            null_means_remove,
            merge_objects,
            ignore_document_not_found,
            read_complete_input,
            use_is_restore,
            consult_aql_write_filter,
        }
    }

    pub fn is_ignore_errors(&self) -> bool {
        self.ignore_errors
    }

    pub fn is_wait_for_sync(&self) -> bool {
        self.wait_for_sync
    }

    pub fn is_null_means_remove(&self) -> bool {
        self.null_means_remove
    }

    pub fn is_merge_objects(&self) -> bool {
        self.merge_objects
    }

    pub fn is_ignore_document_not_found(&self) -> bool {
        self.ignore_document_not_found
    }

    pub fn is_read_complete_input(&self) -> bool {
        self.read_complete_input
    }

    pub fn is_use_is_restore(&self) -> bool {
        self.use_is_restore
    }

    pub fn is_consult_aql_write_filter(&self) -> bool {
        self.consult_aql_write_filter
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewParseQuery {
    query: String,
}

impl NewParseQuery {
    pub fn new<Q>(query: Q) -> Self
        where Q: Into<String>
    {
        NewParseQuery {
            query: query.into(),
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }
}

impl From<Query> for NewParseQuery {
    fn from(query: Query) -> Self {
        let (query, _) = query.deconstruct();
        NewParseQuery::new(query)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExplainQuery {
    /// Contains the query string to be explained.
    query: String,

    /// key/value pairs representing the bind parameters.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    bind_vars: HashMap<String, Value>,

    /// Optional parameters for tweaking query execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<ExplainOptions>,
}

impl NewExplainQuery {
    pub fn new(query: Query) -> Self {
        let (query, bind_vars) = query.deconstruct();
        NewExplainQuery {
            query,
            bind_vars,
            options: None,
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn bind_vars(&self) -> &HashMap<String, Value> {
        &self.bind_vars
    }

    pub fn options_mut(&mut self) -> &mut ExplainOptions {
        self.options.get_or_insert_with(|| ExplainOptions::new())
    }

    pub fn remove_options(&mut self) -> Option<ExplainOptions> {
        mem::replace(&mut self.options, None)
    }

    pub fn options(&self) -> Option<&ExplainOptions> {
        self.options.as_ref()
    }
}

impl From<Query> for NewExplainQuery {
    fn from(query: Query) -> Self {
        NewExplainQuery::new(query)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainOptions {
    /// A flag indicating whether all plans should be returned.
    ///
    /// If set to true, all possible execution plans will be returned. The
    /// default is false, meaning only the optimal plan will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    all_plans: Option<bool>,

    /// Limits the maximum number of plans that are created by the AQL query
    /// optimizer.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_number_of_plans: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    optimizer: Option<Optimizer>,
}

impl ExplainOptions {
    /// Constructs a new instance of an empty `ExplainOptions` struct.
    ///
    /// All fields are set to `None`.
    fn new() -> Self {
        ExplainOptions {
            all_plans: None,
            max_number_of_plans: None,
            optimizer: None,
        }
    }

    pub fn set_all_plans<A>(&mut self, all_plans: A)
        where A: Into<Option<bool>>
    {
        self.all_plans = all_plans.into();
    }

    pub fn is_all_plans(&self) -> Option<bool> {
        self.all_plans
    }

    pub fn set_max_number_of_plans<M>(&mut self, max_number_of_plans: M)
        where M: Into<Option<u32>>
    {
        self.max_number_of_plans = max_number_of_plans.into();
    }

    pub fn max_plans(&self) -> Option<u32> {
        self.max_number_of_plans
    }

    /// Returns a mutable reference to the optimizer options.
    pub fn optimizer_mut(&mut self) -> &mut Optimizer {
        self.optimizer.get_or_insert_with(|| Optimizer::new())
    }

    /// Removes the optimizer options from this struct and returns
    /// the them.
    pub fn remove_optimizer(&mut self) -> Option<Optimizer> {
        mem::replace(&mut self.optimizer, None)
    }

    /// Returns a reference to the optimizer options.
    pub fn optimizer(&self) -> Option<&Optimizer> {
        self.optimizer.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// A list of to-be-included or to-be-excluded optimizer rules can be put
    /// into this attribute, telling the optimizer to include or exclude
    /// specific rules. To disable a rule, prefix its name with a `-`, to
    /// enable a rule, prefix it with a `+`. There is also a pseudo-rule `all`,
    /// which will match all optimizer rules.
    rules: OptimizerRuleSet,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            rules: OptimizerRuleSet::new(),
        }
    }

    pub fn rules_mut(&mut self) -> &mut OptimizerRuleSet {
        &mut self.rules
    }

    pub fn rules(&self) -> &OptimizerRuleSet {
        &self.rules
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OptimizerRuleSet {
    rules_map: HashMap<OptimizerRule, IncludedExcluded>,
}

impl OptimizerRuleSet {
    fn new() -> Self {
        OptimizerRuleSet {
            rules_map: HashMap::new(),
        }
    }

    pub fn include(&mut self, rule: OptimizerRule) -> &mut Self {
        self.rules_map.insert(rule, IncludedExcluded::Included);
        self
    }

    pub fn exclude(&mut self, rule: OptimizerRule) -> &mut Self {
        self.rules_map.insert(rule, IncludedExcluded::Excluded);
        self
    }

    pub fn remove(&mut self, rule: &OptimizerRule) -> &mut Self {
        self.rules_map.remove(rule);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.rules_map.clear();
        self
    }

    pub fn includes(&self, rule: &OptimizerRule) -> bool {
        self.rules_map.get(rule).map_or(false, |x| x == &IncludedExcluded::Included)
    }

    pub fn excludes(&self, rule: &OptimizerRule) -> bool {
        self.rules_map.get(rule).map_or(false, |x| x == &IncludedExcluded::Excluded)
    }

    pub fn contains(&self, rule: &OptimizerRule) -> bool {
        self.rules_map.contains_key(rule)
    }

    pub fn includes_or_excludes(&self, rule: &OptimizerRule) -> Option<&IncludedExcluded> {
        self.rules_map.get(rule)
    }

    pub fn iter(&self) -> OptimizerRuleIter {
        OptimizerRuleIter {
            inner: self.rules_map.iter(),
        }
    }

    pub fn into_iter(self) -> OptimizerRuleIntoIter {
        OptimizerRuleIntoIter {
            inner: self.rules_map.into_iter(),
        }
    }
}

impl Serialize for OptimizerRuleSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use self::IncludedExcluded::*;
        let mut rules_list: Vec<String> = self.rules_map.iter()
            .map(|(rule, inex)| {
                let prefix = match *inex {
                    Included => String::from("+"),
                    Excluded => String::from("-"),
                };
                prefix + rule.as_api_str()
            })
            .collect();
        rules_list.sort_unstable_by(|a, b|
            if a.starts_with('-') {
                Ordering::Less
            } else if b.starts_with('-') {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        );
        rules_list.serialize(serializer)
    }
}

#[derive(Debug)]
pub struct OptimizerRuleIter<'a> {
    inner: Iter<'a, OptimizerRule, IncludedExcluded>,
}

impl<'a> Iterator for OptimizerRuleIter<'a> {
    type Item = (&'a OptimizerRule, &'a IncludedExcluded);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Debug)]
pub struct OptimizerRuleIntoIter {
    inner: IntoIter<OptimizerRule, IncludedExcluded>,
}

impl Iterator for OptimizerRuleIntoIter {
    type Item = (OptimizerRule, IncludedExcluded);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl ExactSizeIterator for OptimizerRuleIntoIter {}

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
            OPTIMIZER_RULE_ALL => All,
            OPTIMIZER_RULE_MOVE_CALCULATIONS_UP => MoveCalculationsUp,
            OPTIMIZER_RULE_MOVE_FILTERS_UP => MoveFiltersUp,
            OPTIMIZER_RULE_SORT_IN_VALUES => SortInValues,
            OPTIMIZER_RULE_REMOVE_UNNECESSARY_FILTERS => RemoveUnnecessaryFilters,
            OPTIMIZER_RULE_REMOVE_REDUNDANT_CALCULATIONS => RemoveRedundantCalculations,
            OPTIMIZER_RULE_REMOVE_UNNECESSARY_CALCULATIONS => RemoveUnnecessaryCalculations,
            OPTIMIZER_RULE_REMOVE_REDUNDANT_SORTS => RemoveRedundantSorts,
            OPTIMIZER_RULE_INTERCHANGE_ADJACENT_ENUMERATIONS => InterchangeAdjacentEnumerations,
            OPTIMIZER_RULE_REMOVE_COLLECT_VARIABLES => RemoveCollectVariables,
            OPTIMIZER_RULE_PROPAGATE_CONSTANT_ATTRIBUTES => PropagateConstantAttributes,
            OPTIMIZER_RULE_REPLACE_OR_WITH_IN => ReplaceOrWithIn,
            OPTIMIZER_RULE_REMOVE_REDUNDANT_OR => RemoveRedundantOr,
            OPTIMIZER_RULE_USE_INDEXES => UseIndexes,
            OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_INDEX => RemoveFilterCoveredByIndex,
            OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_TRAVERSAL => RemoveFilterCoveredByTraversal,
            OPTIMIZER_RULE_USE_INDEX_FOR_SORT => UseIndexForSort,
            OPTIMIZER_RULE_MOVE_CALCULATIONS_DOWN => MoveCalculationsDown,
            OPTIMIZER_RULE_PATCH_UPDATE_STATEMENTS => PatchUpdateStatements,
            OPTIMIZER_RULE_OPTIMIZE_TRAVERSALS => OptimizeTraversals,
            OPTIMIZER_RULE_INLINE_SUBQUERIES => InlineSubQueries,
            OPTIMIZER_RULE_GEO_INDEX_OPTIMIZER => GeoIndexOptimizer,
            OPTIMIZER_RULE_REMOVE_SORT_RAND => RemoveSortRand,
            OPTIMIZER_RULE_REDUCE_EXTRACTION_TO_PROJECTION => ReduceExtractionToProjection,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_DISTRIBUTE_IN_CLUSTER => DistributeInCluster,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_SCATTER_IN_CLUSTER => ScatterInCluster,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_DISTRIBUTE_FILTERCALC_TO_CLUSTER => DistributeFilterCalcToCluster,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_DISTRIBUTE_SORT_TO_CLUSTER => DistributeSortToCluster,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_REMOVE_UNNECESSARY_REMOTE_SCATTER => RemoveUnnecessaryRemoteScatter,
            #[cfg(feature = "cluster")]
            OPTIMIZER_RULE_UNDISTRIBUTE_REMOVE_AFTER_ENUM_COLL => UnDistributeRemoveAfterEnumColl,
            rule => Custom(rule.to_owned()),
        }
    }

    /// Returns this optimizer rule as a string slice to be used with the
    /// *ArangoDB* API.
    pub fn as_api_str(&self) -> &str {
        use self::OptimizerRule::*;
        match *self {
            All => OPTIMIZER_RULE_ALL,
            MoveCalculationsUp => OPTIMIZER_RULE_MOVE_CALCULATIONS_UP,
            MoveFiltersUp => OPTIMIZER_RULE_MOVE_FILTERS_UP,
            SortInValues => OPTIMIZER_RULE_SORT_IN_VALUES,
            RemoveUnnecessaryFilters => OPTIMIZER_RULE_REMOVE_UNNECESSARY_FILTERS,
            RemoveRedundantCalculations => OPTIMIZER_RULE_REMOVE_REDUNDANT_CALCULATIONS,
            RemoveUnnecessaryCalculations => OPTIMIZER_RULE_REMOVE_UNNECESSARY_CALCULATIONS,
            RemoveRedundantSorts => OPTIMIZER_RULE_REMOVE_REDUNDANT_SORTS,
            InterchangeAdjacentEnumerations => OPTIMIZER_RULE_INTERCHANGE_ADJACENT_ENUMERATIONS,
            RemoveCollectVariables => OPTIMIZER_RULE_REMOVE_COLLECT_VARIABLES,
            PropagateConstantAttributes => OPTIMIZER_RULE_PROPAGATE_CONSTANT_ATTRIBUTES,
            ReplaceOrWithIn => OPTIMIZER_RULE_REPLACE_OR_WITH_IN,
            RemoveRedundantOr => OPTIMIZER_RULE_REMOVE_REDUNDANT_OR,
            UseIndexes => OPTIMIZER_RULE_USE_INDEXES,
            RemoveFilterCoveredByIndex => OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_INDEX,
            RemoveFilterCoveredByTraversal => OPTIMIZER_RULE_REMOVE_FILTER_COVERED_BY_TRAVERSAL,
            UseIndexForSort => OPTIMIZER_RULE_USE_INDEX_FOR_SORT,
            MoveCalculationsDown => OPTIMIZER_RULE_MOVE_CALCULATIONS_DOWN,
            PatchUpdateStatements => OPTIMIZER_RULE_PATCH_UPDATE_STATEMENTS,
            OptimizeTraversals => OPTIMIZER_RULE_OPTIMIZE_TRAVERSALS,
            InlineSubQueries => OPTIMIZER_RULE_INLINE_SUBQUERIES,
            GeoIndexOptimizer => OPTIMIZER_RULE_GEO_INDEX_OPTIMIZER,
            RemoveSortRand => OPTIMIZER_RULE_REMOVE_SORT_RAND,
            ReduceExtractionToProjection => OPTIMIZER_RULE_REDUCE_EXTRACTION_TO_PROJECTION,
            #[cfg(feature = "cluster")]
            DistributeInCluster => OPTIMIZER_RULE_DISTRIBUTE_IN_CLUSTER,
            #[cfg(feature = "cluster")]
            ScatterInCluster => OPTIMIZER_RULE_SCATTER_IN_CLUSTER,
            #[cfg(feature = "cluster")]
            DistributeFilterCalcToCluster => OPTIMIZER_RULE_DISTRIBUTE_FILTERCALC_TO_CLUSTER,
            #[cfg(feature = "cluster")]
            DistributeSortToCluster => OPTIMIZER_RULE_DISTRIBUTE_SORT_TO_CLUSTER,
            #[cfg(feature = "cluster")]
            RemoveUnnecessaryRemoteScatter => OPTIMIZER_RULE_REMOVE_UNNECESSARY_REMOTE_SCATTER,
            #[cfg(feature = "cluster")]
            UnDistributeRemoveAfterEnumColl => OPTIMIZER_RULE_UNDISTRIBUTE_REMOVE_AFTER_ENUM_COLL,
            Custom(ref rule) => rule,
        }
    }
}
