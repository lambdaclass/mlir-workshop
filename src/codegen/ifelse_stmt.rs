use std::collections::HashMap;

use melior::ir::{Block, BlockRef, Value};

use crate::ast::IfStmt;

use super::FunctionCtx;

pub fn compile_if<'ctx: 'parent, 'parent>(
    ctx: &FunctionCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
}
