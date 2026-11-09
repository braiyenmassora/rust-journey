/*
    Entry point of the program.

    - `mod` declares a module to organize code.
    - `pub mod hello_rust;` loads `src/exercise/hello_rust.rs`.
    - `pub mod variables;` loads `src/exercise/variables.rs`.
    - `pub` lets `main` access the modules.
    - `::` accesses items inside a module.
    - `fn main` is where the program starts.
*/
mod exercise {
    pub mod hello_rust;
    pub mod variables;
}

fn main() {
    exercise::hello_rust::hello_rust();
    exercise::variables::variables();
}