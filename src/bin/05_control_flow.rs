// Lesson 05: Control flow
// Run: cargo run --bin 05_control_flow
//
// HINTS
// - if cond { } else if cond { } else { }   -- no parentheses needed, cond MUST be bool
//   (no "truthy" values: `if 1 {}` is an error).
// - `if` is an expression: let x = if ok { 1 } else { 2 };  (both arms same type!)
// - loop { }              infinite, exit with `break`. `break value;` returns a value from loop.
// - while cond { }
// - for i in 0..5 { }     0,1,2,3,4      (0..=5 includes 5)
//   for x in arr { }      iterate an array
//   (1..4).rev()          reversed range
// - Labels for nested loops: 'outer: loop { loop { break 'outer; } }
// - `continue` skips to the next iteration.

use std::{io::ErrorKind::BrokenPipe, thread::LocalKey};

fn fib(n: u32) -> u64 {
    let mut f = 0;
    let mut s = 1;
    let mut r = 0;
    let mut i = 0;

    while i < n {
        r = f + s;
        f = s;
        s = r;

        print!("{r}, ");
        i = i + 1;
    }

    r
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;
    loop {
        if i > n / 2 {
            break true;
        }

        if n % i == 0 {
            break false;
        }

        i = i + 1;
    }
}

fn main() {
    // TODO 1: Given a number, print "negative", "zero", or "positive".
    let number = 10;
    if number > 0 {
        println!("positive");
    } else if number == 0 {
        println!("zero");
    } else {
        println!("negative");
    };

    // TODO 2: Use `if` as an expression to set `let kind = ...` to "even" or "odd". Print it.
    //         Then try making one arm return a number. Read the error.
    let kind = if number % 2 == 0 { "even" } else { "odd" };
    println!("number is {}", kind);

    // TODO 3: Use `loop` with a counter. Break when counter reaches 10,
    //         and return counter * 2 from the loop: let result = loop { ... };
    let mut counter = 0;
    let result = loop {
        counter = counter + 1;
        if counter >= 10 {
            break counter * 2;
        };
    };

    println!("end of loop, {result}");

    // TODO 4: Countdown with `while`: 3, 2, 1, "LIFTOFF!"
    //         Then do the same with `for` and `.rev()`.
    let mut i = 3;

    while i >= 0 {
        if i != 0 {
            print!("{} ", i);
        } else {
            print!(" LIFTOFF!");
        }
        i = i - 1;
    }
    println!();

    for j in (0..3).rev() {
        if j == 0 {
            print!(" LIFTOFF!");
        } else {
            print!("{} ", j);
        }
    }

    println!();

    // TODO 5: FizzBuzz 1..=15: multiples of 3 -> "Fizz", of 5 -> "Buzz", of both -> "FizzBuzz".

    for f in 1..16 {
        if f % 3 == 0 && f % 5 == 0 {
            print!("FizzBuzz ");
        } else if f % 3 == 0 {
            print!("Fizz ");
        } else if f % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{f} ");
        }
    }
    println!();

    // TODO 6: Sum all elements of [10, 20, 30, 40] using a `for` loop.
    let mut sum = 0;
    let arr = [10, 20, 30, 40];
    for n in arr {
        sum = sum + n
    }

    // TODO 7: Nested loops with a label: loop i in 1..=5 and j in 1..=5,
    //         print (i, j), and break out of BOTH loops when i * j == 6.

    let mut i = 1;
    'outer: loop {
        let mut j = 1;
        loop {
            if i * j == 6 {
                break 'outer;
            } else if j > 5 {
                break;
            } else {
                j = j + 1;
            }
        }
        i = i + 1;
        if i > 5 {
            break;
        }
    }
    // CHALLENGE: Write `fn fib(n: u32) -> u64` (iterative) and print the first 20 Fibonacci numbers.
    //            Then write `fn is_prime(n: u32) -> bool` and print primes below 50.
    fib(20);
    println!("");
    println!("13 is prime: {}", is_prime(13));
}
