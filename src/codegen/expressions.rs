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
    match expr {
        Expr::Number(_value) => {
            todo!()
        }
        Expr::Variable(name) => {
            todo!()
        }
        Expr::Op(lhs_expr, opcode, rhs_expr) => match opcode {
            Opcode::Mul => todo!(),
            Opcode::Div => todo!(),
            Opcode::Add => todo!(),
            Opcode::Sub => todo!(),
            Opcode::Eq => todo!(),
            Opcode::Neq => todo!(),
        },
    }
}
