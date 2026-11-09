# Rust Journey

A personal journey of learning Rust, one exercise at a time.

## Exercises

| File | Topic |
|---|---|
| `src/exercise/hello_rust.rs` | Functions and the `println!` macro |
| `src/exercise/variables.rs` | Variables, constants, shadowing, and basic types |

## Project Structure

```
rust-journey/
├── Cargo.toml
└── src/
    ├── exercise/   # one file per topic
    └── main.rs     # entry point
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)

## Run

```bash
git clone https://github.com/<your-username>/rust-journey.git
cd rust-journey
cargo run
```

## Learning Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/) (the official Rust book)
- [Rustlings](https://github.com/rust-lang/rustlings) (small exercises to get comfortable reading and writing Rust)

## Notes

- Each exercise is a module under `src/exercise/` and is called from `main.rs`.
- Code comments are written in English to practice technical writing.