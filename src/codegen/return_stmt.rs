use std::collections::HashMap;

use melior::ir::{BlockRef, Value};

use crate::ast::ReturnStmt;

use super::FunctionCtx;

pub fn compile_return<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &ReturnStmt,
) {
}
