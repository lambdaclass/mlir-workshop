use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};

use crate::ast::{AssignStmt, LetStmt};

use super::{expressions::compile_expr, ModuleCtx};

/// A let statement
///
/// let x = 2;
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    let location = Location::unknown(ctx.ctx);
    let LetStmt { variable, expr } = stmt;
    let int_ty = IntegerType::new(ctx.ctx, 64).into();
    let expr_res = compile_expr(ctx, locals, block, &expr);
    let ptr = block.alloca1(ctx.ctx, location, int_ty, 8).unwrap();

    block.store(ctx.ctx, location, ptr, expr_res).unwrap();
    locals.insert(variable.clone(), ptr);
}

/// An assign statement
///
/// x = 2;
pub fn compile_assign<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &AssignStmt,
) {
    let location = Location::unknown(ctx.ctx);
    let AssignStmt { variable, expr } = stmt;

    if !locals.contains_key(variable) {
        panic!("variable not declared")
    }

    let expr_res = compile_expr(ctx, locals, block, &expr);

    let ptr = locals.get(variable).unwrap();

    block.store(ctx.ctx, location, *ptr, expr_res).unwrap();
}
