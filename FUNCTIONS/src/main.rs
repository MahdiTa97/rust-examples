// Rust code uses snake case as the conventional style for function and variable names

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// ============ Statements & Expressions ============
// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resulting value

// Calling a function is an expression.
// Calling a macro is an expression.
// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// ============ Functions with Return Values ============
// We don’t name return values, but we must declare their type after an arrow (->)
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

// Example 1
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

// Example 2
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
