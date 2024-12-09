use std::fs;

use codegen::{compile_program, CompileCtx};
use lalrpop_util::lalrpop_mod;
use melior::{ir::{Location, Module}, Context};

mod ast;
mod codegen;


lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

fn main() {
    let source = std::env::args().nth(1).unwrap();
    let source = fs::read_to_string(&source).unwrap();
    let program = grammar::ProgramParser::new().parse(&source).unwrap();

    let context = Context::new();
    let ctx = CompileCtx {
        ctx: &context,
        module: Module::new(Location::unknown(&context))
    };

    dbg!(&program);

    compile_program(&program, &ctx);
}
