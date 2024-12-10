use std::collections::HashMap;

use melior::{
    helpers::LlvmBlockExt,
    ir::{r#type::IntegerType, Block, Location, Value},
};

use crate::{ast::LetStmt, codegen::expressions::compile_expr};

use super::ModuleCtx;

pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    todo!("implement let")
}
