use std::io;
use rand::Rng;

fn is_prime(n: u64) -> bool {
//    let n = n as u32;
    let mut i: u64 = 0;
    let mut votes: Vec<String> = Vec::new();
    let mut rng = rand::thread_rng();
    while i < 5 {
        i += 1;
        let a = rng.gen_range(1..n) as u64;
        if a.pow((n - 1) as u32)  % n  == 1 {
            let _i = i as usize;
            votes.push("true".to_string());
        }
        }
    let mut yes: u64 = 0;
    for string in votes{
        if string == "true"{
            yes += 1;
        }
    }
    return if yes >= 3 {
        true
    } else {
        false
    }

    }
fn collections_of_primes(n: u64) -> Vec<u64>{
    let mut list_of_primes: Vec<u64>=Vec::new();
    for i in 2..n{
        println!("so far so good5");
        if is_prime(i) {
            println!("so far so good6");
            list_of_primes.push(i)
        }
        }
    println!("{:?}",list_of_primes);
    return list_of_primes;
}



fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error meesages");
    let number: u64 = number.trim().parse().expect("Error msg");
    println!("here is all the prime number between 0 and your number {:?}",collections_of_primes(number))
}
