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
    let location = Location::unknown(ctx.ctx);
    let ReturnStmt { expr } = stmt;

    let expr_res = compile_expr(ctx, locals, block, expr);

    block.append_operation(func::r#return(&[expr_res], location));
}
