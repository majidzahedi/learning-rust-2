// Solution: Lesson 05
// Run: cargo run --example 05_control_flow

// Only calculates, never prints. The caller decides what to do with the value.
// Returns the n-th Fibonacci number: fib(0)=0, fib(1)=1, fib(2)=1, fib(3)=2, ...
fn fib(n: u32) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false; // 0 and 1 are not prime
    }
    let mut i = 2;
    // A divisor bigger than sqrt(n) would pair with one smaller than sqrt(n),
    // so we only need to check up to sqrt(n).
    while i * i <= n {
        if n.is_multiple_of(i) {
            // same as n % i == 0, but reads like English
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    // TODO 1
    let number = -7;
    if number < 0 {
        println!("negative");
    } else if number == 0 {
        println!("zero");
    } else {
        println!("positive");
    }

    // TODO 2
    let kind = if number % 2 == 0 { "even" } else { "odd" };
    println!("{number} is {kind}");
    // let kind = if number % 2 == 0 { "even" } else { 1 };
    // error[E0308]: `if` and `else` have incompatible types

    // TODO 3
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result = {result}"); // 20

    // TODO 4: while
    let mut i = 3;
    while i > 0 {
        println!("{i}");
        i -= 1;
    }
    println!("LIFTOFF!");

    // TODO 4: for + rev. A range only counts UP, so build 1..=3 and reverse it.
    for j in (1..=3).rev() {
        println!("{j}");
    }
    println!("LIFTOFF!");

    // TODO 5: ..= includes 15. Check "both" FIRST, and print the number otherwise.
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }

    // TODO 6: iterate the values directly, no indexes
    let arr = [10, 20, 30, 40];
    let mut sum = 0;
    for n in arr {
        sum += n;
    }
    println!("sum = {sum}"); // 100
    // (later you'll write this as: let sum: i32 = arr.iter().sum();)

    // TODO 7: `for` handles start/end/reset for you
    'outer: for i in 1..=5 {
        for j in 1..=5 {
            println!("({i}, {j})");
            if i * j == 6 {
                println!("found i * j == 6, stopping both loops");
                break 'outer;
            }
        }
    }

    // CHALLENGE: first 20 Fibonacci numbers
    for n in 0..20 {
        print!("{} ", fib(n));
    }
    println!();

    // CHALLENGE: primes below 50
    for n in 0..50 {
        if is_prime(n) {
            print!("{n} ");
        }
    }
    println!();
}
