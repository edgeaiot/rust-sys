# Control Flow in Rust - Learning Guide

## Overview

This project provides a comprehensive introduction to control flow in Rust, including conditional statements (if/else), loops (loop, while, for), and pattern matching (match). Control flow is essential for creating programs that make decisions and repeat operations.

## Lecture Notes

### 1. If/Else Statements

Rust's `if` statement is similar to other languages, but with some important differences.

**Basic Syntax:**
```rust
if condition {
    // code block
} else {
    // code block
}
```

**Key Points:**
- Condition must be a `bool` type (no truthy/falsy values)
- Braces are required (even for single statements)
- Can chain with `else if`

### 2. If as Expression

In Rust, `if` is an **expression**, not just a statement. This means it can return a value:

```rust
let result = if condition { 5 } else { 6 };
```

**Important Rules:**
- All branches must return the same type
- No semicolon after the value (semicolon makes it a statement)
- Very useful for functional-style programming

### 3. Logical Operators

Rust uses standard logical operators:
- `&&` - AND (short-circuit evaluation)
- `||` - OR (short-circuit evaluation)
- `!` - NOT

```rust
if age >= 18 && has_license {
    // Both conditions must be true
}
```

### 4. Loop (Infinite Loop)

The `loop` keyword creates an infinite loop. You must use `break` to exit:

```rust
loop {
    // code
    if condition {
        break; // Exit loop
    }
}
```

**Returning Values:**
Loops can return values using `break`:
```rust
let result = loop {
    if condition {
        break 42; // Returns 42
    }
};
```

### 5. While Loop

`while` loops continue as long as a condition is true:

```rust
while condition {
    // code
}
```

**Use Cases:**
- When you don't know how many iterations
- When condition depends on loop body
- Polling or waiting for conditions

### 6. For Loop

`for` loops iterate over collections. This is the most common loop in Rust:

```rust
for item in collection {
    // code
}
```

**Ranges:**
- `1..5` - 1, 2, 3, 4 (exclusive end)
- `1..=5` - 1, 2, 3, 4, 5 (inclusive end)

**Iterating Collections:**
```rust
let arr = [1, 2, 3];
for element in arr.iter() {
    // element is &i32
}
```

**With Index:**
```rust
for (index, value) in items.iter().enumerate() {
    // index is usize, value is &T
}
```

### 7. Break and Continue

- `break` - Exits the loop immediately
- `continue` - Skips to next iteration

```rust
for i in 1..=10 {
    if i == 3 {
        continue; // Skip 3
    }
    if i == 8 {
        break; // Stop at 8
    }
}
```

### 8. Loop Labels

You can label loops to break or continue outer loops:

```rust
'outer: for i in 1..=3 {
    for j in 1..=3 {
        if condition {
            break 'outer; // Breaks outer loop
        }
    }
}
```

**Naming Convention:**
- Labels use `'label_name` syntax
- Single quotes required
- Useful for nested loops

### 9. Match Expression

`match` is Rust's powerful pattern matching construct. It's exhaustive - all cases must be handled:

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default, // Catch-all pattern
}
```

**Key Features:**
- Exhaustive checking (compiler ensures all cases covered)
- Pattern matching (not just equality)
- Can return values
- More powerful than switch statements

### 10. Match Patterns

**Literal Patterns:**
```rust
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}
```

**Multiple Patterns:**
```rust
match number {
    1 | 2 | 3 => println!("Small"),
    4 | 5 | 6 => println!("Medium"),
    _ => println!("Large"),
}
```

**Range Patterns:**
```rust
match age {
    0..=12 => println!("Child"),
    13..=19 => println!("Teenager"),
    _ => println!("Adult"),
}
```

### 11. Match with Destructuring

You can destructure tuples, structs, and enums in match:

```rust
let point = (3, 5);
match point {
    (0, 0) => println!("Origin"),
    (0, y) => println!("On y-axis at {}", y),
    (x, 0) => println!("On x-axis at {}", x),
    (x, y) => println!("Point at ({}, {})", x, y),
}
```

### 12. Match Guards

Add conditions to patterns with `if`:

```rust
match number {
    Some(x) if x < 5 => println!("Less than 5"),
    Some(x) if x >= 5 => println!("Greater or equal to 5"),
    Some(_) => println!("Other"),
    None => println!("None"),
}
```

### 13. Match with Option

`match` is perfect for handling `Option`:

```rust
match some_value {
    Some(value) => println!("Got: {}", value),
    None => println!("No value"),
}
```

### 14. Match with Result

Similarly for `Result`:

```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

