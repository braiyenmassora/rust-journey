# Rust Journey

A personal journey of learning Rust, one exercise at a time.

## Exercises

| File | Topic |
|---|---|
| `exercise/hello_rust.rs` | Functions and the `println!` macro |
| `exercise/variables.rs` | Variables, constants, shadowing, and basic types |

## Getting Started

Requires [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`).

```bash
git clone https://github.com/braiyenmassora/rust-journey.git
cd rust-journey
cargo run
```

## Adding a New Exercise

1. Create `exercise/<topic>.rs` with a `pub fn <topic>()`.
2. Add `pub mod <topic>;` inside `mod exercise { ... }` in `main.rs`.
3. Call it inside `fn main()`: `exercise::<topic>::<topic>();`.
4. Add a row to the table above.

## Comment Style

- Use `//` for comments.
- Keep each comment short and simple.
- Comments are written in English to practice technical writing.

## Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)
