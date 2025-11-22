use std::collections::{HashMap, HashSet};

use crate::engine::DatabaseEngine;

mod engine;
mod models;

fn main() {
    let db: DatabaseEngine = DatabaseEngine::new();
}
