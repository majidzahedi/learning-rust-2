// Lesson 03: Data types
// Run: cargo run --bin 03_types
//
// HINTS
// - Integers: i8 i16 i32 i64 i128 isize (signed), u8 ... u128 usize (unsigned).
//   Default integer type is i32. usize is used for indexing/lengths.
// - Floats: f32, f64 (default f64).
// - bool: true/false.  char: 'a' (single quotes, 4 bytes, Unicode: '😀' works).
// - Literals: 1_000_000, 0xff, 0b1010, 5u8 (suffix sets type).
// - Min/max: i32::MAX, u8::MIN
// - Casting is explicit with `as`: `let f = 7 as f64;`. No implicit conversions!
// - Overflow: in debug builds `255u8 + 1` panics. Use wrapping_add / checked_add / saturating_add.
// - Tuple: let t: (i32, f64, char) = (1, 2.0, 'c');  access t.0, destructure let (a, b, c) = t;
// - Array: fixed size, same type: let a: [i32; 3] = [1, 2, 3];  let zeros = [0; 5];
//   a.len(), a[0]. Out-of-bounds index panics at runtime.

fn main() {
    // TODO 1: Print the min and max of u8, i8, i32, and u64.
    println!("____TODO1____");
    println!("{:>8}{:>16}{:>22}", "", "MIN", "MAX");
    println!("{:>8}{:>16}{:>22}", "u8", u8::MIN, u8::MAX);
    println!("{:>8}{:>16}{:>22}", "i8", i8::MIN, i8::MAX);
    println!("{:>8}{:>16}{:>22}", "i32", i32::MIN, i32::MAX);
    println!("{:>8}{:>16}{:>22}", "u64", u64::MIN, u64::MAX);
    println!();
    // TODO 2: Try `let a: i32 = 10; let b: f64 = 2.5; let c = a * b;`
    //         Read the error. Fix it with `as`.
    let a: i32 = 10;
    let b: f64 = 2.5;
    let c = a as f64 * b;
    println!("____TODO2____");
    println!("c is: {c}");
    println!();

    // TODO 3: Integer division vs float division: print 7 / 2 and 7.0 / 2.0. And 7 % 2.
    println!("____TODO3____");
    println!("7/2 is: {}", 7 / 2);
    println!("7.0/2.0: {}", 7.0 / 2.0);
    println!("7%2: {}", 7 % 2);
    println!();

    // TODO 4: Overflow. With `let x: u8 = 250;` print:
    //         x.checked_add(10), x.wrapping_add(10), x.saturating_add(10)
    //         (checked_add returns an Option, so print it with {:?})
    let x: u8 = 250;
    println!("____TODO4____");
    println!("checked add: {:?}", x.checked_add(10));
    println!("wrapping add: {}", x.wrapping_add(10));
    println!("saturating add: {}", x.saturating_add(10));
    println!();

    // TODO 5: Casting surprises. Predict then print:
    //         300 as u8,   -1i32 as u32,   3.99_f64 as i32,   'A' as u8,   97u8 as char
    println!("____TODO5____");
    println!("300 as u8 is: {}", 300u32 as u8);
    println!("-1 i32 as u32 is: {}", -1i32 as u32);
    println!("3.99_f64 as i32 is: {}", 3.99_f64 as i32);
    println!("'A' as u8 is: {}", 'A' as u8);
    println!("97u8 as char is: {}", 97u8 as char);
    println!();

    // TODO 6: Make a tuple (name, age, height) and destructure it into 3 variables. Print them.
    let tuple = ("majid", 29, 174.2);
    let name = tuple.0;
    let age = tuple.1;
    let height = tuple.2;

    println!("____TODO6____");
    println!("{name} is {age} y'o with height of {height} cm");
    println!();
    // TODO 7: Make an array of 5 temperatures (f64). Print the first, the last (use len()),
    //         and the whole array with {:?}.
    let temperatures: [f64; 5] = [0.2, 10.3, 20.4, 30.5, 40.6];
    println!("____TODO6____");
    println!("first is: {}", temperatures[0]);
    println!("last is: {}", temperatures[temperatures.len() - 1]);
    println!();

    // TODO 8 (curious): What happens with `let arr = [1, 2, 3]; let i = 10; arr[i];` ?
    //         And with `arr[10]` written directly? Why does one fail at compile time?
    // actually i dont kwno why!
    // let arr = [1, 2, 3];
    // let i = 10;
    // arr[10];
}
