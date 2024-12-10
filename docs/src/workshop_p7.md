# Workshop: Compiling Function calls


```rust
// src/codegen/expressions.rs
Expr::Call { target, args } => todo!("implement function call"),
```

Since all arguments are of the same type, and for simplicity
sake we don't verify the number of arguments matches the function this should be relatively simple using the `func` dialect.
