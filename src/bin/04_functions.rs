// Lesson 04: Functions
// Run: cargo run --bin 04_functions
//
// HINTS
// - fn name(param: Type, other: Type) -> ReturnType { ... }
// - Parameter types are ALWAYS required. Return type required unless it returns nothing ( () ).
// - Statements end with `;` and produce no value. Expressions produce a value.
// - The LAST expression in a block (WITHOUT `;`) is its value, i.e. the implicit return.
//     fn five() -> i32 { 5 }     // ok
//     fn five() -> i32 { 5; }    // error! the `;` turns it into a statement -> returns ()
// - `return x;` exists for early returns.
// - Blocks are expressions too: let y = { let a = 3; a + 1 };   // y = 4
// - Naming convention: snake_case.

// TODO 1: Write `fn greet(name: &str)` that prints "Hello, <name>!".
//         (Don't worry about &str yet, it's a "string slice", we'll learn it in lesson 08.)
fn greet(name: &str) {
    println!("Hello, {name}");
}

// TODO 2: Write `fn add(a: i32, b: i32) -> i32` using an implicit return.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// TODO 3: Write `fn is_even(n: i32) -> bool`.
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// TODO 4: Write `fn celsius_to_fahrenheit(c: f64) -> f64`   (F = C * 9/5 + 32)
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9f64 / 5f64) + 32f64
}

// TODO 5: Write `fn min_max(a: i32, b: i32) -> (i32, i32)` returning a tuple (smaller, bigger).
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        return (a, b);
    } else {
        return (b, a);
    }
}

// TODO 6: Write `fn abs_value(n: i32) -> i32` that uses an EARLY `return` for the negative case.
fn abs_value(n: i32) -> i32 {
    if n < 0 {
        return n * -1;
    }
    n
}

fn main() {
    // Call each function above and print the results.
    greet("Majid");

    println!("add function for 10+15: {}", add(10, 15));
    println!("is_even function for 23: {}", is_even(23));
    println!(
        "celsius_to_fahrenheit function for 24: {}",
        celsius_to_fahrenheit(24f64)
    );

    println!("min_max function for 23,32: {:?}", min_max(23, 32));
    println!("abs_value function for -13: {:?}", abs_value(-13));

    // TODO 7: Add a `;` after the last expression in `add` and read the error message.
    //         Notice how the compiler suggests the fix. Remove it again.

    // TODO 8: Use a block expression to compute a value:
    //         let result = { ... };  then print it.
    //
    let result = {
        let x = 10;
        x + 10
    };

    println!("result is: {result}")
}
