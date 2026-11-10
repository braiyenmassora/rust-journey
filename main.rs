// `mod exercise { ... }` = use the folder `exercise/`.
// `pub mod hello_rust;` = use the file `exercise/hello_rust.rs`.
mod exercise {
    pub mod hello_rust;
    pub mod variables;
}

fn main() {
    exercise::hello_rust::hello_rust();
    exercise::variables::variables();
}
