# Functions in Rust - Learning Guide

## Overview

This project provides a comprehensive introduction to functions in Rust. Functions are the building blocks of Rust programs, allowing you to organize code into reusable, well-defined units. Understanding functions is crucial for writing clean, maintainable Rust code.

## Lecture Notes

### 1. Basic Function Syntax

Functions in Rust are declared using the `fn` keyword:

```rust
fn function_name(parameter: type) -> return_type {
    // function body
}
```

**Key Components:**
- `fn` - keyword to declare a function
- `function_name` - identifier (use `snake_case` convention)
- `parameter: type` - parameters with explicit types
- `-> return_type` - return type (optional, defaults to `()`)
- Function body in curly braces `{}`

### 2. Function Parameters

Functions can take zero or more parameters. Each parameter must have an explicit type:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

**Parameter Rules:**
- Parameters are separated by commas
- Each parameter must have a type annotation
- Parameters are immutable by default
- Use `&mut` for mutable parameters

### 3. Return Values

Rust functions can return values in two ways:

**Implicit Return (Expression):**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = returns this value
}
```

**Explicit Return:**
```rust
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return statement
}
```

**Key Points:**
- Last expression without semicolon is returned
- `return` keyword is optional (usually omitted)
- Return type must match the declared type
- Functions without return type return `()` (unit type)

### 4. Unit Type `()`

Functions that don't return a value implicitly return the unit type `()`:

```rust
fn print_number(n: i32) {
    println!("Number: {}", n);
    // Implicitly returns ()
}
```

This is equivalent to:
```rust
fn print_number(n: i32) -> () {
    println!("Number: {}", n);
}
```

### 5. Multiple Parameters

Functions can take multiple parameters of the same or different types:

```rust
fn calculate(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}
```

### 6. Different Parameter Types

Functions can accept parameters of various types:

```rust
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}
```

**Common Types:**
- Primitive types: `i32`, `f64`, `bool`, `char`
- References: `&str`, `&[T]`, `&mut T`
- Owned types: `String`, `Vec<T>`
- Custom types: structs, enums

### 7. Early Returns

You can return early from a function using the `return` keyword:

```rust
fn check_positive(n: i32) -> Option<i32> {
    if n < 0 {
        return None;  // Early return
    }
    Some(n)  // Normal return
}
```

**When to use early returns:**
- Error handling
- Guard clauses
- Conditional logic
- Simplifying nested conditions

### 8. Returning Multiple Values (Tuples)

Functions can return multiple values using tuples:

```rust
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}
```

**Usage:**
```rust
let (q, r) = divide(17, 5);  // Destructure tuple
```

### 9. Reference Parameters

Passing by reference allows functions to read data without taking ownership:

```rust
fn print_value(x: &i32) {
    println!("Value: {}", x);
}
```

**Benefits:**
- No ownership transfer
- Original value remains accessible
- More efficient for large data
- Allows multiple references

### 10. Mutable Reference Parameters

Mutable references allow functions to modify data:

```rust
fn increment(x: &mut i32) {
    *x += 1;  // Dereference and modify
}
```

**Key Points:**
- Use `&mut` for mutable references
- Must dereference with `*` to modify
- Only one mutable reference at a time
- Caller must pass `&mut variable`

### 11. Returning Owned Types

Functions can return owned types like `String`:

```rust
fn create_greeting(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust!", name)
}
```

**Owned vs Borrowed:**
- `String` - owned, caller takes ownership
- `&str` - borrowed, caller doesn't own
- Choose based on use case

### 12. Nested Function Calls

Functions can call other functions, including themselves (recursion):

```rust
let result = add(multiply(2, 3), multiply(4, 5));
```

**Evaluation:**
- Inner functions evaluated first
- Results passed to outer function
- Right-to-left evaluation for same precedence

### 13. Conditional Logic in Functions

Functions can contain complex logic:

```rust
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

**Note:** `if` is an expression in Rust, so it can be used directly in returns.

### 14. Array/Slice Parameters

Functions can accept array slices:

```rust
fn array_sum(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}
```

**Slice Benefits:**
- Works with arrays of any size
- More flexible than fixed-size arrays
- Efficient (no copying)

### 15. Tuple Parameters

Functions can accept tuples as parameters:

```rust
fn distance_from_origin(point: (f64, f64)) -> f64 {
    let (x, y) = point;  // Destructure
    (x * x + y * y).sqrt()
}
```

## Code Walkthrough

The `main.rs` file demonstrates 15 different function concepts, each building on previous knowledge. Study each example to understand how functions work in Rust.

## Key Learning Points

