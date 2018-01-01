
use std::rc::Rc;
use std::cell::RefCell;

macro_rules! into_expr {
    ($ty:ident -> $variant:ident) => {
        impl From<$ty> for Expr {
            fn from(value: $ty) -> Self {
                Expr::$variant(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AstNode(Rc<RefCell<Node>>);

impl AstNode {
    pub fn new(expr: Expr, id: NodeId) -> Self {
        AstNode(Rc::new(RefCell::new(Node::new(expr, id))))
    }

    pub fn append(&mut self, node: AstNode) {
        self.0.borrow_mut().append(node)
    }
}

pub type NodeId = u64;

#[derive(Debug, PartialEq)]
pub struct Node {
    id: NodeId,
    expression: Expr,
    sub_nodes: Vec<AstNode>,
}

impl Node {
    pub fn new(expression: Expr, id: NodeId) -> Self {
        Node {
            id,
            expression,
            sub_nodes: Vec::new(),
        }
    }

    pub fn append(&mut self, node: AstNode) {
        self.sub_nodes.push(node);
    }
}
//
//#[derive(Debug, Copy, Clone, PartialEq)]
//pub enum NodeType {
//    AttributeAccess,
//    CollectionExpr,
//    Filter,
//    For,
//    Group,
//    In,
//    Literal,
//    Return,
//    Variable,
//}

#[derive(Debug, PartialEq)]
pub enum Expr {
    AttributeAccess(AttributeAccessExpr),
    Collection(CollectionExpr),
    Filter(FilterExpr),
    For(ForExpr),
    Group(GroupExpr),
    In(InExpr),
    Literal(LiteralExpr),
    Return(ReturnExpr),
    Variable(VariableExpr),
}

#[derive(Debug, PartialEq)]
pub struct AttributeAccessExpr {
    attribute: String,
}

impl AttributeAccessExpr {
    pub fn new<A>(attribute: A) -> Self
        where A: Into<String>
    {
        AttributeAccessExpr {
            attribute: attribute.into(),
        }
    }
}

into_expr!(AttributeAccessExpr -> AttributeAccess);

#[derive(Debug, PartialEq)]
pub struct CollectionExpr {
    collection: Identifier,
}

impl CollectionExpr {
    pub fn new<I>(collection: I) -> Self
        where I: Into<Identifier>
    {
        CollectionExpr {
            collection: collection.into(),
        }
    }
}

into_expr!(CollectionExpr -> Collection);

#[derive(Debug, PartialEq)]
pub struct FilterExpr {
}

impl FilterExpr {
    pub fn new() -> Self {
        FilterExpr {
        }
    }
}

into_expr!(FilterExpr -> Filter);

#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq)]
pub struct ForExpr {
}

impl ForExpr {
    pub fn new() -> Self {
        ForExpr {
        }
    }
}

into_expr!(ForExpr -> For);

#[derive(Debug, PartialEq)]
pub struct GroupExpr {
    expression: Vec<Expr>,
}

into_expr!(GroupExpr -> Group);

#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq)]
pub struct InExpr {
}

impl InExpr {
    pub fn new() -> Self {
        InExpr {
        }
    }
}

into_expr!(InExpr -> In);

#[derive(Debug, PartialEq)]
pub struct LiteralExpr {
    value: Value,
}

into_expr!(LiteralExpr -> Literal);

#[derive(Debug, PartialEq)]
pub struct ReturnExpr {
}

impl ReturnExpr {
    pub fn new() -> Self {
        ReturnExpr {
        }
    }
}

into_expr!(ReturnExpr -> Return);

#[derive(Debug, PartialEq)]
pub struct VariableExpr {
    name: Identifier,
}

impl VariableExpr {
    pub fn new<I>(name: I) -> Self
        where I: Into<Identifier>
    {
        VariableExpr {
            name: name.into(),
        }
    }
}

into_expr!(VariableExpr -> Variable);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

impl Identifier {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(value: &'a str) -> Self {
        Identifier(value.to_owned())
    }
}

impl<'a> From<&'a String> for Identifier {
    fn from(value: &'a String) -> Self {
        Identifier(value.to_owned())
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}
