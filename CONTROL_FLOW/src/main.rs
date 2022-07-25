// ====== if Expressions ======

// fn main() {
//     let number = 3;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }

// ====== Repetition with Loops ======
// 3 kinds of loops:
// loop,
// while, and
// for.
// fn main() {
//     loop {
//         println!("again!");
//         // break;
//     }
// }

// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}")
// }

// ====== Loop Labels to Disambiguate Between Multiple Loops ======
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = true;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == false {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining = false;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// ====== Conditional Loops with while ======
// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }

//     println!("LEFTOFF!!!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

fn main() {
    // reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
