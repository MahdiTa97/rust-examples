use std::process;

fn main() {
    let mut fahrenheit: String = String::new();
    let celsius: f64;
    println!("Please input the Fahrenheit.");

    std::io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = match fahrenheit.trim().parse::<f64>() {
        Ok(res) => {
            println!("your integer input: {}", res);
            res
        }
        Err(_) => {
            println!("this was not an integer: {}", fahrenheit);
            process::exit(1);
        }
    };

    celsius = ((fahrenheit - 32.0) * (5.0 / 9.0)).into();

    println!("The Degree IS: {}°C", celsius)
}
