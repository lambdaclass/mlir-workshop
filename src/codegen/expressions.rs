use std::collections::HashMap;

use melior::{
    dialect::{arith, func},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
    ir::{attribute::FlatSymbolRefAttribute, r#type::IntegerType, Block, Location, Type, Value},
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
    let location = Location::unknown(ctx.ctx);
    let i64_type: Type = IntegerType::new(ctx.ctx, 64).into();
    match expr {
        Expr::Number(value) => block.const_int(ctx.ctx, location, value, 64).unwrap(),
        Expr::Variable(name) => {
            let ptr = *locals.get(name).unwrap();
            block.load(ctx.ctx, location, ptr, i64_type).unwrap()
        }
        Expr::Op(lhs_expr, opcode, rhs_expr) => {
            let lhs = compile_expr(ctx, locals, block, lhs_expr);
            let rhs = compile_expr(ctx, locals, block, rhs_expr);
            match opcode {
                Opcode::Mul => block.muli(lhs, rhs, location).unwrap(),
                Opcode::Div => block.divsi(lhs, rhs, location).unwrap(),
                Opcode::Add => block.addi(lhs, rhs, location).unwrap(),
                Opcode::Sub => block.subi(lhs, rhs, location).unwrap(),
                Opcode::Eq => block
                    .cmpi(ctx.ctx, arith::CmpiPredicate::Eq, lhs, rhs, location)
                    .unwrap(),
                Opcode::Neq => block
                    .cmpi(ctx.ctx, arith::CmpiPredicate::Ne, lhs, rhs, location)
                    .unwrap(),
            }
        }
        Expr::Call { target, args } => {
            let arg_values: Vec<_> = args
                .iter()
                .map(|arg| compile_expr(ctx, locals, block, arg))
                .collect();

            block
                .append_op_result(func::call(
                    ctx.ctx,
                    FlatSymbolRefAttribute::new(ctx.ctx, target),
                    &arg_values,
                    &[i64_type],
                    location,
                ))
                .unwrap()
        }
    }
}
