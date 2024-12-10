# Workshop: Compiling Return

The return statement evaluates the expression and returns the computed value.

You will need to check the [`func`](https://mlir.llvm.org/docs/Dialects/Func/) dialect, although it is possible to do with the `llvm` dialect too.

```rust
pub fn compile_return<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &HashMap<String, Value>,
    block: &'parent Block<'ctx>,
    stmt: &ReturnStmt,
) {
    todo!()
}
```
