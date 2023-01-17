use std::io;

struct Rectangle{
    side_a: f64,
    side_b: f64,
}
impl Rectangle {
    fn new(side_a: f64, side_b: f64) -> Rectangle {
        Rectangle { side_a, side_b }
    }

    fn area(&self) -> f64 {
        return self.side_a * self.side_b;
    }

    fn perimeter(&self) -> f64 {
        return (self.side_a * (2 as f64)) + (self.side_b * (2 as f64));
    }
}


fn main() {
    let mut side_a= String::new();
    let mut side_b= String::new();
    println!("please submit the side A of the rectangle");
    io::stdin().read_line(&mut side_a).expect("TODO: panic message");
    let side_a: f64 = side_a.trim().parse().expect("TODO: panic message");
    println!("please submit the side B of the rectangle");
    io::stdin().read_line(&mut side_b).expect("TODO: panic message");
    let side_b: f64 = side_b.trim().parse().expect("TODO: panic message");
    let rectangle = Rectangle::new(side_a, side_b);
    println!("here is the perimeter {} and area {}",
    rectangle.perimeter(),
    rectangle.area())
}
