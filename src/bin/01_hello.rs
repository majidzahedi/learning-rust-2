// Lesson 01: Hello, printing & comments
// Run: cargo run --bin 01_hello
//
// HINTS
// - `println!` is a *macro* (note the `!`), not a function.
// - `{}`   formats with Display (for humans).
// - `{:?}` formats with Debug (for programmers). Try `{:#?}` for pretty output.
// - You can put variables inline: println!("{name}") or use positional args: println!("{}", name)
// - Width/precision: {:>8} right-align, {:<8} left-align, {:^8} center, {:.2} two decimals
// - `print!` does not add a newline. `eprintln!` prints to stderr.
// - Comments: `//` line, `/* */` block, `///` doc comment (documents the next item).

fn main() {
    // TODO 1: Print "Hello, Rust!"
    println!("Hello, Rust!");

    // TODO 2: Make a variable holding your name and print "My name is <name>"
    //         Do it twice: once with {} + argument, once with {name} inline.
    let name = "Jev";
    println!("My name is {}", name);
    println!("My name is {name}");

    // TODO 3: Print the number 3.14159 with only 2 decimals -> 3.14
    println!("{:.2}", 3.14159);

    // TODO 4: Print a small table, names left-aligned in 10 chars, scores right-aligned in 5:
    //         Alice        90
    //         Bob          7
    println!("{:<10}{}", "Alice", 90);
    println!("{:<10}{}", "Bob", 7);

    // TODO 5: Print the tuple (1, "two", 3.0) using {:?}. What happens if you use {} instead? Why?
    // number 3.0 shows as 3 i don't know why!
    println!("({:?}, {:?}, {:?})", 1, "two", 3.0);

    // TODO 6: Print a literal curly brace like: {hello}   (hint: double them)
    println!("{{hello}}");
}
