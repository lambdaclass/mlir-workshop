use std::collections::HashMap;

use melior::{
    dialect::{arith, llvm},
    helpers::{ArithBlockExt, BuiltinBlockExt, LlvmBlockExt},
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
        Expr::Number(value) => compile_number(ctx, block, value),
        Expr::Variable(name) => {
            todo!("implement loading values from the given variable name")
        }
        Expr::Op(lhs_expr, opcode, rhs_expr) => match opcode {
            Opcode::Mul => compile_mul(ctx, block, lhs_expr, rhs_expr, locals),
            Opcode::Div => compile_div(ctx, block, lhs_expr, rhs_expr, locals),
            Opcode::Add => compile_add(ctx, block, lhs_expr, rhs_expr, locals),
            Opcode::Sub => compile_sub(ctx, block, lhs_expr, rhs_expr, locals),
            Opcode::Eq => compile_eq(ctx, block, lhs_expr, rhs_expr, locals),
            Opcode::Neq => compile_neq(ctx, block, lhs_expr, rhs_expr, locals),
        },
        Expr::Call { target, args } => todo!("implement function call"),
    }
}

fn compile_number<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    value: &i64,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let int_type = IntegerType::new(&ctx.ctx, 64).into();
    let int_atributte = IntegerAttribute::new(int_type, *value).into();

    block.append_op_result(arith::constant(&ctx.ctx, int_atributte, location)).unwrap().into()
}

fn compile_mul<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    let res = block.append_operation(arith::muli(lhs, rhs, location));

    res.result(0).unwrap().into()
}

fn compile_div<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    let res = block.append_operation(arith::divsi(lhs, rhs, location));

    res.result(0).unwrap().into()
}

fn compile_add<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    block.append_op_result(arith::addi(lhs, rhs, location)).unwrap().into()
}

fn compile_sub<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    block.append_op_result(arith::subi(lhs, rhs, location)).unwrap().into()
}

fn compile_eq<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    block.append_op_result(arith::cmpi(
        &ctx.ctx,
        arith::CmpiPredicate::Eq,
        lhs,
        rhs,
        location,
    )).unwrap().into()
}

fn compile_neq<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    block: &'parent Block<'ctx>,
    lhs_expr: &Box<Expr>,
    rhs_expr: &Box<Expr>,
    locals: &HashMap<String, Value<'ctx, 'parent>>,
) -> Value<'ctx, 'parent> {
    let location = Location::unknown(ctx.ctx);
    let lhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, lhs_expr);
    let rhs: Value<'ctx, 'parent> = compile_expr(ctx, locals, block, rhs_expr);

    block.append_op_result(arith::cmpi(
        &ctx.ctx,
        arith::CmpiPredicate::Ne,
        lhs,
        rhs,
        location,
    )).unwrap().into()
}
