// Basic enum (no data)
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum with different data types
enum IpAddr {
    V4(String),
    V6(String),
}

// More specific enum variants
enum IpAddrDetailed {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum with methods
enum Status {
    Active,
    Inactive,
    Pending,
}

impl Status {
    fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }

    fn description(&self) -> &str {
        match self {
            Status::Active => "User is active",
            Status::Inactive => "User is inactive",
            Status::Pending => "User status is pending",
        }
    }
}

// Enum with associated function
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsv { h: u16, s: u8, v: u8 },
}

impl Color {
    fn new_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::Rgb(r, g, b)
    }

    fn to_string(&self) -> String {
        match self {
            Color::Red => "Red".to_string(),
            Color::Green => "Green".to_string(),
            Color::Blue => "Blue".to_string(),
            Color::Rgb(r, g, b) => format!("RGB({}, {}, {})", r, g, b),
            Color::Hsv { h, s, v } => format!("HSV({}, {}, {})", h, s, v),
        }
    }
}

// Enum for error handling pattern
enum OperationResult {
    Success(i32),
    DivisionByZero,
    NegativeNumber,
    Overflow,
}

fn divide(a: i32, b: i32) -> OperationResult {
    if b == 0 {
        OperationResult::DivisionByZero
    } else {
        OperationResult::Success(a / b)
    }
}

// Enum with Option-like pattern
enum MaybeValue<T> {
    Some(T),
    None,
}

// Enum for state machine
#[derive(Debug)]
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

