# Enums in Rust - Learning Guide

## Overview

This project provides a comprehensive introduction to enums (enumerations) in Rust. Enums are a powerful feature that allow you to define a type by enumerating its possible variants. They're essential for modeling data that can be one of several different types, making Rust's type system extremely expressive.

## Lecture Notes

### 1. What are Enums?

Enums (enumerations) define a type by enumerating its possible variants. Unlike enums in other languages, Rust enums can carry data, making them extremely powerful.

**Key Concepts:**
- Define a type with multiple variants
- Each variant can have different associated data
- Exhaustive pattern matching
- Foundation for error handling (Option, Result)

### 2. Basic Enum Definition

**Simple Enum (No Data):**
```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

**Characteristics:**
- Use `PascalCase` for enum names
- Use `PascalCase` for variant names
- Variants are namespaced: `Direction::North`
- Can have methods and associated functions

### 3. Enums with Data

Enums can carry data in three ways:

**Tuple Variants:**
```rust
enum Message {
    Write(String),
    Move(i32, i32),
}
```

**Struct-like Variants:**
```rust
enum Message {
    Move { x: i32, y: i32 },
    ChangeColor { r: u8, g: u8, b: u8 },
}
```

**Unit Variants (No Data):**
```rust
enum Message {
    Quit,
}
```

### 4. Pattern Matching with Enums

`match` is the primary way to work with enums:

```rust
match message {
    Message::Quit => println!("Quit"),
    Message::Write(text) => println!("{}", text),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
}
```

**Key Points:**
- Must handle all variants (exhaustive)
- Can destructure data from variants
- Compiler ensures all cases are covered

### 5. The Option Enum

`Option<T>` is Rust's way of handling nullable values:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Usage:**
```rust
let some_number = Some(5);
let no_number: Option<i32> = None;

match some_number {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}
```

**Why Option?**
- No null pointer exceptions
- Forces explicit handling of "no value" case
- Type-safe alternative to null

### 6. The Result Enum

`Result<T, E>` is Rust's way of handling errors:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Usage:**
```rust
let success: Result<i32, &str> = Ok(42);
let failure: Result<i32, &str> = Err("Error");

match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

**Why Result?**
- Explicit error handling
- No exceptions
- Forces error handling at compile time

### 7. Methods on Enums

Enums can have methods just like structs:

```rust
impl Status {
    fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }
}
```

**Key Points:**
- Use `impl` blocks for methods
- Methods can take `&self`, `&mut self`, or `self`
- Can have associated functions (no `self`)

### 8. Associated Functions

Enums can have associated functions (like constructors):

```rust
impl Color {
    fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb(r, g, b)
    }
}
```

**Usage:**
```rust
let color = Color::new_rgb(255, 0, 0);
```

### 9. If Let Syntax

`if let` is a concise way to match a single pattern:

```rust
if let Some(value) = option {
    println!("{}", value);
}
```

**When to Use:**
- Only care about one variant
- Simpler than full `match`
- Good for unwrapping Option/Result

### 10. Enums with Generics

Enums can be generic:

```rust
enum MaybeValue<T> {
    Some(T),
    None,
}
```

**Usage:**
```rust
let maybe_int = MaybeValue::Some(42);
let maybe_string = MaybeValue::Some(String::from("Hello"));
```

### 11. State Machines

