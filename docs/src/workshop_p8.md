# Workshop: Compiling Functions

Now to wrap up the function itself needs to be created, using the `func` dialect and adding it to the `module` `body()` block. (The module is available under the ctx variable.)

```rust
// src/codegen.rs:60+
fn compile_function(ctx: &ModuleCtx<'_>, func: &Function) {
    let mut args: Vec<(Type, Location)> = vec![];

    for _ in &func.args {
        args.push((
            IntegerType::new(ctx.ctx, 64).into(),
            Location::unknown(ctx.ctx),
        ));
    }

    let region = Region::new();
    let block = region.append_block(Block::new(&args));
    let mut locals: HashMap<String, Value> = HashMap::new();

    for stmt in &func.body.stmts {
        compile_statement(ctx, &mut locals, &block, stmt);
    }

    // Create the func operation here.
}
```
