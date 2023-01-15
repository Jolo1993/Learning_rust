use std::io;

fn odd_or_even(n: i32) -> &'static str {
    return if n % 2 != 0 {
        "odd"
    } else {
        "even"
    }
}

fn main() {
    println!("please enter number");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input");
    let number: i32 = number.trim().parse().expect("Failed to parse entry,\n make sure its a valid number ");
    println!("the number is {}",odd_or_even(number));
}