Enums are perfect for state machines:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}
```

### 12. Nested Pattern Matching

You can match on enum data with conditions:

```rust
match message {
    Message::Move { x, y } if x > 0 && y > 0 => {
        println!("Positive quadrant");
    }
    Message::Move { x, y } => {
        println!("Other area");
    }
    _ => {}
}
```

## Code Walkthrough

The `main.rs` file demonstrates 18 different enum concepts, from basic definitions to advanced patterns. Each example builds on previous knowledge.

## Key Learning Points

### Enum Design Principles

1. **Model Alternatives**: Enums represent "one of" relationships
2. **Exhaustive Matching**: Compiler ensures all variants are handled
3. **Type Safety**: No invalid states possible
4. **Data Carrying**: Variants can carry different types of data

### When to Use Enums

1. **Multiple Variants**: When data can be one of several types
2. **State Machines**: Representing states and transitions
3. **Error Handling**: Result and Option patterns
4. **Tree Structures**: Recursive data structures
5. **Command Patterns**: Different command types with different data

### Enums vs Structs

**Enums:**
- "One of" relationship
- Multiple variants
- Each variant can have different data
- Pattern matching required

**Structs:**
- "All of" relationship
- Single variant
- Fixed set of fields
- Direct field access

## Exercises to Try

1. **Create a Shape enum** with Circle, Rectangle, and Triangle variants
2. **Build a Calculator enum** with Add, Subtract, Multiply, Divide operations
3. **Implement a FileType enum** with different file types and metadata
4. **Create a State enum** for a game (Menu, Playing, Paused, GameOver)
5. **Build a NetworkEvent enum** for different network events
6. **Implement a Parser enum** for parsing different token types
7. **Create a Command enum** for a CLI application

## Common Mistakes

1. **Forgetting Match Cases**: Match must be exhaustive (use `_` for catch-all)
2. **Wrong Variant Namespace**: Use `EnumName::Variant`, not just `Variant`
3. **Pattern Matching Errors**: Destructure correctly based on variant type
4. **Ownership Issues**: Moving enum moves its data
5. **Missing Data**: Forgetting to handle data in variants

## Best Practices

### Enum Definition
1. **Use Descriptive Names**: Enum and variant names should be clear
   ```rust
   // Good
   enum HttpMethod {
       Get,
       Post,
       Put,
       Delete,
   }
   
   // Avoid
   enum Method {
       G,
       P,
       U,
       D,
   }
   ```

2. **Group Related Variants**: Variants should represent alternatives
   ```rust
   // Good: Related alternatives
   enum Status {
       Active,
       Inactive,
       Pending,
   }
   ```

3. **Use Appropriate Data Types**: Choose the right data for each variant
   ```rust
   // Good: Appropriate data types
   enum Message {
       Quit,
       Move { x: i32, y: i32 },
       Write(String),
   }
   ```

### Pattern Matching
1. **Always Exhaustive**: Handle all variants or use `_`
   ```rust
   // Good: Exhaustive
   match direction {
       Direction::North => {},
       Direction::South => {},
       Direction::East => {},
       Direction::West => {},
   }
   
   // Also good: With catch-all
   match direction {
       Direction::North => {},
       _ => {},
   }
   ```

2. **Order Matters**: More specific patterns first
   ```rust
   // Good: Specific first
   match message {
       Message::Move { x, y } if x > 0 => {},
       Message::Move { x, y } => {},
       _ => {},
   }
   ```

3. **Destructure Properly**: Match the variant's data structure
   ```rust
   // Good: Correct destructuring
   match message {
       Message::Move { x, y } => println!("({}, {})", x, y),
       Message::Write(text) => println!("{}", text),
       _ => {},
   }
   ```

### Option and Result
1. **Always Handle None/Err**: Don't ignore error cases
   ```rust
   // Good: Handle both cases
   match option {
       Some(value) => process(value),
       None => handle_missing(),
   }
   
   // Avoid: Ignoring None
   if let Some(value) = option {
       process(value);
       // What if None?
   }
   ```

2. **Use Option Methods**: Leverage built-in methods
   ```rust
   // Good: Use methods
   let value = option.unwrap_or(0);
   let value = option.map(|x| x * 2);
   
   // Less ideal: Manual matching
   let value = match option {
       Some(x) => x * 2,
       None => 0,
   };
   ```

3. **Propagate Errors**: Use `?` operator for error propagation
   ```rust
   // Good: Propagate errors
   fn function() -> Result<i32, Error> {
       let value = may_fail()?;
       Ok(value * 2)
   }
   ```

### Methods on Enums
1. **Add Methods for Common Operations**: Encapsulate enum logic
   ```rust
   // Good: Method for common operation
   impl Status {
       fn is_active(&self) -> bool {
           matches!(self, Status::Active)
       }
   }
   ```

2. **Use Associated Functions for Constructors**: Common pattern
   ```rust
   // Good: Constructor pattern
   impl Color {
       fn rgb(r: u8, g: u8, b: u8) -> Color {
           Color::Rgb(r, g, b)
       }
   }
   ```

### If Let vs Match
1. **Use If Let for Single Cases**: When you only care about one variant
   ```rust
   // Good: Single case
   if let Some(value) = option {
       process(value);
   }
   
   // Overkill: Full match for one case
   match option {
       Some(value) => process(value),
       None => {},
   }
   ```

2. **Use Match for Multiple Cases**: When you need to handle multiple variants
   ```rust
   // Good: Multiple cases
   match status {
       Status::Active => {},
       Status::Inactive => {},
       Status::Pending => {},
   }
   ```

### State Machines
1. **Use Enums for States**: Perfect for state machines
   ```rust
   // Good: State machine
   enum GameState {
       Menu,
       Playing,
       Paused,
       GameOver,
   }
   ```

2. **Implement Transitions**: Methods for state changes
   ```rust
   // Good: State transitions
   impl GameState {
       fn start(&self) -> GameState {
           match self {
               GameState::Menu => GameState::Playing,
               _ => *self,
           }
       }
   }
   ```

### Error Handling
1. **Use Result for Fallible Operations**: Operations that can fail
   ```rust
   // Good: Result for errors
   fn divide(a: f64, b: f64) -> Result<f64, String> {
       if b == 0.0 {
           Err("Division by zero".to_string())
       } else {
           Ok(a / b)
       }
   }
   ```

2. **Use Option for Nullable Values**: Values that might not exist
   ```rust
   // Good: Option for nullable
   fn find_user(id: u32) -> Option<User> {
       // Returns Some(user) or None
   }
   ```

3. **Custom Error Types**: Create enums for different error types
   ```rust
   // Good: Custom error enum
   enum ParseError {
       InvalidFormat,
       MissingField,
       OutOfRange,
   }
   ```

## Performance Considerations

1. **Enums are Efficient**: No runtime overhead for variants
2. **Size is Largest Variant**: Enum size = largest variant + discriminant
3. **Pattern Matching is Fast**: Compiler optimizes match expressions
4. **No Heap Allocation**: Unless variant contains heap-allocated data

## Advanced Topics (Future Lessons)

- **Trait Objects**: Using enums with trait objects
- **Recursive Enums**: Enums that contain themselves (Box needed)
- **Derive Macros**: Automatically implementing traits (`#[derive(Debug)]`)
- **Match Guards**: Additional conditions in match arms
- **Exhaustive Patterns**: Ensuring all cases are handled

## Next Steps

After mastering enums, you're ready for:
- **Pattern Matching Deep Dive** - Advanced pattern matching techniques
- **Error Handling** - Result and Option in depth
- **Traits** - Shared behavior and polymorphism
- **Generics** - Writing reusable enum code
- **Ownership with Enums** - How ownership works with enum variants

## Additional Resources

- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Rust by Example - Option](https://doc.rust-lang.org/rust-by-example/std/option.html)
- [Rust by Example - Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Rust Reference - Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html)

