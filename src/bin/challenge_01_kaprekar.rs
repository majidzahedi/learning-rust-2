// Challenge 01: Kaprekar's Mysterious Number
// Run: cargo run --bin challenge_01_kaprekar
//
// Uses ONLY Part 1: integers, arrays, tuples, functions, loops, if.
// No String, no Vec, no ownership needed.
//
// ============================================================================
// STEP 0: RESEARCH FIRST (before writing any code)
// ============================================================================
// Search for: "Kaprekar's routine", "6174", "Kaprekar constant".
// Answer these questions in a comment block right here before you start coding:
//
//   Q1. What is the routine? Describe one step in your own words.
//       if you take any 4-digit number in any base and sort its digits asc or desc then find the diffrence
//       and repeat it you will come to 6174 within 7 iteration
//   Q2. Which 4-digit numbers DON'T work, and why?
//       numbers that are called rep digits like 1111 or 2222 otherwise the first iteration returns 0
//   Q3. What happens when the result has fewer than 4 digits (e.g. 999)?
//       How should you treat it? (This detail breaks most first attempts!)
//       noting just sort like there is a zero for the placeholder with that number of digits
//   Q4. What is the maximum number of steps any 4-digit number needs to reach 6174?
//       its 7
//   Q5. What's the 3-digit equivalent of 6174?
//
// Sanity check for your understanding: 3524 reaches 6174 in exactly 3 steps.
// Work it out on paper first.
//
// ============================================================================
// STEP 1..7: BUILD IT, one small function at a time. Test each one in main()
//            before moving on.
// ============================================================================

// TODO 1: `fn digits(n: u32) -> [u32; 4]`
//         Split a number into its 4 digits. digits(3524) -> [3, 5, 2, 4]
//         digits(999) -> [0, 9, 9, 9]
//         Hint: what do `n % 10` and `n / 10` give you? Fill the array from the end.
fn digits(n: u32) -> [u32; 4] {
    [(n / 1000) % 10, (n / 100) % 10, (n / 10) % 10, (n / 1) % 10]
}
// TODO 2: `fn from_digits(d: [u32; 4]) -> u32`
//         The reverse: [3, 5, 2, 4] -> 3524,   [0, 9, 9, 9] -> 999
fn from_digits(d: [u32; 4]) -> u32 {
    d[0] * 1000 + d[1] * 100 + d[2] * 10 + d[3]
}

// TODO 3: `fn sort_digits(d: [u32; 4]) -> [u32; 4]`   (ascending)
//         Don't use any built-in sort! Research "bubble sort" and implement it yourself.
//         Hints:
//         - The parameter needs `mut` to modify it: `fn sort_digits(mut d: [u32; 4])`
//         - Swap two elements:  let tmp = d[i]; d[i] = d[i + 1]; d[i + 1] = tmp;
//           (or research the `swap` method on arrays)
//         - Arrays of u32 are Copy, so passing and returning them is easy. (Lesson 06 explains why.)
fn sort_digits(mut d: [u32; 4]) -> [u32; 4] {
    loop {
        let mut i = 0;
        let mut is_swapped = false;
        while i < 3 {
            let tmp = d[i];
            if d[i] > d[i + 1] {
                d[i] = d[i + 1];
                d[i + 1] = tmp;
                is_swapped = true;
            }
            i = i + 1;
        }
        if !is_swapped {
            break d;
        }
    }
}

// TODO 4: `fn reverse_digits(d: [u32; 4]) -> [u32; 4]`
//         [2, 3, 4, 5] -> [5, 4, 3, 2]. Now you can build "descending" from "ascending".
fn reverse_digits(d: [u32; 4]) -> [u32; 4] {
    let mut result = sort_digits(d);
    result.reverse();
    result
}

// TODO 5: `fn kaprekar_step(n: u32) -> u32`
//         One step of the routine, built from TODO 1-4.
//         kaprekar_step(3524) -> 3087
fn kaprekar_step(n: u32) -> u32 {
    let asc = sort_digits(digits(n));
    let desc = reverse_digits(digits(n));

    let eq = from_digits(desc) - from_digits(asc);

    eq
}

// TODO 6: `fn is_repdigit(n: u32) -> bool`
//         true when all 4 digits are the same (1111, 2222, ... and what about 0?)
fn is_repdigit(n: u32) -> bool {
    let d = digits(n);
    d[0] == d[1] && d[1] == d[2] && d[2] == d[3]
}
// TODO 7: `fn steps_to_kaprekar(n: u32) -> u32`
//         How many steps until you reach 6174? Print each step as you go, e.g.:
//             3524 -> 3087 -> 8352 -> 6174   (3 steps)
//         Hint: `print!` without newline, from lesson 01.
//         What should happen if n is 6174 itself? Or a repdigit? (Don't loop forever!)
fn steps_to_kaprekar(n: u32) -> (u32, u32) {
    let mut last = n;
    let mut steps = 0;
    loop {
        steps = steps + 1;
        last = kaprekar_step(last);

        if is_repdigit(last) || last == 6174 || last == 0 {
            // print!("6174    ({} steps)", steps - 1);
            break (last, steps - 1);
        } else {
            // print!("{} -> ", last);
        }
    }
}

fn main() {
    // Call and test your functions here as you write them.
    // TODO 8: EXPLORE ALL OF THEM.
    //   Loop over every number 0..=9999 (not repdigits), compute the steps for each,
    //   and count how many numbers need 0, 1, 2, ... 7 steps.
    //   Hint: a "histogram" is just an array of counters: let mut counts = [0u32; 8];
    //         counts[steps as usize] += 1;     (why `as usize`? see lesson 03!)
    //   (Turn OFF the step printing from TODO 7 for this part, or you'll get 10,000 lines.
    //    Maybe split it into two functions: one that prints, one that only counts.)
    //
    //   Print a nice table using lesson 01 formatting, with a bar of '#' characters:
    //       steps  count
    //           0      1  #
    //           1    ...  ####
    //   (one '#' per 100 numbers is a good scale)
    //
    //   Does your result confirm your research answer for Q4?
    let mut counts = [0u32; 8];

    for i in 0..9999 {
        let (_number, steps) = steps_to_kaprekar(i);
        counts[steps as usize] += 1;
    }

    println!("{:>10}{:>10}", "steps", "count");
    for i in 0..8 {
        println!("{:>10}{:>10}", i, counts[i]);
    }

    // TODO 9: Which numbers need the MAXIMUM number of steps? Print the first 10 of them.

    // STRETCH GOALS (pick any):
    //   A. The 3-digit version: verify your answer to Q5 for all 3-digit numbers.
    //      (You'll need [u32; 3] versions of your functions. Notice the copy-paste?
    //       Lesson 16, generics, will make you happy.)
    //   B. Research what happens with 2-digit and 5-digit numbers. Do they converge to
    //      one number, or something else? (Search: "Kaprekar routine cycles")
    //   C. Compute the AVERAGE number of steps over all valid 4-digit numbers, as an f64
    //      with 3 decimals.
}
