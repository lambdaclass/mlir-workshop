#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Eq,
    Neq,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStmt),
    If(IfStmt),
    Return(ReturnStmt),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub variable: String,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Block,
    pub r#else: Option<Block>,
}

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}
