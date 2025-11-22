use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

struct DatabaseEngine {
    data: HashMap<String, HashSet<String>>,
}

impl DatabaseEngine {
    fn store(&self, nData: String) {
        self.tokenize(nData);
        // println!("{}", format!("Inserted data: {}", nData));
    }

    fn search(&self, key: String) {
        println!("Searhed")
    }

    fn tokenize(&self, input: String) {
        let delimeters = ["\"", ","];
        let re = Regex::new(r"[ ,\.]+").unwrap();
        let tokens: HashSet<&str> = re.split(&input).collect();
        println!("{:#?}", tokens)
    }
}

fn main() {
    let mut storage: HashMap<String, HashSet<String>> = HashMap::new();
    let database: DatabaseEngine = DatabaseEngine { data: storage };
    database.store(String::from("Hello, world. How are you?"));
}
