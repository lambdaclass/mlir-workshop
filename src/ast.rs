/// The possible expressions, usually on the right hand side of an assignment
/// let x = <expr> ;
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Call {
        target: String,
        args: Vec<Expr>,
    },
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

// A statement, separated by ;
#[derive(Debug, Clone)]
pub enum Statement {
    Let(LetStmt),
    If(IfStmt),
    Return(ReturnStmt),
}

/// The let statement, it binds a value from an expression to the given variable.
#[derive(Debug, Clone)]
pub struct LetStmt {
    pub variable: String,
    pub expr: Expr,
}

/// An if with an optional else statement, depending on whether the condition evaluates to true,
/// take one or another block.
#[derive(Debug, Clone)]
pub struct IfStmt {
    pub cond: Expr,
    pub then: Block,
    pub r#else: Option<Block>,
}

/// The return statement of a function
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub expr: Expr,
}

/// A block is a series of statements, used as the function body and if else blocks.
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

/// Describes a function, with the arguments.
/// Note: in this simple language functions always return a i64.
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Block,
}

/// The whole program, simply a list of functions.
/// The function named "main" will be the entrypoint.
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}
