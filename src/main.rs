use crate::engine::DatabaseEngine;

mod engine;
mod models;
mod utils;

fn main() {
    let db: DatabaseEngine = DatabaseEngine::new();
    // let test = String::from("Hello, world. How are you?");
    // let tokens = db.tokenize(test);
    // println!("{:?}", tokens);
}
