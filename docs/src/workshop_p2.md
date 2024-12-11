# Workshop: Walkthrough the prepared codebase

```
.
├── ast.rs // The language Abstract syntax tree.
├── codegen
│   ├── expressions.rs
│   ├── ifelse_stmt.rs
│   ├── let_stmt.rs
│   └── return_stmt.rs
├── codegen.rs // Glue code for the codegen methods.
├── grammar.lalrpop // LALRPOP grammar for parsing
├── main.rs // CLI and MLIR Context creation
└── util.rs // Code to translate MLIR to LLVM and link the binary
```

The workshop project already contains the code to handle the following:

- Lexer and parser
- CLI
- The language AST
- Translating to LLVM bytecode and linking the binary.

Thus what's missing is implementing the methods that "compile" the code, a.k.a emit the MLIR operations.
They are located under the `codegen/` folder.

## The AST

The language AST is quite simple, it consists of the following:

```rust
/// The possible expressions, usually on the right hand side of an assignment
/// let x = <expr> ;
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Call { target: String, args: Vec<Expr> },
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

```
