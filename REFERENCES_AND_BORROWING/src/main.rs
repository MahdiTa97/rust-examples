// Immutable References
fn main() {
    let s1 = String::from("hello");

    // The ampersands represent references
    // and they allow you to refer to some value without taking ownership of it
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Mutable References
// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value
fn main_2() {
    let mut s1 = String::from("hello");

    let len = calculate_length_2(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_2(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

// We can use curly brackets to create a new scope, allowing for multiple mutable references
fn main_3() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short)
fn main_4() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
}

// The Rules of References
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.
