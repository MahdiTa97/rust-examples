fn main() {
    variables();
    shadowing();
    float_type();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is: {x}");
}

fn float_type() {
    // f32 has single-precision float
    // f64 has double-precision float

    let _x = 2.0;
    let _y: f32 = 3.0;
}

fn numeric_operations() {
    // Addition
    let _sum = 5 + 10;

    // Subtraction
    let _difference = 95.5 - 4.3;

    // Multiplication
    let _product = 4 * 30;

    // Division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3;

    // Reminder
    let _reminder = 43 % 5;
}

fn boolean_type() {
    let _t = true;

    // With explicit type annotation
    let _f: bool = false;
}

fn character_type() {
    let _c = 'z';

    // With explicit type annotation
    let _z: char = 'ℤ';

    let _heart_eyed_cat = '😻';
}

// Rust has two primitive compound types:
// 1-tuples and 2-arrays.

// 1-Tuple type
fn tuple_type() {
    // Tuples have a fixed length
    // It means once declared, they can't grow or shrink
    let _tup: (i32, i32, u8, f64) = (500, -1, 2, 3.3);

    // Access to tuple items like this
    let (a, b, c, d) = _tup;
    // or this
    let e = _tup.0;

    println!("The tuple value is: {a} ,{b}, {c}, {d}, {e}");
}

// 2-Array type
// Every element of an array must have the same type
// Arrays in Rust have a fixed length
// Arrays are more useful when you know the number of elements will not need to change
fn array_type() {
    let _a = [1, 2, 3, 4, 5];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // let _c = [3, 3, 3, 3, 3];
    let _c = [3; 5];

    // Access to array items like this:
    let _first = _a[0];
    let _second = _a[1];
}
