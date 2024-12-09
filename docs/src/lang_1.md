# Implementing the language: 1

## Project Setup

### Dependencies
- Rust
- LLVM and MLIR

To install LLVM and MLIR you can do so through brew:

`brew install llvm@19` (This workshop uses LLVM/MLIR 19)

```bash
brew install llvm@19
git clone https://github.com/lambdaclass/mlir-workshop
cd mlir-workshop
```

For melior to find the library, we need to setup some env vars (tip, you can add them to `.zshenv`):
```bash
export MLIR_SYS_190_PREFIX="$(brew --prefix llvm@19)"
export LLVM_SYS_191_PREFIX="$(brew --prefix llvm@19)"
export TABLEGEN_190_PREFIX="$(brew --prefix llvm@19)"
```

TODO
