mod manager;

use serde::{Deserialize, Serialize};
use crate::data_structures::InvertedIndex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub alias: String,
    pub index: InvertedIndex,
    pub mapping: IndexMapping,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexMapping {
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "keyword")]
    Keyword,
}

// impl Index {
//     pub fn new(name: &str, mapping: IndexMapping, alias: Some(&str)) -> Index {
//         Index {
//             name: name.to_string(),
//             alias: match alias {
//                 Some(alias) => alias.to_string(),
//                 None => name.to_string()
//             },
//             index: InvertedIndex::new(),
//             mapping,
//         }
//     }
// }