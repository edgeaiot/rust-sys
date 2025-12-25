fn main() {
    println!("=== Rust Functions Learning ===\n");

    // 1. Basic function call
    println!("1. Basic function:");
    greet("Alice");

    // 2. Function with return value
    println!("\n2. Function with return value:");
    let sum = add(5, 3);
    println!("   add(5, 3) = {}", sum);

    // 3. Function with explicit return
    println!("\n3. Explicit return:");
    let product = multiply(4, 7);
    println!("   multiply(4, 7) = {}", product);

    // 4. Function returning unit type (implicit)
    println!("\n4. Function returning unit:");
    print_number(42);

    // 5. Function with multiple parameters
    println!("\n5. Multiple parameters:");
    let result = calculate(10, 5, 2);
    println!("   calculate(10, 5, 2) = {}", result);

    // 6. Function with different types
    println!("\n6. Different parameter types:");
    let area = rectangle_area(5.5, 3.2);
    println!("   rectangle_area(5.5, 3.2) = {}", area);

    // 7. Early return
    println!("\n7. Early return:");
    let value = check_positive(-5);
    println!("   check_positive(-5) = {:?}", value);
    let value2 = check_positive(10);
    println!("   check_positive(10) = {:?}", value2);

    // 8. Function returning multiple values (tuple)
    println!("\n8. Returning multiple values:");
    let (quotient, remainder) = divide(17, 5);
    println!("   divide(17, 5) = ({}, {})", quotient, remainder);

    // 9. Function with reference parameters
    println!("\n9. Reference parameters:");
    let num = 100;
    print_value(&num);
    println!("   Original value unchanged: {}", num);

    // 10. Function with mutable reference
    println!("\n10. Mutable reference:");
    let mut counter = 0;
    increment(&mut counter);
    increment(&mut counter);
    println!("   Counter after two increments: {}", counter);

    // 11. Function returning String
    println!("\n11. Returning String:");
    let message = create_greeting("Bob");
    println!("   {}", message);

    // 12. Nested function calls
    println!("\n12. Nested function calls:");
    let result = add(multiply(2, 3), multiply(4, 5));
    println!("   add(multiply(2, 3), multiply(4, 5)) = {}", result);

    // 13. Function with conditional logic
    println!("\n13. Conditional logic in function:");
    println!("   max(15, 23) = {}", max(15, 23));
    println!("   max(100, 50) = {}", max(100, 50));

    // 14. Function with array parameter
    println!("\n14. Array parameter:");
    let numbers = [1, 2, 3, 4, 5];
    let sum = array_sum(&numbers);
    println!("   array_sum([1, 2, 3, 4, 5]) = {}", sum);

    // 15. Function with tuple parameter
    println!("\n15. Tuple parameter:");
    let point = (3.0, 4.0);
    let distance = distance_from_origin(point);
    println!("   distance_from_origin((3.0, 4.0)) = {:.2}", distance);

    println!("\n=== End of Functions Examples ===");
}

// 1. Basic function - no parameters, no return value
fn greet(name: &str) {
    println!("   Hello, {}!", name);
}

// 2. Function with return value (implicit return - no semicolon)
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = implicit return
}

// 3. Function with explicit return statement
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return
}

// 4. Function returning unit type () - implicit
fn print_number(n: i32) {
    println!("   Number: {}", n);
    // Implicitly returns () - unit type
}

// 5. Function with multiple parameters
fn calculate(a: i32, b: i32, c: i32) -> i32 {
    a * b + c
}

// 6. Function with floating point types
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// 7. Function with early return
fn check_positive(n: i32) -> Option<i32> {
    if n < 0 {
        return None;  // Early return
    }
    Some(n)  // Normal return
}

// 8. Function returning tuple (multiple values)
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)  // Return tuple
}

// 9. Function with immutable reference parameter
fn print_value(x: &i32) {
    println!("   Value: {}", x);
    // Cannot modify x because it's an immutable reference
}

// 10. Function with mutable reference parameter
fn increment(x: &mut i32) {
    *x += 1;  // Dereference and modify
}

// 11. Function returning String
fn create_greeting(name: &str) -> String {
    format!("   Greeting: Hello, {}! Welcome to Rust!", name)
}

// 13. Function with conditional logic
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// 14. Function with array slice parameter
fn array_sum(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}

// 15. Function with tuple parameter
fn distance_from_origin(point: (f64, f64)) -> f64 {
    let (x, y) = point;  // Destructure tuple
    (x * x + y * y).sqrt()
}

