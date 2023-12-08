use std::collections::{HashMap, HashSet};
use uuid::{Uuid};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::semantic_analysis;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct InvertedIndex {
    pub(crate) documents: HashMap<String, HashMap<String, String>>,
    pub(crate) tokens: HashMap<String, Vec<String>>,
}

impl InvertedIndex {
    pub(crate) fn delete_document(&mut self, id: String) -> &InvertedIndex
    {
       return self;
    }

    pub(crate) fn index(&mut self, record: &HashMap<String, String>) -> &InvertedIndex
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

        return self;
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

    pub(crate) fn new() -> InvertedIndex
    {
        InvertedIndex {
            documents: HashMap::new(),
            tokens: HashMap::new(),
        }
    }

    pub(crate) fn search(&self, query: String) -> HashMap<String, &HashMap<String, String>>
    {
        let mut results = HashMap::new();
        let mut unique_documents = HashSet::new();

        let parts = semantic_analysis::analyse(&query);

        for part in parts {
            if let Some(tokens) = self.tokens.get(&part) {
                for token in tokens {
                    if let Some(document) = self.documents.get(token) {
                        let id = document.get("_id").unwrap();
                        if unique_documents.insert(id.to_string()) {
                            // Insert into results only if the document is unique
                            results.insert(id.to_string(), document);
                        }
                    }
                }
            }
        }

        results
    }
}