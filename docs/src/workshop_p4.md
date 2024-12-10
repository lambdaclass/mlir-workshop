# Workshop: Compiling Let and Assign

## Let statement

On let statements, variables are declared, as explained before. In this case we need to allocate space for it and save the
pointer value on the `locals` hashmap.

You will need to use the `llvm` dialect.

```rust
// src/codegen/let_stmt.rs
// let x = 2;
pub fn compile_let<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &LetStmt,
) {
    todo!()
}
```

## Assign statement

Assign is like let, but without creating the variable, only storing the updated value.

```rust
// src/codegen/let_stmt.rs
// x = 2;
pub fn compile_assign<'ctx: 'parent, 'parent>(
    ctx: &ModuleCtx<'ctx>,
    locals: &mut HashMap<String, Value<'ctx, 'parent>>,
    block: &'parent Block<'ctx>,
    stmt: &AssignStmt,
) {
    todo!("implement assign")
}
```
