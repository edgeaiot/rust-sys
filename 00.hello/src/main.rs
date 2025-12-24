fn add_world(s: &mut String) {
    s.push_str(" world!");
}

fn main() {
    let mut s = String::from("Hello");
    add_world(&mut s);
    println!("{}", s); // "Hello world!"
}