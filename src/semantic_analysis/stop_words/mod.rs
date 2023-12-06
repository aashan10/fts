pub fn remove(words: &Vec<String>) -> Vec<String>
{
    let stop_words: Vec<String> = Vec::from([
        "a".to_string(),
        "of".to_string()
    ]);

    let result: Vec<String> = words
        .iter()
        .filter(|word| {
            let lowercase = word.to_lowercase();
            !stop_words.contains(&lowercase)
        })
        .cloned()
        .collect();
    result
}