use std::{collections::HashMap, ops::Deref};

use melior::{
    dialect::scf,
    ir::{Block, BlockRef, Region, Value},
};

use crate::ast::IfStmt;

use super::{expressions::compile_expr, ModuleCtx};

pub fn compile_if<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
    todo!()
}
