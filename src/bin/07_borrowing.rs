// Lesson 07: References & borrowing
// Run: cargo run --bin 07_borrowing
//
// Borrowing = using a value WITHOUT taking ownership of it.
//
// HINTS
// - `&s`      creates a shared (read-only) reference.       Type: &String
// - `&mut s`  creates a mutable (exclusive) reference.      Type: &mut String
//              (the variable itself must also be `let mut s`)
// - THE BORROWING RULES, at any given moment you can have EITHER:
//     * any number of `&` references, OR
//     * exactly ONE `&mut` reference
//   ...but never both. ("many readers OR one writer")
// - References must always be valid: no dangling references, ever.
// - A borrow lasts until its LAST USE, not until the end of the scope
//   (this is called Non-Lexical Lifetimes, "NLL").
// - `*r` dereferences: gets at the value behind the reference. Needed for e.g. `*r += 1`.
//   Method calls auto-dereference, so `r.len()` works without `*`.
// - In a function signature: `fn f(s: &String)` borrows, `fn f(s: String)` takes ownership.

fn main() {
    // TODO 1: Write `fn calculate_length(s: &String) -> usize`.
    //         Call it with `&s`, then print BOTH s and the length. s is still usable. Why?

    // TODO 2: Inside a function taking `&String`, try `s.push_str("!")`. Read the error (E0596).
    //         Fix it: write `fn add_exclamation(s: &mut String)` and call it with `&mut s`.

    // TODO 3: Two mutable borrows at once:
    //             let mut s = String::from("hi");
    //             let r1 = &mut s;
    //             let r2 = &mut s;
    //             println!("{r1} {r2}");
    //         Read the error (E0499). Why would two writers be dangerous?

    // TODO 4: Mixing & and &mut:
    //             let mut s = String::from("hi");
    //             let r1 = &s;
    //             let r2 = &s;
    //             let r3 = &mut s;
    //             println!("{r1} {r2} {r3}");
    //         Read the error (E0502).
    //         Now MOVE the line `println!("{r1} {r2}");` to BEFORE `let r3`, and print only r3
    //         at the end. It compiles! Explain why using the NLL hint above.

    // TODO 5: Dereferencing.
    //             let mut n = 5;
    //             let r = &mut n;
    //             // add 1 to n THROUGH r
    //         Print n afterwards. What happens if you write `r += 1` without `*`?

    // TODO 6: Dangling reference. Write this function and read the error:
    //             fn dangle() -> &String {
    //                 let s = String::from("hi");
    //                 &s
    //             }
    //         Why is it impossible? (Hint: when is `s` dropped?) Fix it by returning a String.
    //         (The error mentions "lifetime". That's lesson 17, don't worry about it yet.)

    // TODO 7: Write `fn count_char(s: &String, c: char) -> usize` that counts how many times
    //         c appears in s. Hint: `for ch in s.chars() { ... }`
    //         Test: count_char(&String::from("banana"), 'a') == 3

    // TODO 8: Write `fn double_all(nums: &mut [i32; 5])` that doubles every element in place.
    //         Hint: `for x in nums { ... }` gives you `&mut i32` items, so you need `*x`.
    //         Print the array before and after.

    // TODO 9: Redo lesson 06 TODO 7 (the loop) WITHOUT clone: make the function borrow instead.

    // CHALLENGE 1: Write `fn append_twice(target: &mut String, extra: &String)` that appends
    //              `extra` to `target` two times. Use it normally, then try:
    //                  append_twice(&mut s, &s);
    //              Why does Rust reject this? What could go wrong if it were allowed?
    //              (Think: what if appending makes the String reallocate its buffer?)
    //
    // CHALLENGE 2: Try `fn longer(a: &String, b: &String) -> &String`.
    //              You'll get an error about a "missing lifetime specifier". Read the help text.
    //              Remember this one! We'll solve it properly in lesson 17.
}
