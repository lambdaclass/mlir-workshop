use std::collections::HashMap;

use melior::{
    dialect::func,
    ir::{Block, BlockRef, Location, Value},
};

use crate::ast::ReturnStmt;

use super::{expressions::compile_expr, ModuleCtx};

pub fn compile_return<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &HashMap<String, Value>,
    block: &'parent Block<'ctx>,
    stmt: &ReturnStmt,
) {
    let value = compile_expr(ctx, locals, block, &stmt.expr);
    block.append_operation(func::r#return(&[value], Location::unknown(&ctx.ctx)));
}
