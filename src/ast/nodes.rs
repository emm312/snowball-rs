use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;
use crate::ast::attrs::AttrHandler;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    New,
    Del,
    Index,
}

#[derive(Debug, Clone)]
pub struct AstType {
    ast: Node,
}

impl AstType {
    pub fn new(ast: Node) -> Self {
        AstType { ast }
    }

    pub fn get_ast(&self) -> &Node {
        &self.ast
    }
}

#[derive(Debug, Clone)]
pub struct GenericDecl {
    name: String,
    default: Option<AstType>,
    impls: Vec<AstType>,
}

impl GenericDecl {
    pub fn new(name: String, impls: Vec<AstType>, default: Option<AstType>) -> Self {
        GenericDecl { name, impls, default }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_impls(&self) -> &Vec<AstType> {
        &self.impls
    }
}

#[derive(Debug, Clone)]
pub struct ClassMember {
    name: String,
    ty: AstType,
}

impl ClassMember {
    pub fn new(name: String, ty: AstType) -> Self {
        ClassMember { name, ty }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_ty(&self) -> &AstType {
        &self.ty
    }
}

#[derive(Debug, Clone)]
pub enum AST<T: std::fmt::Debug + Clone = Node, E: std::fmt::Debug + Clone = ExprNode> {
    TopLevel(Vec<T>),
    Stmt(Stmt<T>),
    Expr(Expr<E>),
}

#[derive(Debug, Clone)]
pub enum Stmt<T: std::fmt::Debug + Clone = Node> {
    Return(Option<T>),
    Break,
    Continue,
    If(T, T, Vec<T>),
    While(T, Vec<T>, /* is_do_while */ bool),
    For(T, T, T, Vec<T>),
    Block(Vec<T>),
    FuncDef(/* name */ String, /* args */ HashMap<String, AstType>, /* ret arg */AstType, Option<T>, Option<Vec<GenericDecl>>),
    VarDef(Option<T>, T),
    ClassDef(Option<T>, Vec<ClassMember>, Vec<GenericDecl>),
    NamespaceDef(Option<T>, Vec<T>),
    Import(T),
    InterfaceDef(Option<T>, Vec<T>, Vec<GenericDecl>),
    EnumDef(Option<T>, Vec<T>),
}

#[derive(Debug, Clone)]
pub enum Expr<T: std::fmt::Debug + Clone = ExprNode> {
    ClassInit(AstType, Vec<T>),
    ClassAccess(T, String),
    NamespaceAccess(T, String),
    Ident(String, Option<Vec<AstType>>),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Call(T, Vec<T>),
    Cast(T, AstType),
    BinaryOp(BinaryOp, T, T, /* is_unary */ bool),
    Assign(T, T),
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: Box<AST>,
    attrs: Option<AttrHandler>,
}

#[derive(Debug, Clone)]
pub struct ExprNode {
    kind: Box<Expr>,
    attrs: Option<AttrHandler>,
}

impl Node {
    pub fn new(kind: AST) -> Self {
        Node { kind: Box::new(kind), attrs: None }
    }

    pub fn get_kind(&self) -> &AST {
        &self.kind
    }

    pub fn with_attrs(&mut self, attrs: AttrHandler) -> &Self {
        self.attrs = Some(attrs);
        self
    }

    pub fn get_attrs(&self) -> Option<&AttrHandler> {
        self.attrs.as_ref()
    }
}
