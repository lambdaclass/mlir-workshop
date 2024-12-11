use std::collections::HashMap;

use melior::{
    dialect::scf,
    ir::{Block, Location, Region, Value},
};

use crate::ast::IfStmt;

use super::{compile_statement, expressions::compile_expr, ModuleCtx};

pub fn compile_if<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
    let location = Location::unknown(ctx.ctx);
    let cond = compile_expr(ctx, locals, block, &stmt.cond);

    block.append_operation(scf::r#if(
        cond,
        &[],
        {
            let region = Region::new();
            let block = region.append_block(Block::new(&[]));

            let mut locals = locals.clone();

            for stmt in &stmt.then.stmts {
                compile_statement(ctx, &mut locals, &block, stmt);
            }

            block.append_operation(scf::r#yield(&[], location));

            region
        },
        {
            let region = Region::new();

            if let Some(else_stmts) = &stmt.r#else {
                let block = region.append_block(Block::new(&[]));

                let mut locals = locals.clone();

                for stmt in &else_stmts.stmts {
                    compile_statement(ctx, &mut locals, &block, stmt);
                }

                block.append_operation(scf::r#yield(&[], location));
            }

            region
        },
        location,
    ));
}
