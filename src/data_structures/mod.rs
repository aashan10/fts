use std::collections::{HashMap};
use uuid::{Uuid};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::semantic_analysis;
#[derive(Debug, Serialize, Deserialize)]
pub struct InvertedIndex {
    pub(crate) documents: HashMap<String, HashMap<String, String>>,
    pub(crate) tokens: HashMap<String, Vec<String>>
}

impl InvertedIndex {
    // pub fn get_documents(&self, key: String) -> Option<HashMap<String, &HashMap<String, String>>>
    // {
    //
    //     let lowercase = key.to_lowercase();
    //     let keywords=  semantic_analysis::analyse(&lowercase);
    //     let mut entries: HashMap<String, &HashMap<String, String>> = HashMap::new();
    //     for keyword in keywords {
    //         if !self.tokens.contains_key(keyword.clone().as_str()) {
    //             return None;
    //         }
    //
    //         let keys = self.tokens.get(keyword.clone().as_str()).unwrap();
    //         for key in keys {
    //             if self.documents.contains_key(key.as_str()) {
    //                 let document = self.documents.get(key.as_str()).unwrap();
    //                 let id= document.get("_id").unwrap();
    //                 entries.insert(id.clone(), document);
    //             }
    //         }
    //     }
    //
    //     Some(entries)
    // }

    pub fn index(&mut self, record: &HashMap<String, String>) -> &InvertedIndex
    {
        let document_id: String = Uuid::new_v4().to_string();
        let mut cloned_record = record.clone();
        cloned_record.insert("_id".to_string(), document_id.clone());
        self.documents.insert(document_id.clone(), cloned_record);

        let mut tokens: Vec<String> = Vec::new();
        for (_key, value) in record.iter() {
            let words = semantic_analysis::analyse(value);
            for word in words {
                tokens.push(word);
            }
        }

        for token in &tokens {
            self.add_token(token, &document_id.clone());
        }

        return self
    }

    fn add_token(&mut self, token: &String, document_id: &String) -> &InvertedIndex
    {
        if self.tokens.contains_key(token.as_str()) {
            let mut documents = self.tokens.get(token.as_str()).unwrap().clone();
            documents.push(document_id.to_string());
            self.tokens.insert(token.to_string(), documents.into_iter().unique().collect());
        } else {
            self.tokens.insert(token.to_string(), Vec::from([document_id.to_string()]));
        }
        return self;
    }

    pub fn new() -> InvertedIndex
    {
        InvertedIndex {
            documents: HashMap::new(),
            tokens: HashMap::new()
        }
    }

    pub fn search(&self, query: String) -> HashMap<String, &HashMap<String, String>>
    {
        let mut results = HashMap::new();

        let parts = semantic_analysis::analyse(&query);

        for part in parts {
            let tokens = self.tokens.get(&part);
            match tokens {
                Some(tokens) => {
                    for token in tokens {
                        let document = self.documents.get(token);
                        match document {
                            Some(document) => {
                                let id = document.get("_id").unwrap();
                                if results.contains_key(id) {
                                    continue;
                                }
                                results.insert(id.clone(), document);
                            },
                            None => ()
                        };
                    }
                },
                None => ()
            };
        }

        return results;
    }
}