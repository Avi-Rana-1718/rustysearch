use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::models::Document;

pub struct DatabaseEngine {
    pub index: HashMap<String, HashSet<String>>,
    pub document_store: HashMap<String, Document>,
}

impl DatabaseEngine {
    pub fn new() -> Self {
        DatabaseEngine {
            index: HashMap::new(),
            document_store: HashMap::new(),
        }
    }
    pub fn store(&self, n_data: String) {
        self.tokenize(n_data);
        // println!("{}", format!("Inserted data: {}", nData));
    }

    pub fn search(&self, key: String) {
        println!("Searhed")
    }

    pub fn tokenize(&self, input: String) -> HashSet<&str> {
        let re = Regex::new(r"[ ,\.]+").unwrap();
        let tokens: HashSet<&str> = re.split(&input).collect();
        return tokens;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize() {
        let db = DatabaseEngine::new();
        let tokens = db.tokenize(String::from("Hello, world. How are you?"));

        assert!(tokens.contains("Hello"));
        assert!(tokens.contains("world"));
        assert!(tokens.contains("How"));
        assert_eq!(tokens.len(), 5);
    }
}
