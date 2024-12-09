use std::collections::HashMap;

use melior::{
    dialect::{arith, llvm},
    ir::{
        attribute::IntegerAttribute, r#type::IntegerType, Block, BlockRef, Location, Type, Value,
    },
};

use crate::ast::{Expr, Opcode};

use super::ModuleCtx;

pub fn compile_expr<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    expr: &Expr,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let i64_type: Type = IntegerType::new(ctx.ctx, 64).into();
    match expr {
        Expr::Number(value) => {
            let value = block
                .append_operation(arith::constant(
                    ctx.ctx,
                    IntegerAttribute::new(IntegerType::new(ctx.ctx, 64).into(), *value).into(),
                    location,
                ))
                .result(0)
                .unwrap()
                .into();

            value
        }
        Expr::Variable(name) => {
            let local_ptr = *locals.get(name).unwrap();
            let value = block
                .append_operation(llvm::load(
                    ctx.ctx,
                    local_ptr,
                    i64_type,
                    location,
                    Default::default(),
                ))
                .result(0)
                .unwrap()
                .into();

            value
        }
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
