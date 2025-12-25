# Structs in Rust - Learning Guide

## Overview

This project provides a comprehensive introduction to structs in Rust. Structs are custom data types that let you name and package together multiple related values. They're fundamental to organizing and modeling data in Rust programs.

## Lecture Notes

### 1. What are Structs?

Structs are custom data types that group related data together. They're similar to classes in object-oriented languages but without inheritance.

**Key Concepts:**
- Named fields with types
- Custom data types
- Encapsulation of related data
- Foundation for building complex programs

### 2. Defining a Struct

**Basic Syntax:**
```rust
struct StructName {
    field1: Type1,
    field2: Type2,
    // ...
}
```

**Example:**
```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

**Key Points:**
- Use `PascalCase` for struct names
- Use `snake_case` for field names
- Each field must have a type
- Fields are private by default (in modules)

### 3. Creating Struct Instances

Create instances using struct literal syntax:

```rust
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};
```

**Important:**
- Must provide all fields
- Order doesn't matter
- Can use field init shorthand (see below)

### 4. Field Init Shorthand

When variable names match field names, you can use shorthand:

```rust
fn create_user(username: String, email: String) -> User {
    User {
        username,  // Instead of username: username
        email,     // Instead of email: email
        age: 30,
        active: true,
    }
}
```

### 5. Accessing Struct Fields

Use dot notation to access fields:

```rust
let user = User { /* ... */ };
println!("{}", user.username);
println!("{}", user.email);
```

**Access Rules:**
- Use `.` to access fields
- Fields follow ownership rules
- Can access multiple fields

### 6. Mutable Structs

Make the entire struct mutable to modify fields:

```rust
let mut user = User { /* ... */ };
user.age = 31;  // Can modify fields
```

**Key Points:**
- `mut` applies to the entire struct
- All fields become mutable
- Can't make individual fields mutable independently

### 7. Struct Update Syntax

Create a new instance using another instance:

```rust
let user2 = User {
    username: String::from("bob"),
    email: String::from("bob@example.com"),
    ..user1  // Copy remaining fields from user1
};
```

**Use Cases:**
- Creating similar structs
- Updating some fields
- Cloning with modifications

### 8. Tuple Structs

Tuple structs have fields without names:

```rust
struct Point(i32, i32, i32);
let origin = Point(0, 0, 0);
```

**Characteristics:**
- Access fields by index: `point.0`, `point.1`
- Useful for wrapping types
- Can have different types in each position

**When to Use:**
- When field names aren't meaningful
- Wrapping primitive types
- Creating distinct types from same underlying types

### 9. Unit Structs

Unit structs have no fields:

```rust
struct Marker;
let marker = Marker;
```

**Use Cases:**
- Marker types (compile-time only, zero size)
- Implementing traits without data
- Type-level programming

### 10. Methods with `impl`

Methods are functions associated with a struct:

```rust
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

**Key Points:**
- `&self` - immutable reference to instance
- `&mut self` - mutable reference to instance
- `self` - takes ownership (rare)
- Methods called with dot notation: `rect.area()`

### 11. Associated Functions

Associated functions don't take `self` (like static methods):

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}
```

**Characteristics:**
- Called with `::` syntax: `Rectangle::new()`
- Often used as constructors
- Can have any name (not just `new`)

### 12. Multiple `impl` Blocks

You can have multiple `impl` blocks for the same struct:

```rust
impl Rectangle {
    fn area(&self) -> f64 { /* ... */ }
}

impl Rectangle {
    fn can_fit(&self, w: f64, h: f64) -> bool { /* ... */ }
}
```

**Use Cases:**
- Organizing related methods
- Adding methods conditionally (with traits)
- Separating concerns

### 13. Nested Structs

Structs can contain other structs:

```rust
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}
```

**Access:**
```rust
person.address.street
```

### 14. Structs with Collections

Structs can contain vectors, arrays, etc.:

```rust
struct ShoppingCart {
    items: Vec<String>,
    total: f64,
}
```

### 15. Structs with Option Fields

Use `Option` for optional fields:

```rust
struct User {
    username: String,
    email: Option<String>,  // May or may not have email
}
```

**Usage:**
```rust
match user.email {
    Some(email) => println!("{}", email),
    None => println!("No email"),
}
```

### 16. Structs with Lifetimes

When structs contain references, you need lifetimes:

```rust
struct TextHolder<'a> {
    text: &'a str,
}
```

**Key Points:**
- Lifetime parameter `'a` ensures reference is valid
- Struct can't outlive the reference it holds
- Advanced topic (covered in detail later)

