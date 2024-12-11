# Workshop: Testing

Now that the functionality is implemented, you can run the tests included in the repo, to do so you can run:

```bash
cargo test
```

The tests are programs under the `test/` directory, they are functions with a well defined name and signature, so we can easily
call them from Rust, using the C call convention. The tests are run using the LLVM JIT engine.
