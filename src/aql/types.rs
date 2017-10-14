
use api::types::JsonValue;
use index::Index;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedQuery {
    collections: Vec<String>,
    bind_vars: Vec<String>,
    ast: Vec<ParsedAstNode>,
}

impl ParsedQuery {
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedAstNode {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    id: Option<i64>,
    value: Option<JsonValue>,
    sub_nodes: Vec<ParsedAstNode>,
}

impl ParsedAstNode {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn value(&self) -> Option<&JsonValue> {
        self.value.as_ref()
    }

    pub fn sub_nodes(&self) -> &[ParsedAstNode] {
        &self.sub_nodes
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainedQuery {
    plan: ExecutionPlan,
    plans: Vec<ExecutionPlan>,
    warnings: Vec<String>,
    stats: ExecutionStats,
    cacheable: bool,
}

impl ExplainedQuery {
    pub fn plan(&self) -> &ExecutionPlan {
        &self.plan
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

    pub fn cacheable(&self) -> bool {
        self.cacheable
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPlan {
    nodes: Vec<ExecutionNode>,
    rules: Vec<String>,
    collections: Vec<ExecutionCollection>,
    variables: Vec<ExecutionVariable>,
    estimated_cost: u32,
    estimated_nr_items: u32,
}

impl ExecutionPlan {
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

    pub fn estimated_cost(&self) -> u32 {
        self.estimated_cost
    }

    pub fn estimated_nr_items(&self) -> u32 {
        self.estimated_nr_items
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionNode {
    #[serde(rename = "type")]
    kind: String,
    id: i64,
    dependencies: Vec<i64>,
    estimated_cost: u32,
    estimated_nr_items: u32,
    depth: u64,
    database: String,
    collection: String,
    in_variable: ExecutionVariable,
    out_variable: ExecutionVariable,
    condition_variable: ExecutionVariable,
    random: bool,
    offset: u64,
    limit: u64,
    full_count: bool,
    #[serde(rename = "subquery")]
    sub_query: Box<ExecutionNode>,
    is_const: bool,
    can_throw: bool,
    expression_type: String,
    indexes: Index,
    expression: ExecutionExpression,
    condition: ExecutionCollection,
    reverse: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionVariable {
    id: i64,
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionCollection {
    #[serde(rename = "type")]
    kind: String,
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionExpression {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    id: i64,
    value: JsonValue,
    sorted: bool,
    quantifier: String,
    levels: Vec<u64>,
    sub_nodes: Vec<ExecutionExpression>,
}

#[allow(missing_copy_implementations)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionStats {
    rules_executed: u32,
    rules_skipped: u32,
    plans_created: u32,
}
