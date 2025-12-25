# Rust Practice Projects

A collection of Rust practice projects for learning and experimentation.

## Projects

### 00.hello
A simple hello program demonstrating Rust fundamentals including functions, mutable references, and string manipulation.

**See:** [GUIDE.md](00.hello/GUIDE.md) for detailed lecture notes.

### 01.variable
Comprehensive examples of Rust variables including immutability, mutability, type annotations, shadowing, and data types.

**See:** [GUIDE.md](01.variable/GUIDE.md) for detailed lecture notes.

### 02.function
In-depth exploration of Rust functions including parameters, return values, references, mutable references, and various function patterns.

**See:** [GUIDE.md](02.function/GUIDE.md) for detailed lecture notes.

### 03.control_flow
Comprehensive guide to Rust control flow including if/else expressions, loops (loop, while, for), pattern matching with match, and if let/while let.

**See:** [GUIDE.md](03.control_flow/GUIDE.md) for detailed lecture notes.

## Building and Running

To build all projects, use:
```bash
cargo build
```

To run a specific project, navigate to its directory and use:
```bash
cd 00.hello
cargo run
```

Or:
```bash
cd 01.variable
cargo run
```

Or:
```bash
cd 02.function
cargo run
```

Or:
```bash
cd 03.control_flow
cargo run
```

## Structure

- Each project has its own `Cargo.toml` configuration file
- Source code is located in the `src/` directory of each project
- Each project includes a `GUIDE.md` file with lecture notes and explanations
- Build artifacts are generated in the `target/` directory

## Learning Path

1. **00.hello** - Start here! Learn about functions, mutable references, and basic string operations
2. **01.variable** - Understand Rust's variable system, immutability, and type system
3. **02.function** - Master Rust functions: parameters, return values, references, and function patterns
4. **03.control_flow** - Learn control flow: if/else, loops, and powerful pattern matching with match
