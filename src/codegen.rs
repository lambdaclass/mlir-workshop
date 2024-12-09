use std::{collections::HashMap, path::Path};

use ifelse_stmt::compile_if;
use let_stmt::compile_let;
use melior::{
    ir::{Block, BlockRef, Module, Region, Value},
    Context,
};
use return_stmt::compile_return;

use crate::{
    ast::{Expr, Function, IfStmt, LetStmt, Opcode, Program, ReturnStmt, Statement},
    util::{link_binary, llvm_compile, OptLevel},
};

pub mod expressions;
pub mod ifelse_stmt;
pub mod let_stmt;
pub mod return_stmt;

pub struct ModuleCtx<'c> {
    pub ctx: &'c Context,
    pub module: Module<'c>,
}

pub fn compile_program(ctx: &ModuleCtx, program: &Program, optlevel: OptLevel, out_name: &Path) {
    for func in &program.functions {
        compile_function(ctx, func);
    }

    // Run passes on module to convert all dialects to LLVM.

    // Convert the MLIR to LLVM IR (requires unsafe since we use mlir-sys and llvm-sys for this)
    let object = unsafe { llvm_compile(&ctx.module, optlevel) };
    let out_obj = out_name.with_extension("o");
    std::fs::write(&out_obj, &object).unwrap();
    link_binary(&[out_obj], out_name).unwrap();
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
