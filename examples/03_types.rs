// Solution: Lesson 03
// Run: cargo run --example 03_types

fn main() {
    // TODO 1 (your table version was great, kept it)
    println!("{:>8}{:>22}{:>22}", "", "MIN", "MAX");
    println!("{:>8}{:>22}{:>22}", "u8", u8::MIN, u8::MAX);
    println!("{:>8}{:>22}{:>22}", "i8", i8::MIN, i8::MAX);
    println!("{:>8}{:>22}{:>22}", "i32", i32::MIN, i32::MAX);
    println!("{:>8}{:>22}{:>22}", "u64", u64::MIN, u64::MAX);

    // TODO 2
    let a: i32 = 10;
    let b: f64 = 2.5;
    // let c = a * b; // error[E0277]: cannot multiply `i32` by `f64`
    let c = a as f64 * b;
    println!("c = {c}");

    // TODO 3
    println!("7 / 2     = {}", 7 / 2); // 3   (integer division truncates)
    println!("7.0 / 2.0 = {}", 7.0 / 2.0); // 3.5
    println!("7 % 2     = {}", 7 % 2); // 1

    // TODO 4
    let x: u8 = 250;
    println!("checked:    {:?}", x.checked_add(10)); // None
    println!("checked:    {:?}", x.checked_add(5)); // Some(255)
    println!("wrapping:   {}", x.wrapping_add(10)); // 4   (260 - 256)
    println!("saturating: {}", x.saturating_add(10)); // 255

    // TODO 5
    // `300 as u8` doesn't compile: the literal 300 would be typed as u8 and doesn't fit
    // (error: literal out of range for `u8`). So give it a bigger type first:
    println!("300 as u8      = {}", 300u32 as u8); // 44  (300 - 256, keeps the low 8 bits)
    println!("-1i32 as u32   = {}", -1i32 as u32); // 4294967295 (same bits, read as unsigned)
    println!("3.99 as i32    = {}", 3.99_f64 as i32); // 3   (truncates, does NOT round)
    println!("'A' as u32     = {}", 'A' as u32); // 65  (char is 4 bytes, so u32 is lossless)
    println!("b'A'           = {}", b'A'); // 65  (a byte literal is already a u8)
    println!("97u8 as char   = {}", 97u8 as char); // a

    // TODO 6: destructuring, all three in one line
    let person = ("Majid", 29, 174.2);
    let (name, age, height) = person;
    println!("{name} is {age} years old and {height} cm tall");

    // TODO 7
    let temperatures: [f64; 5] = [0.2, 10.3, 20.4, 30.5, 40.6];
    println!("first: {}", temperatures[0]);
    println!("last:  {}", temperatures[temperatures.len() - 1]);
    println!("all:   {:?}", temperatures);

    // TODO 8
    // let arr = [1, 2, 3];
    // arr[10];            // compile error: the length (3) is part of the type [i32; 3]
    // let i = 10; arr[i]; // ALSO a compile error: the compiler can trace that i is always 10
    // A runtime panic only happens when the index is unknown at compile time,
    // e.g. coming from user input or a command-line argument:
    let arr = [1, 2, 3];
    let i = std::env::args().count() + 9; // 10 when run with no args, but the compiler can't know
    match arr.get(i) {
        Some(v) => println!("arr[{i}] = {v}"),
        None => println!("arr[{i}] is out of bounds (arr[i] would have panicked)"),
    }
}
