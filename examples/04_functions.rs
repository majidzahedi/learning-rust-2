// Solution: Lesson 04
// Run: cargo run --example 04_functions

// TODO 1
fn greet(name: &str) {
    println!("Hello, {name}!");
}

// TODO 2
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// TODO 3
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// TODO 4: 9 / 5 is INTEGER division = 1. Use float literals so it's 1.8.
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// TODO 5: (smaller, bigger). `if` is an expression, so no `return` needed.
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

// TODO 6
fn abs_value(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}

fn main() {
    greet("Majid");
    println!("add(10, 15)                = {}", add(10, 15));
    println!("is_even(23)                = {}", is_even(23));
    println!(
        "celsius_to_fahrenheit(24)  = {}",
        celsius_to_fahrenheit(24.0)
    ); // 75.2
    println!("min_max(32, 23)            = {:?}", min_max(32, 23)); // (23, 32)
    println!("abs_value(-13)             = {}", abs_value(-13));

    // TODO 7: with `a + b;` you get:
    // error[E0308]: mismatched types ... expected `i32`, found `()`
    // help: remove this semicolon to return this value

    // TODO 8
    let result = {
        let x = 10;
        x + 10 // no `;` -> this is the block's value
    };
    println!("result = {result}");
}
