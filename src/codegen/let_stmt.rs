use std::collections::HashMap;

use melior::ir::{Block, BlockRef, Value};

use crate::ast::LetStmt;

use super::FunctionCtx;

pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &FunctionCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
}
