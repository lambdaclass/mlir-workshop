use std::collections::HashMap;

use melior::{
    ir::{Block, BlockRef, Module, Region, Value},
    Context,
};

use crate::ast::{Expr, Function, IfStmt, LetStmt, Opcode, Program, ReturnStmt, Statement};

pub struct ModuleCtx<'c> {
    pub ctx: &'c Context,
    pub module: Module<'c>,
}

pub fn compile_program(ctx: &ModuleCtx, program: &Program) {
    for func in &program.functions {
        compile_function(ctx, func);
    }
}

pub struct FunctionCtx<'c> {
    pub ctx: &'c Context,
    pub module: Module<'c>,
    pub region: Region<'c>,
}

fn compile_function(ctx: &ModuleCtx, func: &Function) {
    let mut locals: HashMap<String, Value> = HashMap::new();

    let ctx = todo!("implement me");
    let block = todo!("implement me");

    for stmt in &func.body.stmts {
        compile_statement(ctx, &mut locals, block, stmt);
    }
}

fn compile_statement<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &mut HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &Statement,
) {
    match stmt {
        Statement::Let(let_stmt) => compile_let(ctx, locals, block, let_stmt),
        Statement::If(if_stmt) => compile_if(ctx, locals, block, if_stmt),
        Statement::Return(return_stmt) => compile_return(ctx, locals, block, return_stmt),
    }
}

fn compile_let<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &mut HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &LetStmt,
) {
}

fn compile_return<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &ReturnStmt,
) {
}

fn compile_if<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &mut HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &IfStmt,
) {
}

fn compile_expr<'c, 'b>(
    ctx: &FunctionCtx<'c>,
    locals: &HashMap<String, Value>,
    block: &'b Block<'c>,
    expr: &Expr,
) -> Value<'c, 'b> {
    match expr {
        Expr::Number(x) => todo!(),
        Expr::Variable(name) => todo!(),
        Expr::Op(lhs_expr, opcode, rhs_expr) => {
            let lhs = compile_expr(ctx, locals, block, &lhs_expr);
            let rhs = compile_expr(ctx, locals, block, &rhs_expr);

            // Bonus: Add short circuit for bool operations.

            match opcode {
                Opcode::Mul => todo!(),
                Opcode::Div => todo!(),
                Opcode::Add => todo!(),
                Opcode::Sub => todo!(),
                Opcode::Eq => todo!(),
                Opcode::Neq => todo!(),
            }
        }
    }
}
