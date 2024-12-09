use std::collections::HashMap;

use melior::ir::{Block, Value};

use crate::ast::ReturnStmt;

use super::FunctionCtx;

pub fn compile_return<'ctx, 'parent>(
    ctx: &FunctionCtx<'ctx>,
    locals: &HashMap<String, Value>,
    block: &'parent Block<'ctx>,
    stmt: &ReturnStmt,
) {
}
