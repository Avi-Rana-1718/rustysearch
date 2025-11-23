use std::collections::HashMap;
use crate::models::node::{Node};
use crate::utils::tokenize::tokenize;

pub struct IndexEngine {
    index: HashMap<String, Node>
}

impl IndexEngine {
    pub fn new()-> Self {
        IndexEngine { index: HashMap::new() }
    }

    fn search(&self, input: String) {
        let tokens = tokenize(input);
    }

    fn insert(self, input: String) {
        let tokens = tokenize(input);

    }
}