### Function Design Principles

1. **Single Responsibility**: Each function should do one thing well
2. **Clear Naming**: Function names should clearly describe what they do
3. **Type Safety**: Rust's type system catches errors at compile time
4. **Ownership**: Understand when to use owned vs borrowed types

### Return Value Patterns

1. **Implicit Returns**: Preferred style (no semicolon)
2. **Explicit Returns**: Use for early returns or clarity
3. **Unit Returns**: Functions that don't return values
4. **Multiple Returns**: Use tuples for multiple values

### Parameter Patterns

1. **By Value**: Takes ownership (for small types)
2. **By Reference**: Borrows (`&T` for immutable, `&mut T` for mutable)
3. **Slices**: Flexible array-like parameters (`&[T]`)
4. **Tuples**: Group related parameters

## Exercises to Try

1. **Create a function** that calculates the area of a circle
2. **Write a function** that swaps two numbers using mutable references
3. **Implement a function** that finds the minimum of three numbers
4. **Create a function** that returns both sum and product of two numbers
5. **Write a function** that takes a string and returns its length
6. **Implement a recursive function** (e.g., factorial or Fibonacci)
7. **Create a function** that takes a vector and returns its average

## Common Mistakes

1. **Missing semicolon**: Adding semicolon to last expression prevents return
2. **Wrong return type**: Return type must match function signature
3. **Forgetting `mut`**: Can't modify immutable parameters
4. **Ownership issues**: Moving values when you meant to borrow
5. **Type mismatches**: Parameter types must match exactly
6. **Dereferencing**: Forgetting `*` when modifying through references

## Function vs Method

**Functions:**
- Standalone, not associated with a type
- Called with `function_name()`

**Methods:**
- Associated with a type (struct, enum)
- Called with `instance.method_name()`
- First parameter is `self` (or `&self`, `&mut self`)

We'll cover methods in a later lesson!

## Advanced Topics (Future Lessons)

- **Closures**: Anonymous functions
- **Function pointers**: Passing functions as parameters
- **Generics**: Functions that work with multiple types
- **Traits**: Defining function behavior
- **Methods**: Functions associated with types
- **Associated functions**: Functions on types (like constructors)

## Best Practices

### Function Design
1. **Keep Functions Small**: Single responsibility principle
   ```rust
   // Good: Small, focused function
   fn calculate_area(width: f64, height: f64) -> f64 {
       width * height
   }
   
   // Avoid: Too many responsibilities
   fn process_data(input: &str) -> String {
       // Parsing, validation, transformation, formatting...
       // Too much in one function
   }
   ```

2. **Use Descriptive Names**: Function names should clearly describe purpose
   ```rust
   // Good: Clear intent
   fn calculate_total_price(items: &[Item]) -> f64 { }
   fn validate_email(email: &str) -> bool { }
   
   // Avoid: Vague names
   fn calc(x: &[Item]) -> f64 { }
   fn check(s: &str) -> bool { }
   ```

3. **Follow Naming Conventions**: Use `snake_case` for functions
   ```rust
   // Good: snake_case
   fn get_user_name() { }
   fn calculate_total() { }
   ```

### Parameter Design
1. **Prefer Borrowing Over Ownership**: Use references when you don't need ownership
   ```rust
   // Good: Borrows, doesn't take ownership
   fn print_name(name: &str) {
       println!("{}", name);
   }
   
   // Less ideal: Takes ownership (unless you need it)
   fn print_name(name: String) {
       println!("{}", name);
   }
   ```

2. **Use Immutable References by Default**: Only use `&mut` when modification is necessary
   ```rust
   // Good: Immutable reference for reading
   fn display_value(x: &i32) { }
   
   // Only use mutable when needed
   fn increment_value(x: &mut i32) {
       *x += 1;
   }
   ```

3. **Limit Parameter Count**: Too many parameters suggest the function does too much
   ```rust
   // Good: Few parameters
   fn calculate(x: f64, y: f64) -> f64 { }
   
   // Consider: Using a struct for many parameters
   struct Config {
       host: String,
       port: u16,
       timeout: u64,
       retries: u32,
   }
   fn connect(config: &Config) { }
   ```

4. **Use Slices for Collections**: More flexible than fixed-size arrays
   ```rust
   // Good: Works with any size
   fn sum_numbers(numbers: &[i32]) -> i32 { }
   
   // Less flexible: Fixed size
   fn sum_numbers(numbers: [i32; 5]) -> i32 { }
   ```

