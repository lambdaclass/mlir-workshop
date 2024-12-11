# Workshop: Compiling If/Else

To get simple control flow working, you will use the [SCF](https://mlir.llvm.org/docs/Dialects/SCFDialect/) dialect.
With this dialect you don't need to add extra blocks, since the control flow will be contained within the regions inside the SCF operations.

The only limitation is that we can't do early returns this way, but for this simple language it won't matter.

> You will need to clone the `locals` HashMap inside the created regions to avoid lifetime issues. But since any variable created
inside the if or else block only live for that scope, it works good.

```rust
// src/codegen/ifelse_stmt.rs
pub fn compile_if<'ctx, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &IfStmt,
) {
    todo!()
}
```