### 17. Structs as Function Parameters

Pass structs to functions:

```rust
fn display_user(user: &User) {
    println!("{}", user.username);
}
```

**Best Practices:**
- Use `&User` to borrow (don't take ownership)
- Use `&mut User` if you need to modify
- Only take ownership if you need it

### 18. Returning Structs from Functions

Functions can return structs:

```rust
fn create_user(name: String) -> User {
    User {
        username: name,
        // ...
    }
}
```

## Code Walkthrough

The `main.rs` file demonstrates 17 different struct concepts, from basic definitions to advanced patterns. Each example builds on previous knowledge.

## Key Learning Points

### Struct Design Principles

1. **Group Related Data**: Structs should contain logically related fields
2. **Clear Naming**: Use descriptive names for structs and fields
3. **Appropriate Types**: Choose types that match the data
4. **Encapsulation**: Methods operate on struct data

### Ownership and Structs

1. **Struct Owns Its Fields**: When struct is dropped, fields are dropped
2. **Borrowing**: Use `&Struct` to borrow, not take ownership
3. **Moving**: Moving a struct moves all its fields
4. **Copy Types**: If all fields are `Copy`, struct is `Copy`

### Methods vs Functions

1. **Methods**: Associated with instances (`&self`)
2. **Associated Functions**: Not tied to instances (like `new()`)
3. **When to Use**: Methods for operations on data, functions for utilities

## Exercises to Try

1. **Create a Book struct** with title, author, and pages
2. **Add methods** to calculate reading time based on pages
3. **Create a BankAccount struct** with balance and methods to deposit/withdraw
4. **Build a Point struct** with methods to calculate distance
5. **Create nested structs** for a library system (Book, Author, Library)
6. **Implement a Stack** using a struct with a Vec field
7. **Create a struct with Option fields** and handle None cases

## Common Mistakes

1. **Forgetting `mut`**: Can't modify fields of immutable struct
2. **Missing Fields**: Must provide all fields when creating instance
3. **Wrong Field Access**: Tuple structs use `.0`, `.1`, not names
4. **Ownership Issues**: Moving struct moves all fields
5. **Lifetime Errors**: References in structs need lifetime parameters
6. **Method vs Function**: Methods need `&self`, functions don't

## Best Practices

### Struct Definition
1. **Use PascalCase**: Struct names should be `PascalCase`
   ```rust
   // Good
   struct UserAccount { }
   
   // Avoid
   struct user_account { }
   ```

2. **Descriptive Field Names**: Fields should clearly describe their purpose
   ```rust
   // Good
   struct User {
       username: String,
       email_address: String,
   }
   
   // Avoid
   struct User {
       u: String,
       e: String,
   }
   ```

3. **Group Related Fields**: Keep logically related data together
   ```rust
   // Good: Related fields grouped
   struct Rectangle {
       width: f64,
       height: f64,
   }
   ```

### Creating Instances
1. **Use Field Init Shorthand**: When variable names match
   ```rust
   // Good: Shorthand
   fn create_user(username: String, email: String) -> User {
       User { username, email, age: 30, active: true }
   }
   
   // Verbose
   fn create_user(username: String, email: String) -> User {
       User {
           username: username,
           email: email,
           age: 30,
           active: true,
       }
   }
   ```

2. **Use Struct Update Syntax**: For similar instances
   ```rust
   // Good: Update syntax
   let user2 = User {
       username: String::from("bob"),
       ..user1
   };
   ```

### Methods
1. **Prefer Methods Over Functions**: When operating on struct data
   ```rust
   // Good: Method
   impl Rectangle {
       fn area(&self) -> f64 {
           self.width * self.height
       }
   }
   
   // Less ideal: Standalone function
   fn rectangle_area(rect: &Rectangle) -> f64 {
       rect.width * rect.height
   }
   ```

2. **Use Appropriate Self Types**: Choose `&self`, `&mut self`, or `self`
   ```rust
   // Good: &self for reading
   fn area(&self) -> f64 { }
   
   // Good: &mut self for modifying
   fn resize(&mut self, w: f64, h: f64) { }
   
   // Rare: self for consuming
   fn into_tuple(self) -> (f64, f64) { }
   ```

3. **Use Associated Functions for Constructors**: Common pattern
   ```rust
   // Good: Constructor pattern
   impl Rectangle {
       fn new(width: f64, height: f64) -> Rectangle {
           Rectangle { width, height }
       }
   }
   ```

### Passing Structs
1. **Borrow by Default**: Use `&Struct` unless you need ownership
   ```rust
   // Good: Borrow
   fn display(user: &User) { }
   
   // Only if needed: Own
   fn consume(user: User) { }
   ```

2. **Mutable Borrow When Needed**: Use `&mut Struct` for modifications
   ```rust
   // Good: Mutable borrow for modification
   fn update_age(user: &mut User, age: u32) {
       user.age = age;
   }
   ```

### Struct Design
1. **Keep Structs Focused**: Each struct should have a single responsibility
   ```rust
   // Good: Focused
   struct User { }
   struct Address { }
   
   // Avoid: Too many responsibilities
   struct UserWithAddressAndPreferences { }
   ```

2. **Use Nested Structs**: For complex, hierarchical data
   ```rust
   // Good: Nested for clarity
   struct Person {
       name: String,
       address: Address,
   }
   ```

3. **Option for Optional Fields**: Use `Option<T>` for fields that may not exist
   ```rust
   // Good: Optional field
   struct User {
       username: String,
       email: Option<String>,
   }
   ```

### Tuple Structs
1. **Use When Names Don't Matter**: For simple wrappers
   ```rust
   // Good: Tuple struct for simple wrapper
   struct Point(i32, i32);
   
   // Less ideal: Named struct for simple case
   struct Point {
       x: i32,
       y: i32,
   }
   ```

2. **Use for Type Distinction**: Same underlying type, different meaning
   ```rust
   // Good: Distinct types
   struct Meters(f64);
   struct Feet(f64);
   ```

### Unit Structs
1. **Use for Markers**: When you need a type but no data
   ```rust
   // Good: Marker type
   struct SendMarker;
   ```

### Documentation
1. **Document Public Structs**: Use doc comments
   ```rust
   /// Represents a user in the system.
   /// 
   /// Contains user identification and status information.
   struct User {
       /// The user's unique username
       username: String,
       /// The user's email address
       email: String,
   }
   ```

## Performance Considerations

1. **Structs are Stack-Allocated**: By default, structs live on the stack
2. **Fields are Stored Together**: Contiguous memory layout
3. **No Overhead**: Structs have zero runtime cost
4. **Copy vs Move**: `Copy` types are copied, others are moved

## Structs vs Other Types

**Structs vs Tuples:**
- Structs: Named fields, clearer intent
- Tuples: Anonymous, for temporary grouping

**Structs vs Enums:**
- Structs: One variant, multiple fields
- Enums: Multiple variants, each can have data

**Structs vs Classes (OOP):**
- No inheritance
- Composition over inheritance
- Traits for shared behavior (covered later)

## Next Steps

After mastering structs, you're ready for:
- **Enums** - Sum types and pattern matching
- **Traits** - Shared behavior and interfaces
- **Ownership Deep Dive** - Advanced borrowing
- **Generics** - Writing reusable code
- **Error Handling** - Result and Option in depth

## Additional Resources

- [The Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [Rust by Example - Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [Rust Reference - Structs](https://doc.rust-lang.org/reference/items/structs.html)

