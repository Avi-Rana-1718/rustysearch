use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::models::document::Document;

pub mod storage;
pub mod index;

pub struct DatabaseEngine {
    pub index: HashMap<String, HashSet<String>>,
    pub document_store: Vec<Document>
}

impl DatabaseEngine {
    pub fn new() -> Self {
        DatabaseEngine {
            index: HashMap::new(),
            document_store: Vec::new(),
        }
    }
    pub fn store(&self, input: String) {
        let tokens: HashSet<String> = self.tokenize(input);
        let mut data = &self.document_store;

        // println!("{}", format!("Inserted data: {}", nData));
    }

    pub fn insert(&self, currNode: &mut Node, tokens: HashSet<String>, index: i32) {

        if tokens.len() >= index as usize {
            return;
        }

        let token = tokens.get(index);
        if currNode.children.contains_key(token) {
            self.insert(&currNode.children.get(token), tokens, index+1);
        } else {
            currNode.children.insert(token, None);
            self.insert(currNode.children.get(token), tokens, index);
        }
    }

    pub fn tokenize(&self, input: String) -> HashSet<String> {
        let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
        re.split(&input)
            .filter(|s| !s.is_empty()) // filter empty tokens
            .map(|s| s.to_string().to_lowercase())
            .collect()
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let db = DatabaseEngine::new();
        let test = String::from("Hello, world. How are you?");
        let tokens = db.tokenize(test);

        assert!(tokens.contains("Hello"));
        assert!(tokens.contains("world"));
        assert!(!tokens.contains("."));
        assert!(!tokens.contains("you?"));
        assert_eq!(tokens.len(), 5);
    }
}
