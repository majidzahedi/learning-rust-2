// Solution: Lesson 02
// Run: cargo run --example 02_variables

// TODO 1: name says exactly what it is, value is correct (60 * 60).
const SECONDS_PER_HOUR: u32 = 60 * 60;

fn main() {
    println!("seconds per hour: {SECONDS_PER_HOUR}");

    // TODO 2
    // let x = 5;
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // TODO 3: shadowing can change the type...
    let spaces = "   ";
    println!("spaces = {spaces:?}");
    let spaces = spaces.len();
    println!("spaces = {spaces}");
    // ...but `mut` cannot:
    // let mut spaces2 = "   ";
    // spaces2 = spaces2.len(); // error[E0308]: mismatched types, expected `&str`, found `usize`

    // TODO 4: inner y = 10, outer y = 1
    let y = 1;
    {
        let y = y * 10;
        println!("inner y = {y}");
    }
    println!("outer y = {y}");

    // TODO 5: deferred initialization
    // (clippy prefers `let z = 10;` directly; we split it on purpose to learn the rule)
    #[allow(clippy::needless_late_init)]
    let z: i32;
    // println!("{z}"); // error[E0381]: used binding `z` isn't initialized
    z = 10; // assigned exactly once, so no `mut` needed
    println!("z = {z}");
}
