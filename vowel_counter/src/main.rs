use std::io;

fn sentence_prep(sentence: String) -> String{
    let words: Vec<&str>= sentence.split_whitespace().collect();
    let clean_sentence = words.join("");
    let mut clean_sentence= clean_sentence.to_string();
    clean_sentence.retain(|c| !c.is_numeric());
    clean_sentence
}

fn vowel_counter(sentence: &String) -> i32{
    let mut result_vowels: i32 = 0;
    let vowels: [char; 5] = ['a','e','i','o','u'];
    for chars in sentence.chars(){
        if vowels.contains(&chars){
            result_vowels += 1;
        }
    }
    result_vowels
}

fn consonants_counter(sentence: &String) -> i32{
    let vowel_count: i32 = vowel_counter(&sentence);
    let result: i32 = sentence.len() as i32 - vowel_count;
    result
}

fn main() {
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("need to be a word or sentence");
    let sentence: String = sentence_prep(sentence);
    println!("number of vowels {} and consonants {}",vowel_counter(&sentence),consonants_counter(&sentence));
}
