use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Type, Value},
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
    let location = Location::unknown(ctx.ctx);
    let i64_type: Type = IntegerType::new(ctx.ctx, 64).into();
    let ptr = block.alloca1(ctx.ctx, location, i64_type, 8).unwrap();
    let value = compile_expr(ctx, locals, block, &stmt.expr);
    block.store(ctx.ctx, location, ptr, value).unwrap();
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
    let location = Location::unknown(ctx.ctx);
    let ptr = *locals.get(&stmt.variable).unwrap();
    let value = compile_expr(ctx, locals, block, &stmt.expr);
    block.store(ctx.ctx, location, ptr, value).unwrap();
}
