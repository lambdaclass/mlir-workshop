# Workshop: Glue code

Here the glue code is explained, mostly how the lowering and compilation works.

## Initial steps

First the MLIR context and the registry of dialects needs to be initialized, then we add and load the dialects into the context.

```rust
// src/codegen.rs
pub fn compile_program(program: &Program, optlevel: OptLevel, out_name: &Path) {
 // We need a registry to hold all the dialects
    let registry = DialectRegistry::new();
    // Register all dialects that come with MLIR.
    register_all_dialects(&registry);
    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    // ...
}
```

Next, initialize the Module and for ease, put both the context and module in a struct. Afterwards we compile all the functions.

```rust
let mut module = Module::new(Location::unknown(&context));
    let ctx = ModuleCtx {
        ctx: &context,
        module: &module,
    };

    for func in &program.functions {
        compile_function(&ctx, func);
    }
```

Now, the module contains operations from the various used dialects, we need to convert them all to the LLVM dialect to
compile it with LLVM, to do so the PassManager is needed, adding the necessary passes to transform and convert the dialects.

```rust
// Run passes on module to convert all dialects to LLVM.
let pass_manager = PassManager::new(&context);
pass_manager.enable_verifier(true);
pass_manager.add_pass(pass::transform::create_canonicalizer());
pass_manager.add_pass(pass::conversion::create_scf_to_control_flow()); // needed because to_llvm doesn't include it.
pass_manager.add_pass(pass::conversion::create_to_llvm());
pass_manager.run(&mut module).unwrap();
```
