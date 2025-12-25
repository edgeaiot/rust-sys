// Define a basic struct
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// Tuple struct
struct Point(i32, i32, i32);

// Another tuple struct
struct Color(u8, u8, u8);

// Unit struct (no fields)
struct Marker;

// Struct with different field types
struct Rectangle {
    width: f64,
    height: f64,
}

// Nested struct
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

struct Person {
    name: String,
    age: u32,
    address: Address,
}

fn main() {
    println!("=== Rust Structs Learning ===\n");

    // 1. Creating a struct instance
    println!("1. Creating a struct instance:");
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    println!("   User: {} ({})", user1.username, user1.email);

    // 2. Accessing struct fields
    println!("\n2. Accessing struct fields:");
    println!("   Username: {}", user1.username);
    println!("   Email: {}", user1.email);
    println!("   Age: {}", user1.age);
    println!("   Active: {}", user1.active);

    // 3. Mutable struct
    println!("\n3. Mutable struct:");
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: false,
    };
    println!("   Before: active = {}", user2.active);
    user2.active = true;
    println!("   After: active = {}", user2.active);

    // 4. Struct update syntax
    println!("\n4. Struct update syntax:");
    let user3 = User {
        username: String::from("charlie"),
        email: String::from("charlie@example.com"),
        ..user1  // Copy remaining fields from user1
    };
    println!("   User3 age: {} (copied from user1)", user3.age);
    println!("   User3 active: {} (copied from user1)", user3.active);

    // 5. Tuple struct
    println!("\n5. Tuple struct:");
    let origin = Point(0, 0, 0);
    println!("   Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    let point = Point(3, 4, 5);
    println!("   Point: ({}, {}, {})", point.0, point.1, point.2);

    // 6. Tuple struct with named fields (access by index)
    println!("\n6. Tuple struct - Color:");
    let red = Color(255, 0, 0);
    println!("   Red: RGB({}, {}, {})", red.0, red.1, red.2);

    // 7. Unit struct
    println!("\n7. Unit struct:");
    let _marker = Marker;
    println!("   Marker created: {} bytes", std::mem::size_of::<Marker>());

    // 8. Struct with methods (using impl)
    println!("\n8. Struct with methods:");
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("   Rectangle area: {}", rect.area());
    println!("   Rectangle perimeter: {}", rect.perimeter());
    println!("   Is square? {}", rect.is_square());

    // 9. Mutable methods
    println!("\n9. Mutable methods:");
    let mut rect2 = Rectangle {
        width: 5.0,
        height: 5.0,
    };
    println!("   Before resize: width = {}", rect2.width);
    rect2.resize(10.0, 8.0);
    println!("   After resize: width = {}, height = {}", rect2.width, rect2.height);

    // 10. Associated functions (like constructors)
    println!("\n10. Associated functions (constructors):");
    let rect3 = Rectangle::new(15.0, 10.0);
    println!("   Created rectangle: {}x{}", rect3.width, rect3.height);

    let square = Rectangle::square(7.0);
    println!("   Created square: {}x{}", square.width, square.height);

    // 11. Multiple impl blocks
    println!("\n11. Multiple impl blocks:");
    let rect4 = Rectangle {
        width: 12.0,
        height: 8.0,
    };
    println!("   Area: {}", rect4.area());
    println!("   Can fit 3x3? {}", rect4.can_fit(3.0, 3.0));

    // 12. Nested structs
    println!("\n12. Nested structs:");
    let address = Address {
        street: String::from("123 Main St"),
        city: String::from("New York"),
        zip_code: String::from("10001"),
    };
    let person = Person {
        name: String::from("David"),
        age: 35,
        address,
    };
    println!("   Person: {}", person.name);
    println!("   Address: {}, {}, {}", person.address.street, person.address.city, person.address.zip_code);

    // 13. Struct with Option fields
    println!("\n13. Struct with Option fields:");
    let user4 = UserWithOptionalEmail {
        username: String::from("eve"),
        email: Some(String::from("eve@example.com")),
        age: 28,
    };
    match user4.email {
        Some(ref email) => println!("   Email: {}", email),
        None => println!("   No email provided"),
    }

    // 14. Struct as function parameter
    println!("\n14. Struct as function parameter:");
    let user5 = User {
        username: String::from("frank"),
        email: String::from("frank@example.com"),
        age: 40,
        active: true,
    };
    display_user(&user5);

    // 15. Returning struct from function
    println!("\n15. Returning struct from function:");
    let user6 = create_user(String::from("grace"), String::from("grace@example.com"), 22);
    println!("   Created user: {}", user6.username);

    // 16. Struct with vector field
    println!("\n16. Struct with vector field:");
    let mut shopping_cart = ShoppingCart {
        items: vec![
            String::from("Apple"),
            String::from("Banana"),
        ],
        total: 0.0,
    };
    shopping_cart.add_item(String::from("Cherry"));
    println!("   Items: {:?}", shopping_cart.items);
    println!("   Total: ${:.2}", shopping_cart.total);

    // 17. Struct with lifetime (basic example)
    println!("\n17. Struct with string slice:");
    let text = String::from("Hello, World!");
    let text_slice = &text[0..5];
    let holder = TextHolder {
        text: text_slice,
    };
    println!("   Text: {}", holder.text);

    println!("\n=== End of Structs Examples ===");
}

// Struct with Option field
struct UserWithOptionalEmail {
    username: String,
    email: Option<String>,
    age: u32,
}

// Struct with vector
struct ShoppingCart {
    items: Vec<String>,
    total: f64,
}

impl ShoppingCart {
    fn add_item(&mut self, item: String) {
        self.items.push(item);
        self.total += 1.0; // Simple pricing
    }
}

// Struct with string slice (requires lifetime)
struct TextHolder<'a> {
    text: &'a str,
}

// Implementation block for Rectangle
impl Rectangle {
    // Associated function (like constructor)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Another associated function
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // Method (takes &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method with &self
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Method returning bool
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Mutable method (takes &mut self)
    fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }
}

// Another impl block for Rectangle (can have multiple)
impl Rectangle {
    fn can_fit(&self, width: f64, height: f64) -> bool {
        self.width >= width && self.height >= height
    }
}

// Function taking struct as parameter
fn display_user(user: &User) {
    println!("   User: {} ({})", user.username, user.email);
    println!("   Age: {}, Active: {}", user.age, user.active);
}

// Function returning struct
fn create_user(username: String, email: String, age: u32) -> User {
    User {
        username,
        email,
        age,
        active: true,
    }
}

