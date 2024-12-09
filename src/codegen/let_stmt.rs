use std::collections::HashMap;

use melior::ir::{Block, Value};

use crate::ast::LetStmt;

use super::ModuleCtx;

pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    todo!()
}
