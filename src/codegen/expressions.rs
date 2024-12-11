use std::collections::HashMap;

use melior::{
    dialect::{arith, func},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
    ir::{attribute::FlatSymbolRefAttribute, r#type::IntegerType, Block, Location, Value},
};

use crate::ast::{Expr, Opcode};

use super::ModuleCtx;

// A right hand side expression: `2 + x * 3`
pub fn compile_expr<'ctx: 'parent, 'parent>(
    // Helper struct with the MLIR Context and Module
    ctx: &ModuleCtx<'ctx>,
    // Hashmap storing the local variables
    locals: &HashMap<String, Value<'ctx, 'parent>>,
    // The current block to work on.
    block: &'parent Block<'ctx>,
    // The expression to compile.
    expr: &Expr,
) -> Value<'ctx, 'parent> {
    match expr {
        Expr::Number(value) => block
            .const_int(&ctx.ctx, Location::unknown(&ctx.ctx), value, 64)
            .unwrap(),
        Expr::Variable(name) => block
            .load(
                &ctx.ctx,
                Location::unknown(&ctx.ctx),
                *locals.get(name).unwrap(),
                IntegerType::new(&ctx.ctx, 64).into(),
            )
            .unwrap(),
        Expr::Op(lhs_expr, opcode, rhs_expr) => {
            let lhs_value = compile_expr(ctx, locals, block, &lhs_expr);
            let rhs_value = compile_expr(ctx, locals, block, &rhs_expr);
            let location = Location::unknown(&ctx.ctx);
            match opcode {
                Opcode::Mul => block.muli(lhs_value, rhs_value, location).unwrap(),
                Opcode::Div => block.divsi(lhs_value, rhs_value, location).unwrap(),
                Opcode::Add => block.addi(lhs_value, rhs_value, location).unwrap(),
                Opcode::Sub => block.subi(lhs_value, rhs_value, location).unwrap(),
                Opcode::Eq => block
                    .cmpi(
                        &ctx.ctx,
                        arith::CmpiPredicate::Eq,
                        lhs_value,
                        rhs_value,
                        location,
                    )
                    .unwrap(),
                Opcode::Neq => block
                    .cmpi(
                        &ctx.ctx,
                        arith::CmpiPredicate::Ne,
                        lhs_value,
                        rhs_value,
                        location,
                    )
                    .unwrap(),
            }
        }
        Expr::Call { target, args } => {
            let args = args
                .iter()
                .map(|expr| compile_expr(ctx, locals, block, expr))
                .collect::<Vec<_>>();

            block
                .append_op_result(func::call(
                    &ctx.ctx,
                    FlatSymbolRefAttribute::new(&ctx.ctx, &target),
                    &args,
                    &[IntegerType::new(&ctx.ctx, 64).into()],
                    Location::unknown(&ctx.ctx),
                ))
                .unwrap()
        }
    }
}
