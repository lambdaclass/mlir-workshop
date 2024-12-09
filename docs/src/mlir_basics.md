# MLIR and melior Basics

To use MLIR with Rust, the following library is used: <https://github.com/mlir-rs/melior>

This page explains a bit how to use it.

## The Context

```rust
 let context = Context::new();
```

The context is a opaque struct that holds all the created attributes, locations and more, it must be passed to nearly all
the melior methods.

## Location

```rust
// A location pointing to a file line col
let loc: Location<'c> = Location::new(&context, filename, line, column);
// An unknown location.
let loc = Location::unknown(&context);
```

All operations and arguments have a location in MLIR, if there is no real location, you can use the `unknown` method.

## Module

The module is a compile unit, it internally holds a single operation with a single region with a single block, more specifically, a module is a `builtin.module` operation.

```rust
let module: Module<'c> = Module::new(Location::unknown(&context));
```

To add an operation to a module, you an do the following:

```rust
// body() returns a BlockRef. since you can only add operations to blocks.
module.body().append_operation(operation)
```


## Operation

An operation is an instruction, it holds regions, which themselves hold blocks. It also has attributes, operands and results.

- Attributes are like configuration parameters for the operation.
- Operands are the inputs, values.
- Results are the result values the operation produces, it can be 1 or more.

### Types

Each dialect can define their own types, for example, the index dialect defines the index type:

```rust
let idx = Type::index(&context);
```

The builtin dialect defines some common types, they can be created with `Type::<name>` or with other structs, such as `IntegerType`:

```rust
let my_f16 = Type::float16(context);
let my_u64: Type<'c> = IntegerType::new(context, 64).into();
```

### Attributes

Most operations accept or require attributes, for example the `func.func` operation requires a `StringAttribute` to define the function name, or a `TypeAttribute` to define the function type.

```rust
let my_type_attr: Attribute<'c> = TypeAttribute::new(IntegerType::new(context, 64).into()).into();
```

In melior there are 4 ways to create a operation, using ods, using a method from the `dialect` melior rust module or using the operation builder.

### ODS

ODS is generated using tablegen and rust macros from melior side.

With ods:

```rust
use melior::dialect::ods;

let my_alloca = block.append_operation(ods::llvm::alloca(context, res, array_size, elem_type, location).into());
// Get the returned ptr
let ptr: Value<'c> = my_alloca.result(0).unwrap().into();
```

### The dialect module

This is a handcrafted API, so it may miss a lot of operations:

```rust
let my_alloca = block.append_operation(melior::dialect::llvm::alloca(context, array_size, ptr_type, location, extra_options));
// Get the returned ptr
let ptr: Value<'c> = my_alloca.result(0).unwrap().into();
```

### The operation builder

```rust
let location = Location::unknown(&context);
let r#type = Type::index(&context);
let block = Block::new(&[(r#type, location)]);
let argument: Value = block.argument(0).unwrap().into();

let operands = vec![argument, argument, argument];
let operation = OperationBuilder::new("foo", Location::unknown(&context))
    .add_operands(&operands)
    .build()
    .unwrap();
```

## Region

## Block
