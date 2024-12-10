use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};

use crate::{ast::{AssignStmt, LetStmt}, codegen::expressions::compile_expr};

use super::ModuleCtx;

/// A let statement
///
/// let x = 2;
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    todo!("implement let")
}

/// An assign statement
///
/// x = 2;
pub fn compile_assign<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &AssignStmt,
) {
    todo!("implement assign")
}
