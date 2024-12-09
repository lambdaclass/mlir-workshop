use std::collections::HashMap;

use melior::ir::{Block, BlockRef, Value};

use crate::ast::{Expr, Opcode};

use super::FunctionCtx;

pub fn compile_expr<'ctx: 'parent, 'parent>(
    ctx: &FunctionCtx<'ctx>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    expr: &Expr,
) -> Value<'ctx, 'parent> {
    match expr {
        Expr::Number(x) => todo!(),
        Expr::Variable(name) => todo!(),
        Expr::Op(lhs_expr, opcode, rhs_expr) => {
            let lhs = compile_expr(ctx, locals, block, lhs_expr);
            let rhs = compile_expr(ctx, locals, block, rhs_expr);

            match opcode {
                Opcode::Mul => todo!(),
                Opcode::Div => todo!(),
                Opcode::Add => todo!(),
                Opcode::Sub => todo!(),
                Opcode::Eq => todo!(),
                Opcode::Neq => todo!(),
            }
        }
    }
}
