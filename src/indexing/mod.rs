pub(crate) mod manager;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::data_structures::InvertedIndex;
use crate::helpers::get_config_directory;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Index {
    pub(crate) name: String,
    pub(crate) index: InvertedIndex,
    pub(crate) mapping: IndexMapping,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct IndexMapping {
    pub(crate) fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Field {
    pub(crate) name: String,
    #[serde(rename = "type")]
    pub(crate) field_type: FieldType,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum FieldType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "keyword")]
    Keyword,
}

impl Index {
    pub(crate) fn new(name: &str, mapping: IndexMapping) -> Index {
        Index {
            name: name.to_string(),
            index: InvertedIndex::new(),
            mapping,
        }
    }

    pub(crate) fn load(&mut self) -> &Index
    {
        let index_file = get_config_directory().to_string() + "/indices/" + self.name.as_str() + ".json";
        match File::open(index_file) {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();

                let index: InvertedIndex = serde_json::from_str(buffer.as_str()).unwrap();
                self.index = index;
            },
            Err(e) => {
                eprintln!("Index {} was not persisted: {}", self.name.as_str(), e);
            }
        }
        return self;
    }

    pub(crate) fn search(&self, query: String) -> HashMap<String, &HashMap<String, String>>
    {
        return self.index.search(query);
    }

    pub(crate) fn index(&mut self, document: HashMap<String, String>) -> Result<&Index, String>
    {
        let mut mapping: HashMap<String, String> = HashMap::new();
        for field in &self.mapping.fields {
            let key = field.name.to_string();
            if !document.contains_key(&key) {
                return Err(format!("Field {} does not exist in the data {}", field.name, serde_json::to_string(&document).unwrap()).to_string());
            } else {
                mapping.insert(key, document.get(field.name.as_str()).unwrap().to_string());
            }
        }
        self.index.index(&mapping);
        return Ok(self);
    }

    pub(crate) fn delete(&mut self, id: String) -> &Index
    {
        self.index.delete_document(id);
        return self;
    }
    pub(crate) fn save(&self) -> &Index
    {
        let name = self.name.as_str();
        let index_file = get_config_directory().to_string() + "/indices/" + name + ".json";
        match File::create(index_file.clone()) {
            Ok(mut file) => {
                let json = serde_json::to_string(&self.index);
                match json {
                    Ok(json) => {
                        file.write_all(json.as_bytes()).unwrap();
                        println!("Index persisted: {}", index_file.clone());
                    },
                    Err(err) => {
                        eprintln!("Error persisting index: {}", err);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error saving index to filesystem: {}", e);
            }
        };

        return self;
    }
}