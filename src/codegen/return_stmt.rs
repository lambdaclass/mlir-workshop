use std::collections::HashMap;

use melior::ir::{Block, BlockRef, Value};

use crate::ast::ReturnStmt;

use super::ModuleCtx;

pub fn compile_return<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &HashMap<String, Value>,
    block: &'parent Block<'ctx>,
    stmt: &ReturnStmt,
) {
}
