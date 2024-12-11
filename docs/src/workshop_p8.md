# Workshop: Compiling Functions

Now to wrap up the function itself needs to be created, using the `func` dialect and adding it to the `module` `body()` block. (The module is available under the ctx variable.)

You also need to allocate space for the arguments, and store the value there. You can get the value from the block arguments.

Remember that in this language functions always return a i64 value.

Some useful types you will need: `Type`, `IntegerAttribute`, `IntegerType`, `FunctionType`, `TypeAttribute`, `StringAttribute`.

```rust
// src/codegen.rs:60+
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
```
