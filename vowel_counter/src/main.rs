use std::io;

fn vowel_counter(sentence: &String) -> i32{
    let mut result: i32 = 0;
    let vowels: [char; 5] = ['a','e','i','o','u'];
    let sentence: String = sentence.trim().to_lowercase();
    for chars in sentence.chars(){
        if vowels.contains(&chars){
            result += 1;
        }
    }
    println!("{}",result);
    result
}



fn consonants_counter(sentence: &String) -> i32{
    let sentence = sentence.trim().to_lowercase();
    let vowel_count: i32 = vowel_counter(&sentence);
    let result: i32 = sentence.len() as i32 - vowel_count;
    result
}

fn main() {
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("need to be a word or sentence");
    println!("number of vowels {} and consonants {}",vowel_counter(&sentence),consonants_counter(&sentence));
}
