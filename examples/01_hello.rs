// Solution: Lesson 01
// Run: cargo run --example 01_hello

fn main() {
    // TODO 1
    println!("Hello, Rust!");

    // TODO 2
    let name = "Majid";
    println!("My name is {}", name);
    println!("My name is {name}");

    // TODO 3
    // Clippy rejects 3.14159 because it looks like PI: use the real constant.
    // (Same for 2.71828 -> std::f64::consts::E. Clippy knows its math!)
    let price = 9.87654;
    println!("{:.2}", price);
    println!("{:.2}", std::f64::consts::PI);

    // TODO 4: both columns get a width
    println!("{:<10}{:>5}", "Alice", 90);
    println!("{:<10}{:>5}", "Bob", 7);

    // TODO 5: print the tuple as ONE value
    let t = (1, "two", 3.0);
    println!("{:?}", t); // (1, "two", 3.0)
    println!("{:#?}", t); // pretty: one field per line
    // println!("{}", t); // error[E0277]: `({integer}, &str, {float})` doesn't implement `std::fmt::Display`
    // Display ({}) is for end users; Rust won't guess how a tuple should look to a user.
    // Debug ({:?}) is for programmers, and tuples implement it.

    // Display vs Debug on a float:
    println!("{} vs {:?}", 3.0, 3.0); // 3 vs 3.0

    // TODO 6
    println!("{{hello}}");
}
