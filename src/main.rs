use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;
use crate::indexing::{Field, FieldType, IndexMapping};

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

    let mut index_manager = indexing::manager::IndexManager::init();
    let movies_mapping = IndexMapping {
        fields: vec![
            Field {
                name: "title".to_string(),
                field_type: FieldType::Text,
            },
            Field {
                name: "title_english".to_string(),
                field_type: FieldType::Text
            },
            Field {
                name: "title_long".to_string(),
                field_type: FieldType::Text,
            },
            Field {
                name: "summary".to_string(),
                field_type: FieldType::Text
            },
            Field {
                name: "description_full".to_string(),
                field_type: FieldType::Text,
            },
            Field {
                name: "synopsis".to_string(),
                field_type: FieldType::Text
            },
        ]
    };

    let mut start_time = Instant::now();
    let mut movies_index = index_manager.create_index("movies".to_string(), movies_mapping).unwrap();

    println!("Took {} micro seconds to create index.", &start_time.elapsed().as_micros());
    // println!("Indexing..");
    // let length = &movies.len();
    // let mut start_time = Instant::now();
    // for movie in movies {
    //     println!("Indexing {}", &movie.get("title").unwrap());
    //     movies_index.index(movie).unwrap();
    // }
    // println!("\n\nIndexed {} movies. Took {} micro seconds", length, &start_time.elapsed().as_micros());

    let query = "something";
    let mut start_time = Instant::now();
    let results = movies_index.search(query.to_string());
    println!("\n\nSearched the movies index for `{}`. \nTook {} micro seconds. \n{} hits found.", query, &start_time.elapsed().as_micros(), &results.len());
    for (_, movie) in results {
        let id = movie.get("_id").unwrap();
        let title = movie.get("title").unwrap();

        println!("ID: {}\nTitle: {}", id, title);
    }

    // index_manager.save_index(movies_index);
}