### 15. If Let

`if let` is a concise way to match a single pattern:

```rust
if let Some(value) = option {
    // Use value
} else {
    // Handle None
}
```

**When to Use:**
- Only care about one pattern
- Want simpler syntax than full match
- Good for unwrapping Option/Result

### 16. While Let

`while let` combines pattern matching with loops:

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

**Use Cases:**
- Iterating until pattern fails
- Processing until None/Err
- Popping from stacks/queues

### 17. Match Binding

Bind values in patterns:

```rust
match point {
    (0, y) => println!("y-axis at {}", y),
    (x, y) => println!("Point ({}, {})", x, y),
}
```

### 18. Match with @ Binding

Use `@` to bind and match simultaneously:

```rust
match age {
    n @ 0..=12 => println!("Child (age {})", n),
    n @ 13..=19 => println!("Teenager (age {})", n),
    n => println!("Adult (age {})", n),
}
```

## Code Walkthrough

The `main.rs` file demonstrates 24 different control flow concepts, from basic conditionals to advanced pattern matching. Each example builds on previous knowledge.

## Key Learning Points

### Control Flow Philosophy

1. **Expressions Over Statements**: Rust favors expressions (return values)
2. **Exhaustive Matching**: Compiler ensures all cases are handled
3. **Pattern Matching**: Powerful way to destructure and match
4. **Type Safety**: Conditions must be bool, no truthy/falsy

### When to Use What

**If/Else:**
- Simple conditions
- Binary decisions
- When you need if as expression

**Loop:**
- Infinite loops
- When you need to return a value from loop
- Retry logic

**While:**
- Condition-based iteration
- Unknown iteration count
- Polling/waiting

**For:**
- Iterating collections (most common)
- Known iteration count
- Range-based iteration

**Match:**
- Multiple cases
- Pattern matching
- Exhaustive checking needed
- Option/Result handling

**If Let / While Let:**
- Single pattern matching
- Simpler than full match
- Unwrapping Option/Result

## Exercises to Try

1. **FizzBuzz**: Print numbers 1-100, but "Fizz" for multiples of 3, "Buzz" for 5, "FizzBuzz" for both
2. **Factorial**: Calculate factorial using different loop types
3. **Pattern Matching**: Create a function that matches on different tuple patterns
4. **Option Handling**: Write functions that handle Option with match and if let
5. **Result Handling**: Process Result types with match
6. **Nested Loops**: Use loop labels to break outer loops
7. **Guards**: Create match expressions with guards for complex conditions

## Common Mistakes

1. **Forgetting Braces**: Rust requires braces for all blocks
2. **Type Mismatch in If Expression**: All branches must return same type
3. **Non-Bool Condition**: Conditions must be bool (no truthy/falsy)
4. **Missing Match Cases**: Match must be exhaustive (use `_` for catch-all)
5. **Semicolon in If Expression**: Adding semicolon makes it a statement
6. **Infinite Loops**: Forgetting break in loop
7. **Wrong Range Syntax**: `..` vs `..=` (exclusive vs inclusive)

## Best Practices

### If/Else
1. **Prefer If Expressions**: Use `if` as expression when returning values
   ```rust
   // Good: Expression
   let result = if condition { 5 } else { 6 };
   
   // Less ideal: Statement
   let result;
   if condition {
       result = 5;
   } else {
       result = 6;
   }
   ```

2. **Keep Conditions Simple**: Extract complex conditions to variables
   ```rust
   // Good: Clear condition
   let can_drive = age >= 18 && has_license;
   if can_drive { }
   
   // Avoid: Complex inline condition
   if age >= 18 && has_license && !is_suspended { }
   ```

3. **Use Early Returns**: Return early to reduce nesting
   ```rust
   // Good: Early return
   if !condition {
       return;
   }
   // Main logic
   ```

