use std::collections::HashMap;

use melior::ir::{Block, Value};

use crate::ast::IfStmt;

use super::ModuleCtx;

pub fn compile_if<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
    todo!()
}
