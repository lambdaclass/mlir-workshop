use std::collections::HashMap;

use melior::{
    dialect::{arith, llvm},
    helpers::{ArithBlockExt, LlvmBlockExt},
    ir::{
        attribute::IntegerAttribute, r#type::IntegerType, Block, BlockRef, Location, Type, Value,
    },
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
        Expr::Variable(name) => {
            todo!("implement loading values from the given variable name")
        }
        Expr::Op(lhs_expr, opcode, rhs_expr) => match opcode {
            Opcode::Mul => todo!("implement mul"),
            Opcode::Div => todo!("implement div"),
            Opcode::Add => todo!("implement add"),
            Opcode::Sub => todo!("implement sub"),
            Opcode::Eq => todo!("implement eq"),
            Opcode::Neq => todo!("implement neq"),
        },
        Expr::Call { target, args } => todo!("implement function call"),
    }
}
