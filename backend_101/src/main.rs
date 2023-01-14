use std::{io};

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    println!("Please enter your name");
    io::stdin().read_line(&mut name).expect("Failed to read Input");
    println!("Please enter your age");
    io::stdin().read_line(&mut age).expect("Failed to read Input");
    let age: i32 = age.trim().parse().expect("Please type a Number");
    println!("Hello, {}! you are {} years old.", name.trim(),age);
}
