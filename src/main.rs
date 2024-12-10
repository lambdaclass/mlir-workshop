use std::{fs, path::PathBuf};

use clap::Parser;
use codegen::compile_program;
use lalrpop_util::lalrpop_mod;

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
    #[arg(long, default_value_t = 3)]
    opt_level: u8,
}

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(&args.input).unwrap();
    let program = grammar::ProgramParser::new().parse(&source).unwrap();

    compile_program(&program, args.opt_level.into(), &args.output);
}

#[cfg(test)]
mod tests {

    use melior::ExecutionEngine;

    use crate::{codegen::compile_program_jit, grammar};

    fn call_program(engine: &ExecutionEngine, a: i64, b: i64) -> i64 {
        unsafe {
            let add: extern "C" fn(i64, i64) -> i64 = std::mem::transmute(engine.lookup("test"));
            add(a, b)
        }
    }

    #[test]
    fn add() {
        let program = grammar::ProgramParser::new()
            .parse(include_str!("../test/add.prog"))
            .unwrap();

        let engine = compile_program_jit(&program);

        let res = call_program(&engine, 2, 3);
        assert_eq!(res, 5);
    }
}
