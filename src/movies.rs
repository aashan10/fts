use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub fn load() -> Movies
{
    let path = "/Users/aashan/Projects/fts/src/movies.json";
    let mut file = File::open(&path).unwrap();
    let mut buffer= "".to_string();
    file.read_to_string(&mut buffer).expect("");
    let json: Movies = serde_json::from_str(buffer.as_str()).unwrap();
    json
}

#[derive(Serialize, Deserialize)]
pub struct Movies {
    pub movies: Vec<Movie>
}

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub title_english: String,
    pub title_long: String,
    pub summary: String,
    pub description_full: String,
    pub synopsis: String
}