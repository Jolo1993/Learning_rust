use std::io;

struct Person {
    age: i32,
    name: String,
}
impl Person {
    fn new(age: i32, name: String) -> Person{
        Person { name, age }
}
    fn age_in_sec(&self) -> i32{
        let age_sec: i32 = self.age * 31556952;
        age_sec
    }
    fn age_in_days(&self) -> i32{
        let age_days: i32 = self.age * 365;
        age_days
    }
    fn age_in_hours(&self) -> i32{
        let age_hour: i32 = self.age * 8766;
        age_hour
    }
}

fn main() {

    println!("Please enter name and age");
    let mut name_age = String::new();
    io::stdin().read_line(&mut name_age).expect("string and integer");

    let words: Vec<&str> = name_age.split_whitespace().collect();
    let name = String::from(words[0]);
    let age: i32 = words[1].trim().parse().expect("wrong entry this should have been a integer");
    let person = Person::new(age,name);
    println!("hi {} your age in the days {} hours {} sec {}",
             person.name,
             person.age_in_days(),
             person.age_in_hours(),
             person.age_in_sec());
}
