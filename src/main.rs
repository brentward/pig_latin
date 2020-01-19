use std::io;

fn main() {
    loop {
        let mut english_sentence = String::new();
        println!("English to Pig Latin:");
        io::stdin().read_line(&mut english_sentence)
            .expect("Failed to read line");
        english_sentence.pop();
        
        let mut pig_latin_sentence = String::new();
        for word in english_sentence.split(" ") {
            if word.len() > 0 {
                pig_latin_sentence += &format!("{} ", to_pig_latin(word));
            };
        };
        
        println!("{}", pig_latin_sentence);
    }

}

fn to_pig_latin(word: &str) -> String {
    let mut ending: String = "".to_string();
    let mut word_vec: Vec<char> = word.chars().collect();
    let uppercase_word: bool;
    let punctuation: Option<char>;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let punctuations = ['.', ',', '?', '!', ';', ':' ];
    if word_vec[0].is_uppercase() {
        word_vec[0] = word_vec[0].to_lowercase().collect::<Vec<char>>()[0];
        uppercase_word = true;
    } else {
        uppercase_word = false;
    };
    if punctuations.contains(&word_vec[word_vec.len() - 1]) {
        punctuation = Some(word_vec.remove(&word_vec.len() - 1));
    } else {
        punctuation = None;
    };
    if vowels.contains(&word_vec[0]) {
        ending = "hay".to_string();
    } else {
        ending.push(word_vec.remove(0));
        ending += "ay";
    };
    if uppercase_word {
        word_vec[0] = word_vec[0].to_uppercase().collect::<Vec<char>>()[0];
    }    
    let begining: String = word_vec.into_iter().collect();
    let pig_latin_word: String;
    if let Some(punctuation_mark) = punctuation {
        pig_latin_word = format!("{}{}{}", begining, ending, punctuation_mark.to_string());
    } else {
        pig_latin_word = format!("{}{}", begining, ending);
    };
    pig_latin_word
}
