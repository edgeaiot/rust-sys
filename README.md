# Rust Practice Projects

A collection of Rust practice projects for learning and experimentation.

## Projects

### 00.hello
A simple hello program demonstrating Rust fundamentals including functions, mutable references, and string manipulation.

**See:** [GUIDE.md](00.hello/GUIDE.md) for detailed lecture notes.

### 01.variable
Comprehensive examples of Rust variables including immutability, mutability, type annotations, shadowing, and data types.

**See:** [GUIDE.md](01.variable/GUIDE.md) for detailed lecture notes.

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

## Structure

- Each project has its own `Cargo.toml` configuration file
- Source code is located in the `src/` directory of each project
- Each project includes a `GUIDE.md` file with lecture notes and explanations
- Build artifacts are generated in the `target/` directory

## Learning Path

1. **00.hello** - Start here! Learn about functions, mutable references, and basic string operations
2. **01.variable** - Understand Rust's variable system, immutability, and type system
