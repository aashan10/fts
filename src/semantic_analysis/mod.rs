mod n_grams;
mod stop_words;
mod root_words;

use itertools::Itertools;
use unidecode::unidecode;
pub fn analyse(sentence: &String) -> Vec<String>
{
    let cleaned_sentence: String = sentence.chars().filter(|&c| c.is_alphanumeric() || c.is_whitespace()).collect();
    let words: Vec<String> = unidecode(&cleaned_sentence)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let without_stop_words: Vec<String> = stop_words::remove(&words);
    let root_words = root_words::collect(&without_stop_words);
    root_words.into_iter().unique().map(|s| s.to_lowercase()).collect()
}