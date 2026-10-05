// Solution: Challenge 01 - Kaprekar's Mysterious Number
// Run: cargo run --example challenge_01_kaprekar
//
// RESEARCH ANSWERS
//   Q1. One step: arrange the digits in DESCENDING order, then in ASCENDING order,
//       and subtract: big - small. Repeat with the result.
//       Example: 3524 -> 5432 - 2345 = 3087
//       This is specific to base 10. Other bases end at different numbers or in cycles.
//   Q2. Repdigits (1111, 2222, ..., and 0000) don't work: big == small, so you get 0
//       and stay there forever. Every number with at least 2 different digits works.
//   Q3. Always keep 4 digits by padding with leading zeros: 999 is 0999.
//       Example: 2111 -> 2111 - 1112 = 0999 -> 9990 - 0999 = 8991 -> ...
//       (Treating 999 as a 3-digit number would send you to 495 instead.)
//   Q4. At most 7 steps.
//   Q5. 495.
//
// STRETCH B (researched, not coded):
//   2 digits: no fixed point, falls into the cycle 9 -> 81 -> 63 -> 27 -> 45 -> 9
//   5 digits: no fixed point either. Numbers fall into cycles such as
//             61974 -> 82962 -> 75933 -> 63954 -> 61974
//   Only 3 digits (495) and 4 digits (6174) have a single "magic" number.

const KAPREKAR: u32 = 6174;

// TODO 1: fill from the END, peeling off the last digit each time.
fn digits(mut n: u32) -> [u32; 4] {
    let mut d = [0; 4];
    for i in (0..4).rev() {
        d[i] = n % 10; // last digit
        n /= 10; // drop the last digit
    }
    d // numbers below 1000 automatically get leading zeros: 999 -> [0, 9, 9, 9]
}

// TODO 2: "shift left and add": ((3 * 10 + 5) * 10 + 2) * 10 + 4 = 3524
fn from_digits(d: [u32; 4]) -> u32 {
    let mut n = 0;
    for digit in d {
        n = n * 10 + digit;
    }
    n
}

