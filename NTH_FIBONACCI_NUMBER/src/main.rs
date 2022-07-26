use std::{io, process};

fn main() {
    let mut nth_number: String = String::new();
    println!("Please enter nth number:");

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read number.");

    let result: u32 = match nth_number.trim().parse::<u32>() {
        Ok(res) => fibonacci_calculator(res - 1),
        Err(_) => {
            println!("this was not an integer:");
            process::exit(1);
        }
    };

    println!("The {nth_number} nth number is = {result}");
}

fn fibonacci_calculator(nth_number: u32) -> u32 {
    if nth_number <= 1 {
        nth_number
    } else {
        fibonacci_calculator(nth_number - 2) + fibonacci_calculator(nth_number - 1)
    }
}
