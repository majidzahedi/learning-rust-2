// Lesson 08: Slices
// Run: cargo run --bin 08_slices
//
// A slice is a REFERENCE to a contiguous part of a collection. It doesn't own anything.
//
// HINTS
// - String slices:  &s[0..5]   &s[..5]   &s[6..]   &s[..]          Type: &str
// - String literals ARE slices: `let x: &str = "hi";` (they point into the program binary)
// - &String automatically converts to &str ("deref coercion"). So for function parameters,
//   prefer `&str` over `&String`: it accepts both String and literals.
// - Array slices:   &a[1..4]                                        Type: &[i32]
//   &mut a[1..4] gives a mutable slice:                             Type: &mut [i32]
// - Useful slice methods: .len()  .is_empty()  .first()  .last()  .get(i)  .iter()
//   (.first() / .last() / .get() return an Option, print them with {:?})
// - String ranges are BYTE positions, not characters! Slicing in the middle of a
//   multi-byte character panics at runtime.
// - Useful &str methods: .chars()  .bytes()  .find(' ')  .split_whitespace()
//   .trim()  .to_uppercase()  .starts_with("x")  .contains("x")
// - `for (i, item) in x.iter().enumerate()` gives index + item.

fn main() {
    // TODO 1: let s = String::from("hello world");
    //         Make slices `hello` and `world` with explicit ranges [a..b], then again
    //         with the shorthand forms [..b] and [a..]. Print them. Also print &s[..].

    // TODO 2: Write `fn first_word(s: &str) -> &str` that returns the first word.
    //         Hint: loop over `s.bytes().enumerate()`, and when the byte is b' ' return &s[..i].
    //         If there's no space, the whole string is one word.
    //         Test with: "hello world", "single", ""

    // TODO 3: Why slices are safe. Predict, then test:
    //             let mut s = String::from("hello world");
    //             let word = first_word(&s);
    //             s.clear();
    //             println!("{word}");
    //         Read the error. Which borrowing rule from lesson 07 saved you?
    //         (In C, `word` would now point at garbage.)

    // TODO 4: Deref coercion. Write `fn shout(s: &str)` that prints s in uppercase.
    //         Call it with: a String (&my_string), a literal ("hi"), and a slice (&my_string[0..3]).
    //         Then change the parameter to `&String`. Which call stops compiling?

    // TODO 5: Array slices. let a = [1, 2, 3, 4, 5];
    //         Write `fn sum(nums: &[i32]) -> i32`.
    //         Call it with the whole array (&a), the middle three (&a[1..4]), and an empty slice.

    // TODO 6: Write `fn largest(nums: &[i32]) -> i32` (assume nums is not empty).
    //         What SHOULD happen if it's empty? Note your answer; lesson 11 (Option) solves it.
    //         Peek: print a.first() and (&a[..0]).first() with {:?}.

    // TODO 7: Mutable slice. Write `fn zero_out(nums: &mut [i32])` that sets every element to 0.
    //         Call it on only the LAST two elements of a mutable array. Print the whole array.

    // TODO 8: Unicode trap.
    //             let hello = "Здравствуйте";
    //         Print hello.len() and hello.chars().count(). Why are they different?
    //         Then try &hello[0..1] and read the PANIC message (runtime, not compile time!).
    //         What does &hello[0..2] give you? Then print the first 3 CHARACTERS correctly
    //         using .chars() and a counter (or .take(3) if you're curious).

    // CHALLENGE:
    //   - `fn last_word(s: &str) -> &str`
    //   - `fn count_words(s: &str) -> usize`   (hint: split_whitespace, works with a for loop)
    //   - `fn is_palindrome(s: &str) -> bool`  "racecar" -> true, "rust" -> false
    //     (bonus: make "Never odd or even" return true. Ignore spaces and case.)
}
