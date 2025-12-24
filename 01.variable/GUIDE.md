# Variables in Rust - Learning Guide

## Overview

This project comprehensively covers Rust's variable system, including immutability, mutability, type annotations, shadowing, and various data types. Understanding variables is fundamental to writing Rust code effectively.

## Lecture Notes

### 1. Immutable Variables (Default)

In Rust, variables are **immutable by default**. This is a core safety feature that prevents accidental modifications.

```rust
let x = 5;
// x = 6; // Error! Cannot assign to immutable variable
```

**Why immutability?**
- Prevents bugs from accidental mutations
- Makes code easier to reason about
- Enables safe concurrent programming
- Compiler can optimize better

### 2. Mutable Variables (`mut`)

When you need to change a value, use the `mut` keyword:

```rust
let mut y = 10;
y = 20; // OK! Variable is mutable
```

**Key Points:**
- Explicit mutability makes code intent clear
- Only use `mut` when you actually need to modify the value
- Mutable variables can still be shadowed

### 3. Type Annotations

Rust can infer types, but you can also specify them explicitly:

```rust
let integer: i32 = 42;
let floating: f64 = 3.14;
let boolean: bool = true;
let character: char = 'A';
```

**Common Types:**
- **Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (signed)
- **Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (unsigned)
- **Floats**: `f32`, `f64`
- **Boolean**: `bool` (true/false)
- **Character**: `char` (4 bytes, Unicode scalar value)

### 4. Variable Shadowing

Rust allows you to "shadow" a variable by declaring a new variable with the same name:

```rust
let shadow = 5;
let shadow = shadow + 1; // New variable, can be different type
let shadow = "now I'm a string!"; // Completely different type
```

**Shadowing vs. Mutability:**
- **Shadowing**: Creates a new variable, can change type
- **Mutability**: Modifies existing variable, same type

**When to use shadowing:**
- Converting between types
- Reusing variable names in different scopes
- Making immutable variables "effectively mutable" in a new scope

### 5. Constants (`const`)

Constants are always immutable and must have type annotations:

```rust
const MAX_POINTS: u32 = 100_000;
```

**Constants vs. Variables:**
- Constants are evaluated at compile time
- Must have explicit type annotation
- Can use underscores for readability (e.g., `100_000`)
- Valid for the entire program lifetime
- Convention: use `SCREAMING_SNAKE_CASE`

### 6. Multiple Variable Assignment

You can declare multiple variables at once using tuple destructuring:

```rust
let (a, b, c) = (1, 2, 3);
```

This is useful for:
- Returning multiple values from functions
- Swapping variables
- Pattern matching

### 7. Uninitialized Variables

Rust requires variables to be initialized before use:

```rust
let uninitialized: i32;
// println!("{}", uninitialized); // Error!
uninitialized = 100; // Must initialize before use
```

**Why?**
- Prevents use of undefined values
- Compiler ensures all code paths initialize variables
- Eliminates a whole class of bugs

### 8. String Types

Rust has two main string types:

**`&str` (String Slice):**
- Immutable reference to string data
- Can point to string literals or `String` data
- Fixed size (pointer + length)
- More efficient for read-only operations

**`String` (Owned String):**
- Heap-allocated, growable string
- Owned by a variable
- Can be modified
- Created with `String::from("text")` or `"text".to_string()`

```rust
let string_literal = "Hello"; // &str
let owned_string = String::from("World"); // String
```

### 9. Arrays

Arrays in Rust have a fixed size and must contain elements of the same type:

```rust
let array = [1, 2, 3, 4, 5];
let first = array[0]; // Access by index
```

**Key Points:**
- Size is part of the type: `[i32; 5]` means array of 5 i32s
- Indexing is bounds-checked (panics if out of bounds)
- Arrays are stack-allocated

### 10. Tuples

Tuples can contain different types and have a fixed size:

```rust
let tuple: (i32, f64, bool) = (42, 3.14, true);
let first = tuple.0;  // Access by index (0-based)
let second = tuple.1;
```

**Key Points:**
- Can contain mixed types
- Fixed size (cannot grow)
- Access elements with `.0`, `.1`, `.2`, etc.
- Useful for returning multiple values from functions

## Code Walkthrough

The `main.rs` file demonstrates all these concepts with practical examples. Each section builds on the previous one, showing how Rust's variable system works together.

## Key Learning Points

### Rust's Philosophy
1. **Safety First**: Immutability by default prevents bugs
2. **Explicit over Implicit**: Use `mut` when you need mutation
3. **Zero-Cost Abstractions**: Safety doesn't mean slower code
4. **Compile-Time Guarantees**: Many errors caught before runtime

### Best Practices
1. **Prefer immutability**: Only use `mut` when necessary
2. **Use type annotations**: When types aren't obvious
3. **Shadowing is OK**: It's idiomatic Rust, not a code smell
4. **Constants for magic numbers**: Use `const` for values that never change

## Exercises to Try

1. **Experiment with types**: Try different integer sizes and see what happens
2. **Shadowing practice**: Create a variable, shadow it with different types
3. **Array manipulation**: Create arrays of different types and sizes
4. **Tuple destructuring**: Practice extracting values from tuples
5. **String operations**: Try modifying `String` vs `&str`
6. **Constants**: Create constants for common values in your code

## Common Mistakes

1. **Forgetting `mut`**: Trying to modify immutable variables
2. **Type mismatches**: Assigning wrong types to variables
3. **Uninitialized variables**: Using variables before initialization
4. **Array bounds**: Accessing array indices that don't exist
5. **String confusion**: Mixing up `String` and `&str`

## Type Inference

Rust's compiler is smart! It can often infer types:

```rust
let x = 5;        // Inferred as i32
let y = 3.14;     // Inferred as f64
let z = true;     // Inferred as bool
```

But sometimes you need to be explicit:
```rust
let x = 5;        // Could be i8, i16, i32, i64, etc.
let x: u8 = 5;    // Explicitly u8
```

## Next Steps

After mastering variables, you're ready for:
- **Functions** (covered in 00.hello)
- **Control flow** (if/else, loops)
- **Ownership and borrowing** (advanced)
- **Structs and enums** (custom types)

## Additional Resources

- [The Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example - Variables](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)
- [Rust by Example - Types](https://doc.rust-lang.org/rust-by-example/primitives.html)

