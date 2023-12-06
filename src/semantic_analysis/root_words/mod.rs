use rust_stemmers::{Algorithm, Stemmer};

pub fn collect(input: &Vec<String>) -> Vec<String>
{
    let mut words: Vec<String> = Vec::new();

    for word in input {
        words.push(get_root_word(word));
    }

    words
}

pub fn get_root_word(input: &String) -> String
{
    let stemmer = Stemmer::create(Algorithm::English);
    stemmer.stem(input.as_str()).to_string()
}