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

## Best Practices

### Function Design
1. **Single Responsibility**: Each function should do one thing well
   ```rust
   // Good: Single purpose
   fn add_world(s: &mut String) {
       s.push_str(" world!");
   }
   
   // Avoid: Multiple responsibilities
   fn process_and_print(s: &mut String) {
       s.push_str(" world!");
       println!("{}", s);
       // Too many responsibilities
   }
   ```

2. **Clear Function Names**: Use descriptive names that explain what the function does
   ```rust
   // Good: Clear intent
   fn append_world(s: &mut String) { }
   
   // Avoid: Vague names
   fn do_stuff(s: &mut String) { }
   ```

3. **Prefer Borrowing Over Ownership**: Use references when you don't need ownership
   ```rust
   // Good: Borrows, doesn't take ownership
   fn print_string(s: &str) { }
   
   // Less ideal: Takes ownership (unless you need it)
   fn print_string(s: String) { }
   ```

### Reference Usage
1. **Use Immutable References by Default**: Only use `&mut` when modification is necessary
   ```rust
   // Good: Immutable reference for reading
   fn display(s: &str) {
       println!("{}", s);
   }
   
   // Only use mutable when needed
   fn modify(s: &mut String) {
       s.push_str("!");
   }
   ```

2. **Minimize Mutable References**: Rust's borrow checker prevents data races, but prefer immutability
   ```rust
   // Good: Immutable when possible
   let s = String::from("Hello");
   display(&s);
   
   // Only use mut when you need to change
   let mut s = String::from("Hello");
   modify(&mut s);
   ```

### String Handling
1. **Choose the Right String Type**: Use `&str` for function parameters when you don't need ownership
   ```rust
   // Good: Accepts both &str and String
   fn greet(name: &str) {
       println!("Hello, {}!", name);
   }
   
   // Can call with:
   greet("Alice");              // &str
   greet(&String::from("Bob")); // &String coerces to &str
   ```

2. **Use `String` for Ownership**: When you need to own and modify strings
   ```rust
   // Good: Owns and can modify
   let mut s = String::from("Hello");
   s.push_str(" world!");
   ```

3. **Avoid Unnecessary Allocations**: Prefer string slices for read-only operations
   ```rust
   // Good: No allocation
   fn process(text: &str) { }
   
   // Avoid: Unnecessary allocation
   fn process(text: String) { }
   ```

### Code Organization
1. **Keep Functions Focused**: Small, focused functions are easier to test and maintain
2. **Document Complex Logic**: Use comments to explain why, not what
   ```rust
   // Good: Explains why
   // Use mutable reference to avoid cloning the entire string
   fn append_suffix(s: &mut String) { }
   ```

3. **Follow Rust Conventions**: Use `snake_case` for functions, clear parameter names
   ```rust
   // Good: Follows conventions
   fn add_world_to_string(s: &mut String) { }
   ```

### Error Prevention
1. **Leverage the Compiler**: Let Rust's type system catch errors at compile time
2. **Trust the Borrow Checker**: It prevents many common bugs
3. **Read Compiler Messages**: Rust's error messages are helpful - learn from them

### Performance Considerations
1. **Prefer References**: Passing by reference avoids copying
2. **Use Slices**: `&str` and `&[T]` are more efficient than owned types for reading
3. **Minimize Mutations**: Immutable code is easier to optimize

## Next Steps

After mastering this lesson, move on to:
- **01.variable** - Learn about Rust's variable system, types, and immutability

## Additional Resources

- [The Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

