use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use crate::helpers;
use crate::indexing::Index;

pub struct IndexManager {
    indices: HashMap<String, Index>,
    metadata: IndexMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct IndexMetadata {
    files: HashMap<String, String>,
}

// impl IndexManager {
//     pub fn initialize() -> IndexManager
//     {
//         let mut config_directory = helpers::get_config_directory().clone().to_owned();
//         let indices_directory = config_directory + "/indices";
//         let metadata = indices_directory + "/indices.json";
//         let mut metadata_contents = "".to_string();
//         match File::open(metadata).unwrap() {
//             (mut f) => {
//                 f.read_to_string(&mut metadata_contents).unwrap();
//             }
//             None => metadata_contents = "{\"files\": []}".to_string()
//         }
//
//         let metadata: IndexMetadata = serde_json::from_str(metadata_contents.as_str()).unwrap();
//         let mut indices: HashMap<String, Index> = HashMap::new();
//         for (key, value) in metadata.files {
//             let index = IndexManager::get_index(value);
//             match index.unwrap() {
//                 index => indices.insert(key, index),
//                 None => {}
//             };
//         }
//
//         IndexManager {
//             metadata,
//             indices
//         }
//     }
//
//     fn get_index(file: String) -> Option<Index>
//     {
//         let mut config_directory = helpers::get_config_directory().clone().to_owned();
//         let indices_directory = config_directory + "/indices";
//         match File::open(indices_directory + "/" + file.as_str()).expect("File indices.json not found!") {
//             (mut file) => {
//
//                 let mut contents = "".to_string();
//                 file.read_to_string(&mut contents).unwrap();
//                 let index: Index = serde_json::from_str(&contents).unwrap();
//                 Some(index)
//             }
//         }
//     }
// }