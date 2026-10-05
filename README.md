# Learn Rust — from the beginning

Each lesson is a file in `src/bin/`. It has **hints and tasks only**, so you write the code.

Run a lesson:

```sh
cargo run --bin 01_hello
```

Useful while you work:

```sh
cargo check            # fast compile check, no binary
cargo clippy           # lints: more idiomatic suggestions
cargo fmt              # auto-format
rustc --explain E0382  # long explanation of any error code
```

> Tip: read compiler errors slowly, top to bottom. The Rust compiler is your teacher.

## Roadmap

### Part 1: Basics
- [ ] 01_hello: `println!`, formatting, comments
- [ ] 02_variables: `let`, `mut`, shadowing, `const`
- [ ] 03_types: scalars, tuples, arrays, casting
- [ ] 04_functions: parameters, return values, expressions vs statements
- [ ] 05_control_flow: `if`, `loop`, `while`, `for`

### Part 2: Ownership (the heart of Rust)
- [ ] 06_ownership: move, clone, copy
- [ ] 07_borrowing: `&`, `&mut`, borrow rules
- [ ] 08_slices: `&str`, `&[T]`

### Part 3: Modeling data
- [ ] 09_structs, 10_enums_match, 11_option_result

### Part 4: Practical Rust
- [ ] 12_collections (Vec, String, HashMap), 13_errors (`?`), 14_modules
- [ ] 15_traits, 16_generics, 17_lifetimes
- [ ] 18_closures_iterators, 19_tests, 20_smart_pointers

### Challenges (research first, then build)
- [ ] challenge_01_kaprekar: the mysterious number 6174 (Part 1 only)
- [ ] challenge_02_packet: dissect a ping frame: Ethernet, IPv4, ICMP, checksums, subnets (Part 1 + bit ops)

### Part 5: Mini projects
- [ ] guessing game, todo CLI, word counter, ...

My reference solutions live in `examples/`: `cargo run --example 05_control_flow`.
Try the lesson yourself first, then compare.

When you finish a lesson, ask me to review it, and I'll add the next one.
