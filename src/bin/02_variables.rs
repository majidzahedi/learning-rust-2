// Lesson 02: Variables, mutability, shadowing, constants
// Run: cargo run --bin 02_variables
//
// HINTS
// - `let x = 5;` is immutable by default. Assigning again is a compile error.
// - `let mut x = 5;` lets you change it.
// - Shadowing: `let x = x + 1;` creates a NEW variable with the same name.
//   It can even change type! (mut cannot change type.)
// - `const MAX: u32 = 100;` needs a type annotation and must be known at compile time.
//   Convention: SCREAMING_SNAKE_CASE.
// - Unused variables give warnings. Prefix with `_` to silence: `let _x = 1;`
// - Blocks `{ ... }` create a new scope. Shadowing inside a block ends at the `}`.

// TODO 1: Declare a const SECONDS_PER_HOUR here (outside main) and print it in main.
const SECOND_PER_HOUR: u32 = 60;

fn main() {
    // TODO 2: Create an immutable `x = 5`, then try `x = 6;`.
    //         Read the compiler error (note the error code, try `rustc --explain <code>`).
    //         Then fix it with `mut`.
    let x = 5;
    println!("{SECOND_PER_HOUR}");
    println!("{x}");

    // TODO 3: Shadowing. Start with `let spaces = "   ";` (a string),
    //         then shadow it with its length: spaces.len()
    //         Print both. Then try doing the same with `let mut` and see why it fails.
    let spaces = "   ";
    println!("{}",spaces);
    let spaces = spaces.len();
    println!("{}",spaces);

    // TODO 4: Scope. Predict the output BEFORE running:
    let y = 1;
    {
        let y = y * 10;
        println!("inner y = {y}");
    }
    println!("outer y = {y}");

    // TODO 5: Declare `let z: i32;` without a value. Try to print it. What does the compiler say?
    //         Then assign it later (once) and print. Does it need `mut`?
    let z:i32 = 10;
    println!("{z}");
}
