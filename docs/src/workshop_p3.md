# Workshop: Compiling Expressions

To compile expressions, the following is needed:

- Create a constant number.
- From a variable identifier, get it's value.
- Apply a binary operation to 2 other expressions.


```rust
// src/codegen/expressions.rs
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
        Expr::Number(_value) => {
            todo!("implement constant numbers")
        }
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
    }
}
```


## Constants in MLIR

There are various ways to create a constant, in our case, we have 2 dialects available to use:
- The [llvm](https://mlir.llvm.org/docs/Dialects/LLVM/) dialect.
- The [arith](https://mlir.llvm.org/docs/Dialects/ArithOps/) dialect.

> You can find documentation about all dialects and their operations here: <https://mlir.llvm.org/docs/Dialects/>

It is recommended to use the `arith` dialect in this case.

Some useful types you will need: `Type`, `IntegerAttribute`, `IntegerType`, `Location`.

## Loading a variable value

To make things simpler, all variables are stored inside an `llvm.alloca`, which is an operation
that given a size gives a pointer to it. Thus, depending on the use a load/store operation is needed. This avoids dealing with Block arguments but makes the compiler rely on LLVM to optimize these `allocas` (which it does really well).

For this case you can use the [llvm](https://mlir.llvm.org/docs/Dialects/LLVM/) dialect to **load** from the pointer. The variable pointer value can be found in the given hashmap `locals`.

## Binary operations

> To iterate is human, to recurse, divine

Here you will need to use the `arith` dialect to compute the binary operations from computing the `lhs` and `rhs` expressions.
