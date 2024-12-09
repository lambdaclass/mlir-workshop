# About this Workshop

In this workshop you will learn a bit about what is LLVM and MLIR, by implementing a really simple programming language compiler. This language only has variables of type u64, and only supports simple functions with arguments, basic arithmetic operations and if else statements.

# What is LLVM?

To know about MLIR, you need to know what is LLVM.

LLVM is what one would call a compiler backend, made of many resuable components, it deals with the intrinsicacies of each CPU architecture, providing a higher level API for programmers to work with, kind of like a "game engine" but for making compiled (or JIT) programming languages, so you don't have to deal with the lowest level details (lower than what some people call low-level anyway).

To abstract away all the specific cpu details, LLVM is a "target", with high level pseudo-assembly, what we call the LLVM IR. We can consider LLVM an abstract machine (much like C), it has infinite registers and some other details, to program this machine we use LLVM IR (IR means Intermediate Representation).

An example of this IR (snippet from [A Gentle Introduction to LLVM IR](https://mcyoung.xyz/2023/08/01/llvm-ir/)):

```llvmir
define i32 @pow(i32 %x, i32 %y) {
  ; Create slots for r and the index, and initialize them.
  ; This is equivalent to something like
  ;   int i = 0, r = 1;
  ; in C.
  %r = alloca i32
  %i = alloca i32
  store i32 1, ptr %r
  store i32 0, ptr %i
  br label %loop_start

loop_start:
  ; Load the index and check if it equals y.
  %i.check = load i32, ptr %i
  %done = icmp eq i32 %i.check, %y
  br i1 %done, label %exit, label %loop

loop:
  ; r *= x
  %r.old = load i32, ptr %r
  %r.new = mul i32 %r.old, %x
  store i32 %r.new, ptr %r

  ; i += 1
  %i.old = load i32, ptr %i
  %i.new = add i32 %i.old, 1
  store i32 %i.new, ptr %i

  br label %loop_start

exit:
  %r.ret = load i32, ptr %r
  ret i32 %r.ret
}
```

To model the specific attributes of the CPU we want to target, we define a [data layout](https://llvm.org/docs/LangRef.html#langref-datalayout) and specify a target triple, such as `x86_64-apple-macosx10.7.0`

The datalayout specifies how data is laid out in memory, such as the alignment and size of different data types, whether its big or little endian, etc.

The LLVM IR uses the following structure:

A module is a compilation unit, and within a module there are functions, globals and more.

A function is made up of blocks, each block contains a sequence of instructions, these instructions are always run sequentially and there is no control flow within a single block, to implement control flow you must jump to other blocks.

It uses Single Static Assignment form (SSA form), which means a variable is assigned once and only once, this allows LLVM to do a lot of optimizations.

Due to this, when control flow is involved, one must take care to dominate all the uses, for example you may have a target block
that has 2 predecessors (2 blocks where each ends up jumping to this target block), each of these predecessors can define variables that will be used in this target block, to "unify" those variables in the target block you must use the [phi](https://llvm.org/docs/LangRef.html#i-phi) instruction, which defines a PHI node.

One can avoid using PHI nodes by relying on `allocas`, a alloca is a reservation of the stack space, basically you give it a size and align and it gives you a pointer to this allocation, you can then simply load/store that pointer, from any branch and you don't have to deal with PHI nodes this way, this is what most languages do, like Rust, they rely on LLVM to later optimize the allocas into register uses whenever possible.

# What is MLIR?

MLIR is kind of a IR of IRs, and it supports many of them using "dialects". For example, you may have heard of NVVM IR (CUDA), MLIR supports modeling it through the NVVM dialect (or ROCDL for AMD), but there is also a more generic and higher level GPU dialect.

Within MLIR, there is a dialect to model the LLVM IR itself, and also conversions and transformations from other dialects into the LLVM IR dialect.

With this, one can create a "dialect" that is high level and can be converted into a GPU kernel or a CPU program for example, this is kind of what the [TOSA](https://mlir.llvm.org/docs/Dialects/TOSA/) dialect does.

As the main page says "MLIR aims to address software fragmentation", and by defining multiple dialects and conversions between them, it's how it achieves that.

Some notable dialects:

- Builtin: The builtin dialect contains a core set of Attributes, Operations, and Types that have wide applicability across a very large number of domains and abstractions. Many of the components of this dialect are also instrumental in the implementation of the core IR.
- Affine: This dialect provides a powerful abstraction for affine operations and analyses.
- Async: Types and operations for async dialect This dialect contains operations for modeling asynchronous execution.
- SCF: The scf (structured control flow) dialect contains operations that represent control flow constructs such as if and for. Being structured means that the control flow has a structure unlike, for example, gotos or asserts.
- CF: This dialect contains low-level, i.e. non-region based, control flow constructs. These constructs generally represent control flow directly on SSA blocks of a control flow graph.
- LLVM: This dialect maps LLVM IR into MLIR by defining the corresponding operations and types. LLVM IR metadata is usually represented as MLIR attributes, which offer additional structure verification.
- GPU: This dialect provides middle-level abstractions for launching GPU kernels following a programming model similar to that of CUDA or OpenCL.
- Arith: The arith dialect is intended to hold basic integer and floating point mathematical operations. This includes unary, binary, and ternary arithmetic ops, bitwise and shift ops, cast ops, and compare ops. Operations in this dialect also accept vectors and tensors of integers or floats.
- TOSA: TOSA was developed after parallel efforts to rationalize the top-down picture from multiple high-level frameworks, as well as a bottom-up view of different hardware target concerns (CPU, GPU and NPU), and reflects a set of choices that attempt to manage both sets of requirements.
- Func: This dialect contains operations surrounding high order function abstractions, such as calls.

The structure of the MLIR IR is the following:

A module defines a compile unit, the module is made up of one or multiple operations.
An operation is made up of one or multiple regions.
A region is made up of one or multiple blocks.
A block is made up of one or multiple operations.

With this recursive structure, it can define the logic of all the IRs.

Example MLIR code, using multiple dialects:

```mlir
module {
  func.func @foo() {
    %c0 = arith.constant 0 : index
    %0 = scf.while (%arg0 = %c0) : (index) -> f64 {
      %false = arith.constant false
      %cst = arith.constant 4.200000e+01 : f64
      scf.condition(%false) %cst : f64
    } do {
    ^bb0(%arg0: f64):
      %c42 = arith.constant 42 : index
      scf.yield %c42 : index
    }
    return
  }
}
```

In MLIR, blocks can have arguments, this is the MLIR solution to PHI nodes, if a target block uses a variable for multiple independent branches, add its as an argument and the jumps from the predecessors must pass it in the respective jump operation.

You can see in the code that there is a while loop, this is thanks to the SCF dialect, which provides high level control flow operations, if your target is LLVM, this dialect is then converted into blocks and LLVM dialect jumps.

In our case, we want to have a compiled program, so LLVM IR will be our target, this means we have to add passes to convert the multiple dialects we use into the LLVM dialect, and then convert the MLIR to LLVM IR and compile it. This is done either programatically or with `mlir-opt` and `mlir-translate`.


# Other Learning Resources

These are extra resources, they aren't meant to be read now in the workshop but they are here for your convenience.

Resources marked with **→** are best.

- Introduction
    - **→** [2019 EuroLLVM Developers’ Meeting: MLIR: Multi-Level Intermediate Representation Compiler Infrastructure](https://www.youtube.com/watch?v=qzljG6DKgic)
    - → [MLIR: A Compiler Infrastructure for the End of Moore’s Law](https://arxiv.org/pdf/2002.11054.pdf)
    The paper introducing the MLIR framework
        - 7-minute video summary of paper:
        [Read a paper: Multi-level Intermediate Representation (MLIR)](https://www.youtube.com/watch?v=6BwqK6E8v3g)
        - Another version of the paper:
        [MLIR: Scaling Compiler Infrastructure for Domain Specific Computation](https://storage.googleapis.com/pub-tools-public-publication-data/pdf/85bf23fe88bd5c7ff60365bd0c6882928562cbeb.pdf)
- MLIR Tutorial
    - **→** (slides) [MLIR Tutorial (LLVM Dev Mtg, 2020)](https://llvm.org/devmtg/2020-09/slides/MLIR_Tutorial.pdf)
    - **→** (video) [2020 LLVM Developers’ Meeting: M. Amini & R. Riddle “MLIR Tutorial”](https://www.youtube.com/watch?v=Y4SvqTtOIDk)
    - (older slides) [MLIR Tutorial (LLVM Developers Meeting, Euro-LLVM 2019)](https://llvm.org/devmtg/2019-04/slides/Tutorial-AminiVasilacheZinenko-MLIR.pdf)
    - (older slides) [MLIR Tutorial (MLIR 4 HPC, 2019)](https://users.cs.utah.edu/~mhall/mlir4hpc/pienaar-MLIR-Tutorial.pdf)
    - (older video) [2019 EuroLLVM Developers’ Meeting: Mehdi & Vasilache & Zinenko “Building a Compiler with MLIR”](https://www.youtube.com/watch?v=cyICUIZ56wQ)
- **→** Another MLIR Tutorial
https://github.com/j2kun/mlir-tutorial
- **→** [How to build a compiler with LLVM and MLIR](https://www.youtube.com/playlist?list=PLlONLmJCfHTo9WYfsoQvwjsa5ZB6hjOG5)
- Other articles, posts
    - **→** [Intro to LLVM and MLIR with Rust and Melior](https://edgarluque.com/blog/mlir-with-rust/)
    - **→** [MLIR Notes](http://lastweek.io/notes/MLIR/)
    - **→** [Compilers and IRs: LLVM IR, SPIR-V, and MLIR](https://www.lei.chat/posts/compilers-and-irs-llvm-ir-spirv-and-mlir/) [[HN]](https://news.ycombinator.com/item?id=33387149)
    - [MLIR: Redefining the compiler infrastructure](https://iq.opengenus.org/mlir-compiler-infrastructure/)
    - [Pinch: Implementing a borrow-checked language with MLIR](https://badland.io/pinch.md)
- [Official Documentation](https://mlir.llvm.org/docs/)
    - [MLIR Homepage](https://mlir.llvm.org/)
    - [MLIR Language Reference](https://mlir.llvm.org/docs/LangRef/)
    - [MLIR Compiler](https://www.youtube.com/MLIRCompiler) Youtube Channel

### Talks, Presentations, & Videos

- [2020 LLVM in HPC Workshop: Keynote: MLIR: an Agile Infrastructure for Building a Compiler Ecosystem](https://www.youtube.com/watch?v=0bxyZDGs-aA)
- [2021 LLVM Dev Mtg “Representing Concurrency with Graph Regions in MLIR”](https://www.youtube.com/watch?v=Vfk9n3ir_5s)
- [2022 LLVM Dev Mtg: Paths towards unifying LLVM and MLIR](https://www.youtube.com/watch?v=VbFqA9rvxPs)
- [2022 LLVM Dev Mtg: VAST: MLIR for program analysis of C/C++](https://www.youtube.com/watch?v=YFqWa4pxXzM)
- [2022 LLVM Dev Mtg: MLIR for Functional Programming](https://www.youtube.com/watch?v=cyMQbZ0B84Q)
- [2022 EuroLLVM Dev Mtg “Prototyping a Compiler for Homomorphic Encryption Using MLIR”](https://www.youtube.com/watch?v=QyxiqmO6_qQ)
- [cirgen: MLIR based compiler for zk-STARK circuit generation - Frank Laub (RISC Zero)](https://www.youtube.com/watch?v=TsP14-hI_W0)
- [Prototyping a compiler for homomorphic encryption using MLIR](https://www.youtube.com/watch?v=F9qXBuSkQFY)
    - [Slides](https://llvm.org/devmtg/2022-04-03/slides/Prototyping.a.compiler.for.homomorphic.encryption.in.MLIR.pdf)
- [The HEIR Compiler w/ Jeremy Kun](https://www.youtube.com/watch?v=ne5D_kqlxYg)

### Useful code

- [`mlir-rs/melior`](https://github.com/mlir-rs/melior)
- [`mlir-rs/mlir-sys`](https://github.com/mlir-rs/mlir-sys) Rust bindings to the MLIR C API.
- [`GetFirefly/firefly`](https://github.com/GetFirefly/firefly) An alternative BEAM implementation, designed for WebAssembly.

[MLIR Tutorial](https://www.notion.so/MLIR-Tutorial-9480bebef1894384ab291a936a003dc3?pvs=21)

[Misc Resources](https://www.notion.so/Misc-Resources-7f38df3dd83245678f7d44f7051ea2c4?pvs=21)
