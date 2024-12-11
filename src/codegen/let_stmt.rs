use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};

use crate::{
    ast::{AssignStmt, LetStmt},
    codegen::expressions::compile_expr,
};

use super::ModuleCtx;

/// A let statement
///
/// let x = 2;
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    let ptr = block
        .alloca1(
            &ctx.ctx,
            Location::unknown(&ctx.ctx),
            IntegerType::new(&ctx.ctx, 64).into(),
            0,
        )
        .unwrap();

    let value = compile_expr(ctx, locals, block, &stmt.expr);

    block
        .store(&ctx.ctx, Location::unknown(&ctx.ctx), ptr, value)
        .unwrap();

    locals.insert(stmt.variable.clone(), ptr);
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
    let ptr = locals.get(&stmt.variable).unwrap();

    let value = compile_expr(ctx, locals, block, &stmt.expr);

    block
        .store(&ctx.ctx, Location::unknown(&ctx.ctx), *ptr, value)
        .unwrap();
}
