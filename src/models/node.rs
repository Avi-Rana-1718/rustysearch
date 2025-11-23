use std::collections::HashMap;

pub struct Node {
    pub children: HashMap<String, Node>,
    pub document_id: String,
    pub end_node: bool
}