### Loops
1. **Prefer For Loops**: Most common and idiomatic
   ```rust
   // Good: For loop
   for item in items.iter() { }
   
   // Less common: While loop
   let mut i = 0;
   while i < items.len() {
       // ...
       i += 1;
   }
   ```

2. **Use Iterators**: Prefer iterator methods when possible
   ```rust
   // Good: Iterator
   for (i, item) in items.iter().enumerate() { }
   ```

3. **Avoid Infinite Loops**: Use `loop` only when necessary
   ```rust
   // Good: Clear exit condition
   while !done {
       // ...
   }
   
   // Only when needed
   loop {
       if condition {
           break;
       }
   }
   ```

4. **Use Break with Values**: Return values from loops
   ```rust
   // Good: Return value
   let result = loop {
       if found {
           break value;
       }
   };
   ```

### Match
1. **Always Exhaustive**: Handle all cases or use `_`
   ```rust
   // Good: Exhaustive
   match value {
       Some(x) => x,
       None => 0,
   }
   ```

2. **Order Matters**: More specific patterns first
   ```rust
   // Good: Specific first
   match point {
       (0, 0) => "origin",
       (0, _) => "y-axis",
       (_, 0) => "x-axis",
       (_, _) => "other",
   }
   ```

3. **Use Guards for Complex Conditions**: When pattern isn't enough
   ```rust
   // Good: Guard for complex condition
   match value {
       Some(x) if x > 0 && x < 100 => "valid",
       Some(_) => "invalid",
       None => "missing",
   }
   ```

4. **Prefer Match Over If-Else Chains**: More powerful and safer
   ```rust
   // Good: Match
   match value {
       1 => "one",
       2 => "two",
       _ => "other",
   }
   
   // Less ideal: Long if-else chain
   if value == 1 {
       "one"
   } else if value == 2 {
       "two"
   } else {
       "other"
   }
   ```

### If Let / While Let
1. **Use for Single Patterns**: When you only care about one case
   ```rust
   // Good: Single pattern
   if let Some(value) = option {
       // Use value
   }
   
   // Overkill: Full match for one case
   match option {
       Some(value) => { /* use value */ },
       None => {},
   }
   ```

2. **Combine with Else**: Handle the other case
   ```rust
   // Good: Handle both cases
   if let Some(value) = option {
       // Handle Some
   } else {
       // Handle None
   }
   ```

### Pattern Matching
1. **Destructure When Useful**: Extract values from structures
   ```rust
   // Good: Destructure
   match point {
       (x, y) if x == y => "diagonal",
       (x, y) => format!("({}, {})", x, y),
   }
   ```

2. **Use @ for Binding**: When you need both pattern and value
   ```rust
   // Good: Bind and match
   match age {
       n @ 0..=12 => format!("Child: {}", n),
       n => format!("Adult: {}", n),
   }
   ```

3. **Ignore Unused Values**: Use `_` for values you don't need
   ```rust
   // Good: Ignore unused
   match tuple {
       (x, _, z) => x + z, // Don't need y
   }
   ```

### Error Handling
1. **Match Result**: Always handle both Ok and Err
   ```rust
   // Good: Handle both
   match result {
       Ok(value) => process(value),
       Err(e) => handle_error(e),
   }
   ```

2. **Use If Let for Simple Cases**: When you only care about success
   ```rust
   // Good: Simple success case
   if let Ok(value) = result {
       process(value);
   }
   ```

## Performance Considerations

1. **For Loops are Fast**: Iterator methods are optimized
2. **Match is Efficient**: Compiler optimizes match expressions
3. **Avoid Unnecessary Clones**: Use references in loops
   ```rust
   // Good: Reference
   for item in items.iter() { }
   
   // Avoid: Unnecessary clone
   for item in items.clone() { }
   ```

## Next Steps

After mastering control flow, you're ready for:
- **Ownership and Borrowing** - Deep dive into Rust's memory model
- **Structs** - Custom data types
- **Enums** - Sum types and pattern matching
- **Error Handling** - Result and Option in depth
- **Collections** - Vectors, HashMaps, etc.

## Additional Resources

- [The Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example - Control Flow](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- [Rust by Example - Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [Rust Reference - Patterns](https://doc.rust-lang.org/reference/patterns.html)

