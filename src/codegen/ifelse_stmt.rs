use std::collections::HashMap;

use melior::{
    dialect::scf,
    ir::{r#type::IntegerType, Block, Location, Region, Value},
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
    let IfStmt { cond, then, r#else } = stmt;

    let cond_res = compile_expr(ctx, locals, block, cond);

    block.append_operation(scf::r#if(
        cond_res,
        &[],
        {
            let then_region = Region::new();
            let then_block = then_region.append_block(Block::new(&[]));

            let mut locals = locals.clone();

            for stmt in &then.stmts {
                compile_statement(ctx, &mut locals, &then_block, stmt);
            }

            then_block.append_operation(scf::r#yield(&[], location));

            then_region
        },
        {
            let else_region = Region::new();
            let else_block = else_region.append_block(Block::new(&[]));

            let mut locals = locals.clone();

            if let Some(else_stmts) = r#else {
                for stmt in &else_stmts.stmts {
                    compile_statement(ctx, &mut locals, &else_block, stmt);
                }
            }

            else_block.append_operation(scf::r#yield(&[], location));

            else_region
        },
        location,
    ));
}
