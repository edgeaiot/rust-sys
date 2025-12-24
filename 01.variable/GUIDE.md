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

## Best Practices

### Immutability
1. **Prefer Immutable Variables**: Only use `mut` when you actually need to modify
   ```rust
   // Good: Immutable by default
   let x = 5;
   let name = "Alice";
   
   // Only use mut when needed
   let mut counter = 0;
   counter += 1;
   ```

2. **Question Mutability**: Before adding `mut`, ask "Do I really need to change this?"
   ```rust
   // Often you can avoid mut with shadowing
   let value = 5;
   let value = value + 1;  // Shadowing instead of mut
   ```

### Type Annotations
1. **Be Explicit When Helpful**: Add type annotations when types aren't obvious
   ```rust
   // Good: Type is clear from context
   let x = 5;
   
   // Good: Type annotation clarifies intent
   let port: u16 = 8080;
   let max_users: usize = 100;
   ```

2. **Use Type Annotations for Constants**: Always required, but good practice
   ```rust
   // Required for constants
   const MAX_CONNECTIONS: u32 = 100;
   const TIMEOUT_SECONDS: f64 = 30.0;
   ```

3. **Let Inference Work**: Don't over-annotate when types are obvious
   ```rust
   // Good: Inference is clear
   let name = "Alice";
   let age = 25;
   
   // Unnecessary: Type is obvious
   let name: &str = "Alice";  // Usually not needed
   ```

### Variable Shadowing
1. **Shadowing is Idiomatic**: It's a Rust feature, not a bug
   ```rust
   // Good: Converting types
   let input = "42";
   let input: i32 = input.parse().unwrap();
   ```

2. **Use Shadowing for Transformations**: When you're transforming a value
   ```rust
   // Good: Clear transformation
   let user_input = get_input();
   let user_input = user_input.trim();
   let user_input = user_input.to_lowercase();
   ```

3. **Avoid Confusing Shadowing**: Don't shadow in ways that confuse readers
   ```rust
   // Avoid: Confusing shadowing
   let x = 5;
   let x = "hello";  // Completely different type, might confuse
   ```

### Constants vs Variables
1. **Use Constants for Magic Numbers**: Replace magic numbers with named constants
   ```rust
   // Good: Named constant
   const MAX_RETRIES: u32 = 3;
   if retry_count < MAX_RETRIES { }
   
   // Avoid: Magic number
   if retry_count < 3 { }  // What does 3 mean?
   ```

2. **Constants for Configuration**: Values that don't change during execution
   ```rust
   const API_BASE_URL: &str = "https://api.example.com";
   const DEFAULT_TIMEOUT: u64 = 30;
   ```

3. **Use Variables for Runtime Values**: Values that change or are computed
   ```rust
   // Good: Runtime value
   let user_age = calculate_age(birth_date);
   let current_time = get_current_time();
   ```

### String Types
1. **Prefer `&str` for Parameters**: More flexible, accepts both `&str` and `String`
   ```rust
   // Good: Accepts both types
   fn process(text: &str) { }
   
   process("literal");
   process(&String::from("owned"));
   ```

2. **Use `String` When You Need Ownership**: For building or modifying strings
   ```rust
   // Good: Need to own and modify
   let mut message = String::from("Hello");
   message.push_str(" world!");
   ```

3. **Avoid Unnecessary Conversions**: Don't convert `&str` to `String` unnecessarily
   ```rust
   // Avoid: Unnecessary conversion
   let s = "hello".to_string();
   
   // Good: Use string literal directly
   let s = "hello";
   ```

### Type Selection
1. **Choose Appropriate Integer Types**: Consider size and signedness
   ```rust
   // Good: Appropriate types
   let age: u8 = 25;           // Age can't be negative
   let temperature: i16 = -5;   // Temperature can be negative
   let file_size: u64 = 1024;   // File sizes are large
   ```

2. **Use `usize` for Indices**: Standard for array/vector indexing
   ```rust
   // Good: usize for indexing
   let index: usize = 0;
   let item = array[index];
   ```

3. **Prefer `f64` Over `f32`**: Unless you have specific memory constraints
   ```rust
   // Good: f64 is default
   let pi: f64 = 3.14159;
   ```

### Variable Naming
1. **Use Descriptive Names**: Names should explain purpose
   ```rust
   // Good: Clear purpose
   let user_count = 10;
   let max_retries = 3;
   
   // Avoid: Vague names
   let x = 10;
   let temp = 3;
   ```

2. **Follow Rust Conventions**: Use `snake_case` for variables
   ```rust
   // Good: snake_case
   let user_name = "Alice";
   let max_connections = 100;
   ```

3. **Use Constants for Constants**: `SCREAMING_SNAKE_CASE` for constants
   ```rust
   // Good: SCREAMING_SNAKE_CASE for constants
   const MAX_USERS: u32 = 1000;
   const API_VERSION: &str = "v1";
   ```

### Initialization
1. **Initialize Variables**: Rust requires initialization, use it wisely
   ```rust
   // Good: Initialize when declared
   let value = 42;
   
   // Good: Initialize in conditional
   let value = if condition { 10 } else { 20 };
   ```

2. **Avoid Uninitialized Variables**: Rust prevents this, but be aware
   ```rust
   // Rust won't let you use uninitialized variables
   let x: i32;
   // println!("{}", x);  // Error!
   x = 42;  // Must initialize before use
   ```

### Performance Tips
1. **Prefer Stack Types**: Use stack-allocated types when possible
   ```rust
   // Good: Stack-allocated
   let number: i32 = 42;
   let array: [i32; 5] = [1, 2, 3, 4, 5];
   ```

2. **Use References**: Avoid unnecessary cloning
   ```rust
   // Good: Borrow instead of clone
   let text = String::from("hello");
   process(&text);  // Borrow
   
   // Avoid: Unnecessary clone
   process(text.clone());  // Clones entire string
   ```

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

