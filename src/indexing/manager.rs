use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use crate::helpers::get_config_directory;
use crate::indexing::{Index, IndexMapping};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexMetaData {
    name: String,
    aliases: Vec<String>
}

impl IndexMetaData {
    pub fn new(name: String) -> IndexMetaData
    {
        IndexMetaData {
            name,
            aliases: vec![],
        }
    }

    pub fn set_alias(&mut self, alias: String) -> &IndexMetaData {
        self.aliases.push(alias);
        return self;
    }

    pub fn remove_alias(&mut self, alias: String) -> &IndexMetaData
    {
        self.aliases = self.aliases.iter().filter(|a| a != &&alias ).map(|a| a.to_string()).collect::<Vec<String>>();
        return self;
    }

    pub fn get_index(&self, mapping: IndexMapping) -> Index
    {
        let mut index = Index::new(self.name.as_str(), mapping);
        index.load();
        index
    }
}

pub struct IndexManager {
    pub metadata: BTreeMap<String, IndexMetaData>,
    indices: BTreeMap<String, Index>
}

impl IndexManager {
    pub fn init() -> IndexManager
    {
        let mut index_manager = IndexManager {
            metadata: BTreeMap::<String, IndexMetaData>::new(),
            indices: BTreeMap::<String, Index>::new(),
        };
        index_manager.load_metadata().unwrap();
        index_manager

    }

    pub fn save_metadata(&self) -> Result<bool, String>
    {
        let metadata_file = get_config_directory().to_string() + "/metadata.json";
        match File::create(metadata_file) {
            Ok(mut file) => {
                let mut json = serde_json::to_string(&self.metadata).unwrap();
                file.write_all(json.as_bytes()).unwrap();
                Ok(true)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    fn load_metadata(&mut self) -> Result<bool, String>
    {
        let metadata_file = get_config_directory().to_string() + "/metadata.json";
        match File::open(metadata_file) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string).unwrap();
                let metadata: BTreeMap<String, IndexMetaData> = serde_json::from_str(&string).unwrap();
                self.metadata = metadata;
                Ok(true)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn create_index(&mut self, name: String, mapping: IndexMapping) -> Option<Index>
    {
        let metadata = IndexMetaData {
            name: name.to_string(),
            aliases: vec![],
        };
        self.metadata.insert(name.to_string(), metadata);
        self.save_metadata().unwrap();
        match self.metadata.get(&name) {
            Some(m) => Some(m.get_index(mapping)),
            None => None
        }
    }

    pub fn get_index(&self, name: String) -> Option<&Index>
    {
        let mut index_name = name.to_string();
        for (key, metadata) in self.metadata.iter() {
            for alias in  &metadata.aliases {
                if alias == &name {
                    index_name = key.to_string();
                }
            }
        }

        if self.indices.contains_key(index_name.as_str()) {
            let index: Option<&Index> = self.indices.get(index_name.as_str());
            return match index {
                Some(i) => {
                    Some(i)
                },
                None => None
            }
        }


        None

    }

    pub fn save_index(&mut self, index: Index) -> Option<bool>
    {
        let name = &index.name.to_string();
        &index.save();
        if self.indices.contains_key(name) {
            self.indices.remove(name);
        }
        self.indices.insert(name.to_string(), index);

        Some(true)
    }
}