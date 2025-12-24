# Hello World - Rust Learning Guide

## Overview

This project introduces you to fundamental Rust concepts including functions, mutable references, and string manipulation. It's your first step into understanding how Rust handles memory safety and ownership.

## Lecture Notes

### 1. Functions in Rust

Functions in Rust are declared using the `fn` keyword. They can take parameters and return values.

**Syntax:**
```rust
fn function_name(parameter: type) -> return_type {
    // function body
}
```

**Key Points:**
- Function names use `snake_case` by convention
- Parameters must have explicit types
- Return type is optional (if omitted, returns `()` - unit type)
- The last expression in a function is automatically returned (no semicolon needed)

### 2. Mutable References (`&mut`)

Rust's ownership system prevents multiple mutable references to the same data. Mutable references allow functions to modify data without taking ownership.

**Example:**
```rust
fn add_world(s: &mut String) {
    s.push_str(" world!");
}
```

**Key Concepts:**
- `&mut` creates a mutable reference
- The caller must pass a mutable variable: `&mut variable_name`
- Only one mutable reference can exist at a time (prevents data races)
- References are automatically dereferenced when calling methods

### 3. String Types in Rust

Rust has two main string types:

**`String` (Owned String):**
- Heap-allocated, growable string
- Owned by a variable
- Can be modified
- Created with `String::from("text")` or `"text".to_string()`

**`&str` (String Slice):**
- Immutable reference to a string
- Can point to `String` or string literals
- More efficient for read-only operations

### 4. The `main()` Function

Every Rust program must have a `main()` function. It's the entry point of your program.

```rust
fn main() {
    // Your code here
}
```

## Code Explanation

Let's break down the code in `src/main.rs`:

```rust
fn add_world(s: &mut String) {
    s.push_str(" world!");
}
```

This function:
- Takes a mutable reference to a `String`
- Appends " world!" to the string
- Doesn't return anything (implicitly returns `()`)

```rust
fn main() {
    let mut s = String::from("Hello");
    add_world(&mut s);
    println!("{}", s); // "Hello world!"
}
```

The `main` function:
1. Creates a mutable `String` containing "Hello"
2. Passes a mutable reference to `add_world()`
3. Prints the modified string

## Key Learning Points

### Ownership Rules
- Each value has a single owner
- When a value goes out of scope, it's dropped
- References allow borrowing without taking ownership

### Mutability
- Variables are immutable by default
- Use `mut` keyword to make variables mutable
- Mutable references (`&mut`) allow modification

### Memory Safety
- Rust prevents dangling pointers at compile time
- No garbage collector needed
- Zero-cost abstractions

## Exercises to Try

1. **Modify the function** to add a different suffix
2. **Create a new function** that takes an immutable reference (`&str`) and prints it
3. **Try removing `mut`** from the variable declaration and see what error you get
4. **Create a function** that returns a new `String` instead of modifying in place

## Common Mistakes

1. **Forgetting `mut`**: If you want to modify a variable, it must be declared as `mut`
2. **Wrong reference type**: Use `&mut` for mutable references, `&` for immutable
3. **Multiple mutable references**: Rust won't allow multiple `&mut` references to the same data

## Next Steps

After mastering this lesson, move on to:
- **01.variable** - Learn about Rust's variable system, types, and immutability

## Additional Resources

- [The Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

