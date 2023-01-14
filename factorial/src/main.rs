use std::io;

fn factorial(n: i32) -> i32 {
    let mut result = 1;
    for i in 2..=n {
     result *= i;
    }
    result
}


fn main() {
    let mut number = String::new();
    println!("Please enter a number:");
    io::stdin().read_line(&mut number).expect("please input a valid string");
    let number: i32 = number.trim().parse().expect("failed to read input");

    println!("{}",factorial(number));
    }


