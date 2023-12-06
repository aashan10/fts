use std::collections::HashMap;
use std::time::Instant;
use crate::data_structures::InvertedIndex;

mod semantic_analysis;
mod indexing;
mod searching;
mod data_structures;
mod movies;
mod helpers;


fn main() {
    let mut movies = Vec::new();
    for movie in movies::load().movies {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("title".to_string(), movie.title);
        map.insert("title_english".to_string(), movie.title_english);
        map.insert("title_long".to_string(), movie.title_long);
        map.insert("summary".to_string(), movie.summary);
        map.insert("description_full".to_string(), movie.description_full);
        map.insert("synopsis".to_string(), movie.synopsis);
        movies.push(map);
    }

    let mut inverted_index = InvertedIndex::new();
    println!("Indexing {} documents", movies.len());
    let start_time = Instant::now();
    for movie in movies {
        inverted_index.index(&movie);
    }
    let end_time = start_time.elapsed();
    let elapsed_seconds = end_time.as_micros();
    let tokens = inverted_index.tokens.iter().len();
    println!("Indexing completed. Took {} micro seconds ({} seconds)", elapsed_seconds, end_time.as_secs());
    println!("Indexed {} tokens", tokens);

    let keyword = "mad monster party".to_string();
    let keywords = semantic_analysis::analyse(&keyword);

    for k in keywords {

        let documents = inverted_index.search(k);

        for (id, record) in documents.iter() {

            println!("{}: {:?}\n", id, record.get("title").unwrap());
        }
    }
}
