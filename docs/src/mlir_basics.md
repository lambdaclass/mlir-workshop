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
let my_type_attr: Attribute<'c> =
    TypeAttribute::new(IntegerType::new(context, 64).into()).into();
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
let my_alloca = block.append_operation(
        melior::dialect::llvm::alloca(context, array_size, ptr_type, location, extra_options)
    );
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

A region holds one or multiple blocks, it depends on the operation whether there are 1 or more regions.

Usually multiple regions are used in higher level dialects, like SCF, which has while and for constructs, the CF dialect instead works
with blocks.

A region is more isolated than a block, you can easily use a value from a predecessor block within a given block, but taking a value from another region that is not a parent requires passing it as argument to the operation/block. This makes operations that work with regions like SCF a bit harder to work with in some contexts.

```rust
let region = Region::new();

// Add a block to the region.
let block_ref = region.append_block(Block::new(&[]));

// Here one would implement the function body

// pass the region to a operation.
let func_op = func::func(context, name, r#type, region, attributes, location);
```

## Block

A block holds a sequence of operations, control flow can only happen within the isolated operations but control returns always to the next operation within the block. A block must always have a terminator, that is a operation that has the Terminator Trait, this is usually operations that do branching like `cf.br` or that diverge `llvm.unreachable`

```rust
// To create a block we must pass the arguments it accepts, it is an array of a tuple of (Type, Location)
let block = Block::new(&[
    (Type::float32(&context), Location::unknown(&context))
]);

// Get the first argument to use it in future operations:
let arg1: Value = block.argument(0)?.into();

block.append_operation(my_op_here);

```

## Example function adding 2 arguments

Here you can view how to create a function that accepts 2 arguments:

```rust
use melior::{
    Context,
    dialect::{arith, DialectRegistry, func},
    ir::{*, attribute::{StringAttribute, TypeAttribute}, r#type::FunctionType},
    utility::register_all_dialects,
};

// We need a registry to hold all the dialects
let registry = DialectRegistry::new();
// Register all dialects that come with MLIR.
register_all_dialects(&registry);

// The MLIR context, like the LLVM one.
let context = Context::new();
context.append_dialect_registry(&registry);
context.load_all_available_dialects();

// A location is a debug location like in LLVM, in MLIR all
// operations need a location, even if its "unknown".
let location = Location::unknown(&context);

// A MLIR module is akin to a LLVM module.
let module = Module::new(location);

// A integer-like type with platform dependent bit width. (like size_t or usize)
// This is a type defined in the Builtin dialect.
let index_type = Type::index(&context);

// Append a `func::func` operation to the body (a block) of the module.
// This operation accepts a string attribute, which is the name.
// A type attribute, which contains a function type in this case.
// Then it accepts a single region, which is where the body
// of the function will be, this region can have
// multiple blocks, which is how you may implement
// control flow within the function.
// These blocks each can have more operations.
module.body().append_operation(func::func(
    &context,
    // accepts a StringAttribute which is the function name.
    StringAttribute::new(&context, "add"),
    // A type attribute, defining the function signature.
    TypeAttribute::new(
            FunctionType::new(&context, &[index_type, index_type], &[index_type]).into()
        ),
    {
        // The first block within the region, blocks accept arguments
        // In regions with control flow, MLIR leverages
        // this structure to implicitly represent
        // the passage of control-flow dependent values without the complex nuances
        // of PHI nodes in traditional SSA representations.
        let block = Block::new(&[(index_type, location), (index_type, location)]);

        // Use the arith dialect to add the 2 arguments.
        let sum = block.append_operation(arith::addi(
            block.argument(0).unwrap().into(),
            block.argument(1).unwrap().into(),
            location
        ));

        // Return the result using the "func" dialect return operation.
        block.append_operation(
            func::r#return( &[sum.result(0).unwrap().into()], location)
        );

        // The Func operation requires a region,
        // we add the block we created to the region and return it,
        // which is passed as an argument to the `func::func` function.
        let region = Region::new();
        region.append_block(block);
        region
    },
    &[],
    location,
));

assert!(module.as_operation().verify());
```
