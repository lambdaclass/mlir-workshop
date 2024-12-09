use std::collections::HashMap;

use melior::ir::{BlockRef, Value};

use crate::ast::LetStmt;

use super::FunctionCtx;

pub fn compile_let<'c>(
    ctx: &FunctionCtx<'c>,
    locals: &mut HashMap<String, Value>,
    block: BlockRef<'c, '_>,
    stmt: &LetStmt,
) {
}
