/*
    Basic function example in Rust.

    Output:
    hello rust

    - `pub` makes this function public, so other modules can use it.
    - `fn` is the keyword used to define a function.
    - `hello_rust` is the function name. Rust uses snake_case for
      function names.
    - `()` holds the function's parameters. It is empty because this
      function takes no input.
    - `{ ... }` is the function body, where the code runs.
    - `println!` is a macro (note the `!`), not a regular function.
      It prints text to the console and adds a new line at the end.
    - Every statement ends with a semicolon `;`.
*/
pub fn hello_rust() {
    println!("hello rust");
}