fn main() {
    println!("=== Rust Enums Learning ===\n");

    // 1. Basic enum usage
    println!("1. Basic enum:");
    let direction = Direction::North;
    match direction {
        Direction::North => println!("   Heading North"),
        Direction::South => println!("   Heading South"),
        Direction::East => println!("   Heading East"),
        Direction::West => println!("   Heading West"),
    }

    // 2. Enum with data - struct-like variant
    println!("\n2. Enum with struct-like variant:");
    let msg1 = Message::Move { x: 10, y: 20 };
    match msg1 {
        Message::Move { x, y } => {
            println!("   Move to ({}, {})", x, y);
        }
        _ => {}
    }

    // 3. Enum with data - tuple variant
    println!("\n3. Enum with tuple variant:");
    let msg2 = Message::Write(String::from("Hello"));
    match msg2 {
        Message::Write(text) => {
            println!("   Write: {}", text);
        }
        _ => {}
    }

    // 4. Enum with multiple data variants
    println!("\n4. Enum with multiple data variants:");
    let msg3 = Message::ChangeColor(255, 0, 0);
    match msg3 {
        Message::Quit => println!("   Quit"),
        Message::Move { x, y } => println!("   Move to ({}, {})", x, y),
        Message::Write(s) => println!("   Write: {}", s),
        Message::ChangeColor(r, g, b) => {
            println!("   Change color to RGB({}, {}, {})", r, g, b);
        }
    }

    // 5. IP Address enum
    println!("\n5. IP Address enum:");
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    
    match home {
        IpAddr::V4(addr) => println!("   IPv4: {}", addr),
        IpAddr::V6(addr) => println!("   IPv6: {}", addr),
    }

    // 6. Detailed IP Address enum
    println!("\n6. Detailed IP Address enum:");
    let home_detailed = IpAddrDetailed::V4(127, 0, 0, 1);
    match home_detailed {
        IpAddrDetailed::V4(a, b, c, d) => {
            println!("   IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddrDetailed::V6(addr) => {
            println!("   IPv6: {}", addr);
        }
    }

    // 7. Enum methods
    println!("\n7. Enum methods:");
    let status = Status::Active;
    println!("   Is active? {}", status.is_active());
    println!("   Description: {}", status.description());

    let status2 = Status::Pending;
    println!("   Is active? {}", status2.is_active());
    println!("   Description: {}", status2.description());

    // 8. Enum with associated function
    println!("\n8. Enum with associated function:");
    let color1 = Color::new_rgb(255, 128, 0);
    println!("   Color: {}", color1.to_string());

    let color2 = Color::Hsv { h: 30, s: 100, v: 100 };
    println!("   Color: {}", color2.to_string());

    // 9. Pattern matching with enums
    println!("\n9. Pattern matching with enums:");
    let msg4 = Message::Quit;
    match msg4 {
        Message::Quit => println!("   The Quit variant has no data"),
        Message::Move { x, y } => println!("   Move to ({}, {})", x, y),
        Message::Write(text) => println!("   Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("   Color: RGB({}, {}, {})", r, g, b);
        }
    }

    // 10. If let with enum
    println!("\n10. If let with enum:");
    let msg5 = Message::Write(String::from("Hello, Rust!"));
    if let Message::Write(text) = msg5 {
        println!("   Got message: {}", text);
    }

    // 11. Option enum (built-in)
    println!("\n11. Option enum (built-in):");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(value) => println!("   Got value: {}", value),
        None => println!("   No value"),
    }

    match no_number {
        Some(value) => println!("   Got value: {}", value),
        None => println!("   No value (None)"),
    }

    // 12. Result enum (built-in)
    println!("\n12. Result enum (built-in):");
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("Something went wrong");

    match success {
        Ok(value) => println!("   Success: {}", value),
        Err(error) => println!("   Error: {}", error),
    }

    match failure {
        Ok(value) => println!("   Success: {}", value),
        Err(error) => println!("   Error: {}", error),
    }

    // 13. Custom Result-like enum
    println!("\n13. Custom Result-like enum:");
    let result1 = divide(10, 2);
    match result1 {
        OperationResult::Success(value) => println!("   Result: {}", value),
        OperationResult::DivisionByZero => println!("   Error: Division by zero"),
        OperationResult::NegativeNumber => println!("   Error: Negative number"),
        OperationResult::Overflow => println!("   Error: Overflow"),
    }

    let result2 = divide(10, 0);
    match result2 {
        OperationResult::Success(value) => println!("   Result: {}", value),
        OperationResult::DivisionByZero => println!("   Error: Division by zero"),
        OperationResult::NegativeNumber => println!("   Error: Negative number"),
        OperationResult::Overflow => println!("   Error: Overflow"),
    }

    // 14. Enum with generic type
    println!("\n14. Enum with generic type:");
    let maybe_int = MaybeValue::Some(42);
    let maybe_string = MaybeValue::Some(String::from("Hello"));
    let nothing: MaybeValue<i32> = MaybeValue::None;

    match maybe_int {
        MaybeValue::Some(value) => println!("   Got integer: {}", value),
        MaybeValue::None => println!("   No value"),
    }

    match maybe_string {
        MaybeValue::Some(value) => println!("   Got string: {}", value),
        MaybeValue::None => println!("   No value"),
    }

    match nothing {
        MaybeValue::Some(value) => println!("   Got value: {}", value),
        MaybeValue::None => println!("   No value (None)"),
    }

    // 15. State machine with enum
    println!("\n15. State machine with enum:");
    let mut light = TrafficLight::Red;
    println!("   Current: {:?}", light);
    
    light = light.next();
    println!("   After next: {:?}", light);
    
    light = light.next();
    println!("   After next: {:?}", light);

    // 16. Nested pattern matching
    println!("\n16. Nested pattern matching:");
    let msg6 = Message::Move { x: 5, y: -3 };
    match msg6 {
        Message::Move { x, y } if x > 0 && y > 0 => {
            println!("   Moving to positive quadrant: ({}, {})", x, y);
        }
        Message::Move { x, y } if x < 0 || y < 0 => {
            println!("   Moving to negative area: ({}, {})", x, y);
        }
        Message::Move { x, y } => {
            println!("   Moving to: ({}, {})", x, y);
        }
        _ => {}
    }

    // 17. Match with multiple patterns
    println!("\n17. Match with multiple patterns:");
    let direction2 = Direction::North;
    match direction2 {
        Direction::North | Direction::South => {
            println!("   Moving vertically");
        }
        Direction::East | Direction::West => {
            println!("   Moving horizontally");
        }
    }

    // 18. Using Option methods
    println!("\n18. Using Option methods:");
    let some_value = Some(5);
    println!("   is_some: {}", some_value.is_some());
    println!("   is_none: {}", some_value.is_none());
    
    if let Some(value) = some_value {
        println!("   Unwrapped value: {}", value);
    }

    let none_value: Option<i32> = None;
    let default = none_value.unwrap_or(0);
    println!("   Default value: {}", default);

    println!("\n=== End of Enums Examples ===");
}

