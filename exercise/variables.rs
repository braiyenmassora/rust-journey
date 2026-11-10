// Variables in Rust

// Constant: value never changes. Type must be written.
const MAX_RETRIES: u32 = 3;

// Static: global value. Type must be written.
static APP_NAME: &str = "MyApp";

pub fn variables() {
    // 1. `let` = cannot be changed
    let age = 34;
    let name = "Joni";
    println!("Name: {name}, age: {age}");

    // 2. `let mut` = can be changed
    let mut email = "rusting@gmail.com";
    println!("Old email: {email}");
    email = "joni@gmail.com";
    println!("New email: {email}");

    // 3. Number types
    let a = 3; // Rust guesses the type: i32
    let b: i32 = -1; // i32 can be negative
    let c: u8 = 10; // u8 is 0 to 255, no negative
    println!("Numbers: {a}, {b}, {c}");

    // 4. Shadowing = make a new variable with the same name
    let value = 5;
    let value = value + 1; // 6
    let value = value * 2; // 12
    println!("Shadowing: {value}");

    // 5. Constant and static (written at the top of this file)
    println!("Constant: {MAX_RETRIES}, Static: {APP_NAME}");

    // 6. Other types
    let pi: f64 = 3.14; // decimal number
    let letter: char = 'R'; // one character, single quotes
    let is_active: bool = true; // true or false
    println!("Float: {pi}, Char: {letter}, Bool: {is_active}");

    // 7. Tuple = group of values, types can be different
    let person = ("Joni", 34, true);
    println!("Tuple: {}, {}, {}", person.0, person.1, person.2);

    // 8. Array = list of values, all the same type
    let numbers = [1, 2, 3];
    println!("Array first item: {}", numbers[0]);
}
