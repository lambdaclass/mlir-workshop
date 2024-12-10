# Workshop: Compiling Let

On let statements, variables are declared, as explained before, in this case we need to allocate space for it and save the
pointer value on the `locals` hashmap.

You will need to use the `llvm` dialect.

```rust
// src/codegen/let_stmt.rs
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    todo!()
}
```
