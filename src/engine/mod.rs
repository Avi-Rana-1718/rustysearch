use crate::engine::index::IndexEngine;
use crate::engine::storage::StorageEngine;

pub mod storage;
pub mod index;

pub struct DatabaseEngine {
    pub index: index::IndexEngine,
    pub storage: storage::StorageEngine
}

impl DatabaseEngine {
    pub fn new() -> Self {
        DatabaseEngine {
            index: IndexEngine::new(),
            storage: StorageEngine::new(),
        }
    }
    // pub fn store(&self, input: String) {
    //     let tokens: HashSet<String> = self.tokenize(input);
    //     let mut data = &self.document_store;

    //     // println!("{}", format!("Inserted data: {}", nData));
    // }

    // pub fn insert(&self, currNode: &mut Node, tokens: HashSet<String>, index: i32) {

    //     if tokens.len() >= index as usize {
    //         return;
    //     }

    //     let token = tokens.get(index);
    //     if currNode.children.contains_key(token) {
    //         self.insert(&currNode.children.get(token), tokens, index+1);
    //     } else {
    //         currNode.children.insert(token, None);
    //         self.insert(currNode.children.get(token), tokens, index);
    //     }
    // }

}

