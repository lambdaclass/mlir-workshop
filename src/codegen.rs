use std::{collections::HashMap, path::Path};

use ifelse_stmt::compile_if;
use let_stmt::{compile_assign, compile_let};
use melior::{
    dialect::{
        func::{self, func},
        DialectRegistry,
    },
    helpers::LlvmBlockExt,
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::{FunctionType, IntegerType},
        Block, Location, Module, Region, Type, Value,
    },
    pass::{self, PassManager},
    utility::register_all_dialects,
    Context, ExecutionEngine,
};
use return_stmt::compile_return;

use crate::{
    ast::{Function, Program, Statement},
    util::{link_binary, llvm_compile, OptLevel},
};

pub mod expressions;
pub mod ifelse_stmt;
pub mod let_stmt;
pub mod return_stmt;

pub struct ModuleCtx<'c> {
    pub ctx: &'c Context,
    pub module: &'c Module<'c>,
}

pub fn compile_program(program: &Program, optlevel: OptLevel, out_name: &Path) {
    // We need a registry to hold all the dialects
    let registry = DialectRegistry::new();
    // Register all dialects that come with MLIR.
    register_all_dialects(&registry);
    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let mut module = Module::new(Location::unknown(&context));
    let ctx = ModuleCtx {
        ctx: &context,
        module: &module,
    };

    for func in &program.functions {
        compile_function(&ctx, func);
    }

    // Run passes on module to convert all dialects to LLVM.
    let pass_manager = PassManager::new(&context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.add_pass(pass::conversion::create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
    pass_manager.add_pass(pass::conversion::create_to_llvm());
    pass_manager.run(&mut module).unwrap();

    let mlir_code = module.as_operation().to_string();
    std::fs::write(out_name.with_extension("mlir"), mlir_code).unwrap();

    // Convert the MLIR to LLVM IR (requires unsafe since we use mlir-sys and llvm-sys for this)
    let object = unsafe { llvm_compile(&module, optlevel) };
    let out_obj = out_name.with_extension("o");
    std::fs::write(&out_obj, &object).unwrap();
    link_binary(&[out_obj], out_name).unwrap();
}

#[cfg(test)]
pub fn compile_program_jit(program: &Program) -> ExecutionEngine {
    // We need a registry to hold all the dialects
    let registry = DialectRegistry::new();
    // Register all dialects that come with MLIR.
    register_all_dialects(&registry);
    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    let mut module = Module::new(Location::unknown(&context));
    let ctx = ModuleCtx {
        ctx: &context,
        module: &module,
    };

    for func in &program.functions {
        compile_function(&ctx, func);
    }

    // Run passes on module to convert all dialects to LLVM.
    let pass_manager = PassManager::new(&context);
    pass_manager.enable_verifier(true);
    pass_manager.add_pass(pass::transform::create_canonicalizer());
    pass_manager.add_pass(pass::conversion::create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
    pass_manager.add_pass(pass::conversion::create_to_llvm());
    pass_manager.run(&mut module).unwrap();

    println!("{}", module.as_operation());

    ExecutionEngine::new(&module, 3, &[], false)
}

fn compile_function(ctx: &ModuleCtx<'_>, func: &Function) {
    let mut args: Vec<(Type, Location)> = vec![];
    let mut func_args: Vec<Type> = Vec::new();

    for _ in &func.args {
        args.push((
            IntegerType::new(ctx.ctx, 64).into(),
            Location::unknown(ctx.ctx),
        ));
        func_args.push(IntegerType::new(ctx.ctx, 64).into());
    }

    let region = Region::new();
    let block = region.append_block(Block::new(&args));
    let mut locals: HashMap<String, Value> = HashMap::new();

    // Allocate space for the arguments, get them from the block, storing them and save them on locals hashmap.

    for stmt in &func.body.stmts {
        compile_statement(ctx, &mut locals, &block, stmt);
    }

    // Create the func operation here.
}

fn compile_statement<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &Statement,
) {
    match stmt {
        Statement::Let(let_stmt) => {
            compile_let(ctx, locals, block, let_stmt);
        }
        Statement::If(if_stmt) => {
            compile_if(ctx, locals, block, if_stmt);
        }
        Statement::Return(return_stmt) => {
            compile_return(ctx, locals, block, return_stmt);
        }
        Statement::Assign(assign_stmt) => {
            compile_assign(ctx, locals, block, assign_stmt);
        }
    }
}
