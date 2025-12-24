
fn main() {
    println!("Hello, world!");
    println!("5 + 3 = {}", add(5, 3));
    println!("5 - 3 = {}", subtract(5, 3));
    println!("5 * 3 = {}", multiply(5, 3));
    println!("5 / 3 = {}", divide(5, 3));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}