### Return Values
1. **Prefer Implicit Returns**: More idiomatic Rust style
   ```rust
   // Good: Implicit return
   fn add(a: i32, b: i32) -> i32 {
       a + b  // No semicolon
   }
   
   // Also fine: Explicit return (for early returns)
   fn divide(a: i32, b: i32) -> Option<i32> {
       if b == 0 {
           return None;  // Early return
       }
       Some(a / b)
   }
   ```

2. **Return Early for Error Cases**: Use guard clauses
   ```rust
   // Good: Early return for errors
   fn process(value: i32) -> Option<i32> {
       if value < 0 {
           return None;
       }
       // Main logic here
       Some(value * 2)
   }
   ```

3. **Use Tuples for Multiple Returns**: Clear and idiomatic
   ```rust
   // Good: Tuple for multiple values
   fn divide(a: i32, b: i32) -> (i32, i32) {
       (a / b, a % b)
   }
   ```

4. **Return Appropriate Types**: Use `Option` for nullable, `Result` for errors
   ```rust
   // Good: Option for nullable
   fn find_item(id: u32) -> Option<Item> { }
   
   // Good: Result for errors
   fn parse_number(s: &str) -> Result<i32, ParseIntError> { }
   ```

### Documentation
1. **Document Public Functions**: Use doc comments for public API
   ```rust
   /// Calculates the area of a rectangle.
   /// 
   /// # Arguments
   /// 
   /// * `width` - The width of the rectangle
   /// * `height` - The height of the rectangle
   /// 
   /// # Returns
   /// 
   /// The area of the rectangle as a `f64`.
   /// 
   /// # Example
   /// 
   /// ```
   /// let area = rectangle_area(5.0, 3.0);
   /// assert_eq!(area, 15.0);
   /// ```
   fn rectangle_area(width: f64, height: f64) -> f64 {
       width * height
   }
   ```

2. **Explain Why, Not What**: Comments should explain reasoning
   ```rust
   // Good: Explains why
   // Use mutable reference to avoid cloning the entire vector
   fn sort_items(items: &mut Vec<Item>) { }
   
   // Avoid: States the obvious
   // This function sorts items
   fn sort_items(items: &mut Vec<Item>) { }
   ```

### Error Handling
1. **Use Result for Recoverable Errors**: Return `Result<T, E>` for operations that can fail
   ```rust
   // Good: Result for errors
   fn divide(a: f64, b: f64) -> Result<f64, String> {
       if b == 0.0 {
           return Err("Division by zero".to_string());
       }
       Ok(a / b)
   }
   ```

2. **Use Option for Nullable Values**: When a value might not exist
   ```rust
   // Good: Option for nullable
   fn find_user(id: u32) -> Option<User> {
       // Returns Some(user) or None
   }
   ```

### Performance
1. **Avoid Unnecessary Cloning**: Pass by reference when possible
   ```rust
   // Good: No clone needed
   fn process(data: &String) { }
   
   // Avoid: Unnecessary clone
   fn process(data: String) { }  // Takes ownership
   ```

2. **Use Slices for Large Collections**: More efficient than owned types
   ```rust
   // Good: Efficient slice
   fn sum(numbers: &[i32]) -> i32 { }
   
   // Less efficient: Owns the vector
   fn sum(numbers: Vec<i32>) -> i32 { }
   ```

### Code Organization
1. **Group Related Functions**: Organize functions logically
2. **Keep Functions Close to Usage**: Define functions near where they're used
3. **Extract Complex Logic**: Break complex functions into smaller ones
   ```rust
   // Good: Broken into smaller functions
   fn process_user(user: &User) -> Result<(), Error> {
       validate_user(user)?;
       save_user(user)?;
       send_notification(user)?;
       Ok(())
   }
   ```

### Type Safety
1. **Leverage Type System**: Let Rust catch errors at compile time
2. **Use Strong Types**: Avoid `String` when a more specific type works
   ```rust
   // Good: Specific type
   struct Email(String);
   fn send_email(to: Email) { }
   
   // Less ideal: Generic string
   fn send_email(to: String) { }
   ```

3. **Be Explicit When Helpful**: Type annotations clarify intent
   ```rust
   // Good: Type clarifies intent
   fn calculate(port: u16, timeout: u64) { }
   ```

## Next Steps

After mastering functions, you're ready for:
- **Control Flow** - if/else, loops, match
- **Structs** - Custom data types
- **Methods** - Functions on types
- **Error Handling** - Result and Option types
- **Ownership Deep Dive** - Advanced borrowing

## Additional Resources

- [The Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example - Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
- [Rust by Example - Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [Rust Reference - Functions](https://doc.rust-lang.org/reference/items/functions.html)

