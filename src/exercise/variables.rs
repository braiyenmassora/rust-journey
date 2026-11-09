/*
    Variables in Rust: complete examples.

    Rust has no `var` keyword. It uses:
    - `let`      : immutable variable (like `const` in JavaScript)
    - `let mut`  : mutable variable (like `let` / `var` in JavaScript)
    - `const`    : compile-time constant
    - `static`   : global variable with a fixed memory address

    Output:
    Original email: rusting@gmail.com
    Name: Joni, age: 34
    Updated email: joni@gmail.com
    Number examples: 3, -1, 10
    Shadowing: 5 -> 6 -> 12
    Shadowed as text: 12 (text)
    Constants: 3, 10
    Static: MyApp
    Float: 3.14, Char: R, Bool: true
    Tuple: (Joni, 34, true)
    Destructured: Joni, 34, true
    Array: [1, 2, 3], first element: 1
    Declared first: 100
*/

/*
    Constants
    - Declared with `const`, never with `mut`.
    - The type MUST be written explicitly.
    - The value must be known at compile time.
    - Naming convention: SCREAMING_SNAKE_CASE.
    - Can be declared outside functions (global scope).
*/
const MAX_RETRIES: u32 = 3;

/*
    Static variables
    - Declared with `static`, a global variable that lives for the
      whole program and has a fixed memory address.
    - The type MUST be written explicitly.
    - Immutable by default. `static mut` exists but requires `unsafe`,
      so avoid it as a beginner.
    - Naming convention: SCREAMING_SNAKE_CASE.
*/
static APP_NAME: &str = "MyApp";

pub fn variables() {
    /*
        1. Immutable variables
        By default, variables in Rust are immutable.
        Once a value is assigned, it cannot be changed.
        (Similar to `const` in JavaScript.)
    */
    let age = 34;
    let full_name = "Joni";

    /*
        2. Mutable variables
        Add `mut` when you want to change a variable's value later.
        (Similar to `let` or `var` in JavaScript.)
    */
    let mut email = "rusting@gmail.com";
    println!("Original email: {}", email);

    /* Allowed, because `email` is declared with `mut` */
    email = "joni@gmail.com";

    println!("Name: {}, age: {}", full_name, age);
    println!("Updated email: {}", email);

    /*
        3. Data types
        Rust can infer a variable's type from its value (type inference).
        You can also write the type explicitly.

        - i32: signed integer (can be negative)
        - u8:  unsigned integer (0 to 255)
    */
    let inferred_number = 3; /* inferred as i32 */
    let signed_number: i32 = -1;
    let unsigned_number: u8 = 10;

    println!(
        "Number examples: {}, {}, {}",
        inferred_number, signed_number, unsigned_number
    );

    /*
        4. Ignoring values with `_`
        - `let _ = value;` does not store the value at all.
        - `let _name = value;` still stores the value, but the compiler
          will not show an unused-variable warning.
    */
    let _ = "This value is ignored";
    let _unused_value = "This value is stored";

    /*
        5. Shadowing
        Declaring a new variable with the same name hides the old one.
        Unlike `mut`, shadowing creates a NEW variable, so it can
        even change the type (number -> text).
    */
    let value = 5;
    let value = value + 1; /* 6 */
    let value = value * 2; /* 12 */
    println!("Shadowing: 5 -> 6 -> {}", value);

    let value = format!("{} (text)", value); /* type changed to String */
    println!("Shadowed as text: {}", value);

    /*
        6. Constants
        - `MAX_RETRIES` is declared above, outside the function.
        - `LOCAL_LIMIT` is declared inside the function.
        - Unlike `let`, a constant can never be changed or shadowed
          in the same scope.
    */
    const LOCAL_LIMIT: i32 = 10;
    println!("Constants: {}, {}", MAX_RETRIES, LOCAL_LIMIT);

    /*
        7. Static variables
        See `APP_NAME` above. Use `static` for global data that needs
        a fixed memory address. For most cases, `const` is enough.
    */
    println!("Static: {}", APP_NAME);

    /*
        8. Other basic scalar types
        - f64:  floating-point number (default for decimals)
        - char: a single Unicode character, uses single quotes
        - bool: true or false
    */
    let pi: f64 = 3.14;
    let letter: char = 'R';
    let is_active: bool = true;
    println!("Float: {}, Char: {}, Bool: {}", pi, letter, is_active);

    /*
        9. Compound types: tuple and array
        - Tuple: fixed size, can hold different types.
        - Array: fixed size, all elements share the same type.
    */
    let person: (&str, i32, bool) = ("Joni", 34, true);
    println!("Tuple: ({}, {}, {})", person.0, person.1, person.2);

    /* Destructuring: unpack a tuple into separate variables */
    let (name, years, active) = person;
    println!("Destructured: {}, {}, {}", name, years, active);

    let numbers: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}, first element: {}", numbers, numbers[0]);

    /*
        10. Declaring first, assigning later
        A variable can be declared without a value, as long as it is
        assigned exactly once before it is used.
    */
    let score: i32;
    score = 100;
    println!("Declared first: {}", score);
}