use std::{collections::HashMap, ops::Deref};

use melior::{
    dialect::scf,
    ir::{Block, Location, Region, Value},
};

use crate::ast::WhileStmt;

use super::{compile_statement, expressions::compile_expr, ModuleCtx};

pub fn compile_while<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &WhileStmt,
) {
    block.append_operation(scf::r#while(
        &[],
        &[],
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));

            let cond_value = compile_expr(ctx, locals, &block, &stmt.cond);

            block.append_operation(scf::condition(cond_value, &[], Location::unknown(&ctx.ctx)));

            region
        },
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));

            let mut locals = locals.clone();

            for stmt in &stmt.then.stmts {
                compile_statement(ctx, &mut locals, block.deref(), stmt);
            }

            block.append_operation(scf::r#yield(&[], Location::unknown(&ctx.ctx)));

            region
        },
        Location::unknown(&ctx.ctx),
    ));
}
