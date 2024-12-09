use std::{fs, path::PathBuf};

use clap::Parser;
use codegen::{compile_program, ModuleCtx};
use lalrpop_util::lalrpop_mod;
use melior::{
    ir::{Location, Module},
    Context,
};

mod ast;
mod codegen;
mod util;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    grammar
);

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Program file
    input: PathBuf,

    #[arg(short, long, default_value = "out.a")]
    output: PathBuf,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 3)]
    optlevel: u8,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(&args.input).unwrap();
    let program = grammar::ProgramParser::new().parse(&source).unwrap();

    let context = Context::new();
    let ctx = ModuleCtx {
        ctx: &context,
        module: Module::new(Location::unknown(&context)),
    };

    compile_program(&ctx, &program, args.optlevel.into(), &args.output);
}