// TODO 3: bubble sort. Each pass "bubbles" the biggest remaining value to the end.
fn sort_digits(mut d: [u32; 4]) -> [u32; 4] {
    loop {
        let mut swapped = false;
        for i in 0..d.len() - 1 {
            if d[i] > d[i + 1] {
                d.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            return d; // a pass with no swaps means it's sorted
        }
    }
}

// TODO 4: ONLY reverses, so it does exactly what the name says.
//         Swap the outside pair, then move inwards: (0, 3), (1, 2)
fn reverse_digits(mut d: [u32; 4]) -> [u32; 4] {
    let len = d.len();
    for i in 0..len / 2 {
        d.swap(i, len - 1 - i);
    }
    d
}

// TODO 5: sort ONCE, then reverse the sorted array to get descending.
fn kaprekar_step(n: u32) -> u32 {
    let asc = sort_digits(digits(n));
    let desc = reverse_digits(asc);
    from_digits(desc) - from_digits(asc) // desc >= asc always, so u32 never underflows
}

// TODO 6: 0 counts too, it's 0000.
fn is_repdigit(n: u32) -> bool {
    let d = digits(n);
    d[0] == d[1] && d[1] == d[2] && d[2] == d[3]
}

// TODO 7, part 1: only COUNTS, never prints, so the histogram can use it.
// CHECK FIRST, then step. That way 6174 itself is 0 steps, and no `- 1` is needed.
// Callers must not pass a repdigit (it would loop forever on 0).
fn steps_to_kaprekar(mut n: u32) -> u32 {
    let mut steps = 0;
    while n != KAPREKAR {
        n = kaprekar_step(n);
        steps += 1;
    }
    steps
}

// TODO 7, part 2: only PRINTS. Same loop, different job.
fn print_chain(mut n: u32) {
    if is_repdigit(n) {
        println!("{n:04} is a repdigit, it goes to 0 and stays there");
        return;
    }
    let start = n;
    print!("{n:04}"); // {:04} pads with zeros, so 999 shows as 0999
    while n != KAPREKAR {
        n = kaprekar_step(n);
        print!(" -> {n:04}");
    }
    println!("   ({} steps)", steps_to_kaprekar(start));
}

// STRETCH A: the 3-digit versions. Same logic, [u32; 3] instead of [u32; 4].
// All this copy-paste is exactly what generics (lesson 16) will remove.
fn digits3(mut n: u32) -> [u32; 3] {
    let mut d = [0; 3];
    for i in (0..3).rev() {
        d[i] = n % 10;
        n /= 10;
    }
    d
}

fn kaprekar_step3(n: u32) -> u32 {
    let mut d = digits3(n);
    // with only 3 elements, a simple bubble sort is 3 compare-and-swaps
    if d[0] > d[1] {
        d.swap(0, 1);
    }
    if d[1] > d[2] {
        d.swap(1, 2);
    }
    if d[0] > d[1] {
        d.swap(0, 1);
    }
    let asc = d[0] * 100 + d[1] * 10 + d[2];
    let desc = d[2] * 100 + d[1] * 10 + d[0];
    desc - asc
}

fn main() {
    // Test each function with known answers. `assert_eq!(a, b)` panics if a != b.
    // That's a quick way to self-check (lesson 19 turns these into real tests).
    assert_eq!(digits(3524), [3, 5, 2, 4]);
    assert_eq!(digits(999), [0, 9, 9, 9]);
    assert_eq!(from_digits([0, 9, 9, 9]), 999);
    assert_eq!(sort_digits([3, 5, 2, 4]), [2, 3, 4, 5]);
    assert_eq!(reverse_digits([3, 5, 2, 4]), [4, 2, 5, 3]); // really reverses, doesn't sort
    assert_eq!(kaprekar_step(3524), 3087);
    assert!(is_repdigit(0) && is_repdigit(7777) && !is_repdigit(7778));
    assert_eq!(steps_to_kaprekar(3524), 3); // the sanity check from the research step
    assert_eq!(steps_to_kaprekar(KAPREKAR), 0);
    println!("all checks passed\n");

    print_chain(3524);
    print_chain(2111); // the leading-zero case from Q3
    print_chain(KAPREKAR);
    print_chain(5555);
    println!();

    // TODO 8: histogram. `..=` so 9999 is included, and repdigits are skipped.
    let mut counts = [0u32; 8];
    let mut total_steps = 0;
    let mut valid = 0;
    let mut max_steps = 0;
    for n in 0..=9999 {
        if is_repdigit(n) {
            continue;
        }
        let steps = steps_to_kaprekar(n);
        counts[steps as usize] += 1; // array indexes must be usize
        total_steps += steps;
        valid += 1;
        if steps > max_steps {
            max_steps = steps;
        }
    }

    println!("{:>5}  {:>5}", "steps", "count");
    // .iter().enumerate() gives (index, &value), no manual indexing needed
    for (steps, count) in counts.iter().enumerate() {
        print!("{steps:>5}  {count:>5}  ");
        // One '#' per 100 numbers, rounded UP so small counts still get a mark.
        for _ in 0..count.div_ceil(100) {
            print!("#");
        }
        println!();
    }
    // Computed, not typed in: this is what actually checks the research answer.
    println!("checked {valid} numbers, max steps = {max_steps} (research says 7)\n");

    // TODO 9: the first 10 numbers that need the maximum number of steps
    print!("first 10 numbers needing {max_steps} steps:");
    let mut found = 0;
    let mut n = 0;
    while found < 10 {
        if !is_repdigit(n) && steps_to_kaprekar(n) == max_steps {
            print!(" {n:04}");
            found += 1;
        }
        n += 1;
    }
    println!("\n");

    // STRETCH C: average. Cast BOTH sides to f64, otherwise it's integer division.
    let average = total_steps as f64 / valid as f64;
    println!("average steps: {average:.3}\n");

    // STRETCH A: check that every non-repdigit 3-digit number reaches 495
    let mut max_steps3 = 0;
    for n in 0..=999 {
        let d = digits3(n);
        if d[0] == d[1] && d[1] == d[2] {
            continue;
        }
        let mut x = n;
        let mut steps = 0;
        while x != 495 {
            x = kaprekar_step3(x);
            steps += 1;
        }
        if steps > max_steps3 {
            max_steps3 = steps;
        }
    }
    println!("3 digits: all numbers reach 495, max steps = {max_steps3}");
}
