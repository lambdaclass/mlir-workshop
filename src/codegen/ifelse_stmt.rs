use std::collections::HashMap;

use melior::ir::{BlockRef, Value};

use crate::ast::IfStmt;

use super::FunctionCtx;

pub fn compile_if<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &mut HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &IfStmt,
) {
}
