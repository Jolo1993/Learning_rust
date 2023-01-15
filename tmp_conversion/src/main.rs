use std::io;

fn convert_to_fahrenheit(n: f32) -> f32 {
      return (n * 1.8) + 32.0;
}


fn main() {
    let mut number = String::new();
    println!("please enter the degree in celsius");
    io::stdin().read_line(&mut number).expect("can't read input");
    let number: f32 = number.trim().parse().expect("can't parse input must be a number");
    println!("Temperature in F instead of C = {}",convert_to_fahrenheit(number));
}
