use std::io;

fn main() {
    let mut side_a = String::new();
    let mut side_b = String::new();
    println!("Please enter Sides");
    io::stdin().read_line(&mut side_a).expect("Failed to read Input");
    io::stdin().read_line(&mut side_b).expect("Failed to read Input");
    let side_a: i32 = side_a.trim().parse().expect("need to be a number");
    let side_b: i32 = side_b.trim().parse().expect("need to be a number");
    let area = side_a * side_b;
    if side_b == side_a {
        println!("This is not a rectangle but square\n but here is the result anyway {}", area)
    } else {
        println!("{}", area);
    }
}
