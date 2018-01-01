
#[cfg(test)] mod tests;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use rincon_core::api::types::Value;

pub use ast::*;
use ctx::IdGen;

#[derive(Debug, Clone)]
pub struct Context(Rc<RefCell<Aql>>);

impl Context {
    fn new(aql: Aql, root_node: AstNode) -> Self {
        let mut aql = aql;
        aql.set_ast(root_node);
        Context(Rc::new(RefCell::new(aql)))
    }

    fn create_ast_node<E>(&mut self, expr: E) -> AstNode
        where E: Into<Expr>
    {
        self.0.borrow_mut().create_ast_node(expr)
    }
}

#[derive(Debug)]
pub struct Aql {
    id_gen: IdGen,
    ast: Option<AstNode>,
    bind_vars: HashMap<String, Value>,
    collections: Vec<Identifier>,
}

impl Aql {
    fn new() -> Self {
        Aql {
            id_gen: IdGen::new(),
            ast: None,
            bind_vars: HashMap::new(),
            collections: Vec::new(),
        }
    }

    fn set_ast(&mut self, root_node: AstNode) {
        assert!(self.ast.is_none(), "The AST is already set for this AQL context: {:?}", self);
        self.ast = Some(root_node)
    }

    fn create_ast_node<E>(&mut self, expr: E) -> AstNode
        where E: Into<Expr>
    {
        AstNode::new(expr.into(), self.id_gen.next())
    }

    pub fn for_<I>(variable: I) -> ForClause
        where I: Into<Identifier>
    {
        let mut ctx = Aql::new();
        let mut for_node = ctx.create_ast_node(ForExpr::new());
        let variable_node = ctx.create_ast_node(VariableExpr::new(variable));
        for_node.append(variable_node.clone());
        ForClause {
            ctx: Context::new(ctx, for_node),
            node: variable_node,
        }
    }
}

#[derive(Debug)]
pub struct ForClause {
    ctx: Context,
    node: AstNode,
}

impl ForClause {
    pub fn in_<I>(mut self, collection: I) -> InClause
        where I: Into<Identifier>
    {
        let mut in_node = self.ctx.create_ast_node(InExpr::new());
        let collection_node = self.ctx.create_ast_node(CollectionExpr::new(collection));
        in_node.append(collection_node);
        self.node.append(in_node.clone());
        InClause {
            ctx: self.ctx,
            node: in_node,
        }
    }
}

#[derive(Debug)]
pub struct InClause {
    ctx: Context,
    node: AstNode,
}

impl InClause {
    pub fn return_<E>(mut self, expression: E) -> ReturnClause
        where E: Into<String>
    {
        let mut return_node = self.ctx.create_ast_node(ReturnExpr::new());
        let access_node = self.ctx.create_ast_node(AttributeAccessExpr::new(expression));
        return_node.append(access_node);
        self.node.append(return_node.clone());
        ReturnClause {
            ctx: self.ctx,
            node: return_node,
        }
    }
}

#[derive(Debug)]
pub struct ReturnClause {
    ctx: Context,
    node: AstNode,
}
