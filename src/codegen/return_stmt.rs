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
    todo!("implement return")
}
