fn main() {
    println!("=== Rust Variables Learning ===\n");

    // 1. Basic variable declaration (immutable by default)
    let x = 5;
    println!("1. Immutable variable: x = {}", x);
    // x = 6; // This would cause a compile error!

    // 2. Mutable variables
    let mut y = 10;
    println!("2. Before mutation: y = {}", y);
    y = 20;
    println!("   After mutation: y = {}", y);

    // 3. Type annotations
    let integer: i32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';
    println!("3. Type annotations:");
    println!("   integer: {} (i32)", integer);
    println!("   floating: {} (f64)", floating);
    println!("   boolean: {} (bool)", boolean);
    println!("   character: {} (char)", character);

    // 4. Variable shadowing
    let shadow = 5;
    println!("4. Variable shadowing:");
    println!("   Original shadow = {}", shadow);
    let shadow = shadow + 1; // Creates a new variable with the same name
    println!("   Shadowed shadow = {}", shadow);
    let shadow = "now I'm a string!";
    println!("   Shadowed again: shadow = {}", shadow);

    // 5. Constants (must have type annotation)
    const MAX_POINTS: u32 = 100_000;
    println!("5. Constant: MAX_POINTS = {}", MAX_POINTS);

    // 6. Multiple variables
    let (a, b, c) = (1, 2, 3);
    println!("6. Multiple variables: a={}, b={}, c={}", a, b, c);

    // 7. Uninitialized variables (must be initialized before use)
    let uninitialized: i32;
    // println!("{}", uninitialized); // Error! Must initialize first
    uninitialized = 100;
    println!("7. Uninitialized (now initialized): {}", uninitialized);

    // 8. String types
    let string_literal = "Hello"; // &str (string slice)
    let owned_string = String::from("World"); // String (owned)
    println!("8. String types:");
    println!("   string_literal: {} (type: &str)", string_literal);
    println!("   owned_string: {} (type: String)", owned_string);

    // 9. Arrays
    let array = [1, 2, 3, 4, 5];
    println!("9. Array: {:?}", array);
    println!("   First element: {}", array[0]);

    // 10. Tuples
    let tuple: (i32, f64, bool) = (42, 3.14, true);
    println!("10. Tuple: {:?}", tuple);
    println!("    First element: {}", tuple.0);
    println!("    Second element: {}", tuple.1);
    println!("    Third element: {}", tuple.2);

    println!("\n=== End of Variables Examples ===");
}

