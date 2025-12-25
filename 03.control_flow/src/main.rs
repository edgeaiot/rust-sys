fn main() {
    println!("=== Rust Control Flow Learning ===\n");

    // 1. Basic if statement
    println!("1. Basic if statement:");
    let number = 7;
    if number < 5 {
        println!("   {} is less than 5", number);
    } else {
        println!("   {} is not less than 5", number);
    }

    // 2. if-else if-else chain
    println!("\n2. if-else if-else chain:");
    let score = 85;
    if score >= 90 {
        println!("   Grade: A");
    } else if score >= 80 {
        println!("   Grade: B");
    } else if score >= 70 {
        println!("   Grade: C");
    } else {
        println!("   Grade: F");
    }

    // 3. if as expression (returns value)
    println!("\n3. if as expression:");
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("   Result: {}", result);

    // 4. Multiple conditions with && and ||
    println!("\n4. Multiple conditions:");
    let age = 25;
    let has_license = true;
    if age >= 18 && has_license {
        println!("   Can drive!");
    }
    if age < 18 || !has_license {
        println!("   Cannot drive");
    }

    // 5. loop (infinite loop)
    println!("\n5. loop (infinite loop - breaking after 3 iterations):");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 3 {
            break;
        }
        print!("   {}", counter);
    }
    println!();

    // 6. loop with return value
    println!("\n6. loop with return value:");
    let mut counter2 = 0;
    let result = loop {
        counter2 += 1;
        if counter2 == 5 {
            break counter2 * 2; // Return value from loop
        }
    };
    println!("   Loop returned: {}", result);

    // 7. while loop
    println!("\n7. while loop:");
    let mut number = 3;
    while number != 0 {
        print!("   {}", number);
        number -= 1;
    }
    println!("\n   Liftoff!");

    // 8. for loop with range
    println!("\n8. for loop with range:");
    print!("   ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // 9. for loop with array
    println!("\n9. for loop with array:");
    let arr = [10, 20, 30, 40, 50];
    print!("   ");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();

    // 10. for loop with enumerate
    println!("\n10. for loop with enumerate:");
    let items = ["apple", "banana", "cherry"];
    for (index, item) in items.iter().enumerate() {
        println!("   {}: {}", index, item);
    }

    // 11. match expression (basic)
    println!("\n11. match expression (basic):");
    let number = 3;
    match number {
        1 => println!("   One"),
        2 => println!("   Two"),
        3 => println!("   Three"),
        _ => println!("   Something else"),
    }

    // 12. match with multiple patterns
    println!("\n12. match with multiple patterns:");
    let number = 2;
    match number {
        1 | 2 | 3 => println!("   Small number"),
        4 | 5 | 6 => println!("   Medium number"),
        _ => println!("   Large number"),
    }

    // 13. match with ranges
    println!("\n13. match with ranges:");
    let age = 25;
    match age {
        0..=12 => println!("   Child"),
        13..=19 => println!("   Teenager"),
        20..=64 => println!("   Adult"),
        _ => println!("   Senior"),
    }

    // 14. match with destructuring
    println!("\n14. match with destructuring:");
    let point = (3, 5);
    match point {
        (0, 0) => println!("   Origin"),
        (0, y) => println!("   On y-axis at y={}", y),
        (x, 0) => println!("   On x-axis at x={}", x),
        (x, y) => println!("   Point at ({}, {})", x, y),
    }

    // 15. match with guards
    println!("\n15. match with guards:");
    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("   Less than 5: {}", x),
        Some(x) if x >= 5 => println!("   Greater or equal to 5: {}", x),
        Some(_) => println!("   Some other value"),
        None => println!("   None"),
    }

    // 16. match with Option
    println!("\n16. match with Option:");
    let some_value = Some(42);
    match some_value {
        Some(value) => println!("   Got value: {}", value),
        None => println!("   No value"),
    }

    // 17. match with Result
    println!("\n17. match with Result:");
    let result: Result<i32, &str> = Ok(200);
    match result {
        Ok(value) => println!("   Success: {}", value),
        Err(error) => println!("   Error: {}", error),
    }

    // 18. if let (simplified match)
    println!("\n18. if let (simplified match):");
    let some_number = Some(7);
    if let Some(value) = some_number {
        println!("   Got value: {}", value);
    } else {
        println!("   No value");
    }

    // 19. while let
    println!("\n19. while let:");
    let mut stack = vec![1, 2, 3];
    print!("   Popped: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // 20. Nested control flow
    println!("\n20. Nested control flow:");
    for i in 1..=10 {
        if i % 2 == 0 {
            match i {
                2 | 4 | 6 | 8 => println!("   {} is a small even number", i),
                10 => println!("   {} is ten!", i),
                _ => println!("   {} is even", i),
            }
        }
    }

    // 21. break and continue in loops
    println!("\n21. break and continue:");
    print!("   ");
    for i in 1..=10 {
        if i == 3 {
            continue; // Skip 3
        }
        if i == 8 {
            break; // Stop at 8
        }
        print!("{} ", i);
    }
    println!();

    // 22. Loop labels
    println!("\n22. Loop labels:");
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("   Breaking outer loop at i={}, j={}", i, j);
                break 'outer;
            }
            print!("   ({}, {}) ", i, j);
        }
    }
    println!();

    // 23. Match with binding
    println!("\n23. Match with binding:");
    let point = (0, 5);
    match point {
        (0, y) => println!("   On y-axis at {}", y),
        (x, y) => println!("   Point ({}, {})", x, y),
    }

    // 24. Match with @ binding
    println!("\n24. Match with @ binding:");
    let age = 25;
    match age {
        n @ 0..=12 => println!("   Child (age {})", n),
        n @ 13..=19 => println!("   Teenager (age {})", n),
        n => println!("   Adult (age {})", n),
    }

    println!("\n=== End of Control Flow Examples ===");
}

