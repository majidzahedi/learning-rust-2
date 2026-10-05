// Lesson 06: Ownership
// Run: cargo run --bin 06_ownership
//
// This is THE lesson that makes Rust different. Go slow and read every error.
//
// THE 3 RULES
//   1. Each value has exactly ONE owner (a variable).
//   2. When the owner goes out of scope (`}`), the value is dropped (memory freed).
//   3. Ownership can be MOVED to another variable or function. The old one becomes unusable.
//
// HINTS
// - Stack vs heap: i32, f64, bool, char have a fixed size and live on the stack.
//   `String` has a growable buffer on the heap; the variable holds (pointer, len, capacity).
// - `let s = "hi";`               is a &str literal: fixed text baked into the program.
//   `let s = String::from("hi");` is an owned, growable String on the heap.
//   Also: "hi".to_string(), String::new()
// - `let b = a;` with a String MOVES it: `a` is now invalid (no double free possible!).
// - `let b = a.clone();` makes a deep copy of the heap data (explicit and potentially costly).
// - Types that implement `Copy` are copied instead of moved: all integers, floats, bool, char,
//   and tuples/arrays made ONLY of Copy types.
// - Passing a value to a function moves (or copies) it, exactly like `let` does.
//   Returning a value from a function moves ownership OUT to the caller.
// - println! does NOT take ownership (it borrows, see lesson 07).
// - Tip: comment out the "break it on purpose" lines after you've read the error,
//   so the rest of the file still runs.

fn main() {
    // TODO 1: Create a String "hello" with String::from, then append ", world" with push_str.
    //         Print it. Why does the variable need `mut`?
    let mut hello = String::from("hello");
    hello.push_str(", world");
    // push_str changes hello thats why it needs mut
    println!("{hello}");

    // TODO 2: MOVE. Write:
    let s1 = String::from("hi");
    let s2 = s1.clone();
    println!("{s1}");
    //         Read the error (E0382). Which line does the compiler say the move happened on?
    //         Fix it TWO ways: (a) print s2 instead, (b) use .clone().

    // TODO 3: COPY. Do the exact same thing as TODO 2 with `let n1 = 5;`. Why does it work?
    //         Then predict and test: which of these are Copy (can be used after `let b = a;`)?
    //             (i32, bool)        [i32; 3]        (i32, String)        &str
    let n1 = 5;
    let n2 = n1;
    println!("{n2}")
    // because n1 is i32 and of known size so it takes stack
    // i think only (i32, bool)

    // TODO 4: Functions MOVE too.
    //         Write `fn take(s: String)` that prints s. Call it with a String, then try to
    //         print that String again afterwards. Read the error.
    //         Write `fn take_number(n: i32)` and do the same with an i32. What's different?

    // TODO 5: Give ownership BACK.
    //         Write `fn add_world(s: String) -> String` that appends " world" and returns it.
    //         Call it like:  let s = add_world(s);   (shadowing again!)
    //         Hint: the parameter needs to be mutable to push_str: `fn add_world(mut s: String)`

    // TODO 6: SCOPE & DROP. Create a String inside a `{ }` block and try to print it after
    //         the block. What's the error? At which `}` is the String's memory freed?

    // TODO 7: Moves in a loop. Predict, then test:
    //             let msg = String::from("ping");
    //             for _ in 0..3 {
    //                 take(msg);
    //             }
    //         Read the error ("value moved here, in previous iteration of loop").
    //         Fix it with .clone(). (Lesson 07 shows the better fix.)

    // TODO 8: Predict which `println!` lines compile BEFORE running. Then check.
    //             let a = String::from("a");
    //             let b = a.clone();
    //             let c = b;
    //             let d = 10;
    //             let e = d;
    //             println!("{a}");
    //             println!("{b}");
    //             println!("{c}");
    //             println!("{d} {e}");

    // CHALLENGE: Write `fn longer(a: String, b: String) -> String` that returns the longer one.
    //            Call it, then try to use BOTH originals afterwards. What happened to the
    //            shorter one? (It was dropped inside the function!)
    //            Now write `fn lengths(a: String, b: String) -> (String, String, usize, usize)`
    //            that returns both strings back plus their lengths, so the caller keeps them.
    //            Annoying, right? Passing ownership around just to read something.
    //            That's exactly the problem lesson 07 (borrowing) solves.
}
