use std::collections::HashSet;

use regex::Regex;

pub fn tokenize(input: String) -> HashSet<String> {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    re.split(&input)
        .filter(|s| !s.is_empty()) // filter empty tokens
        .map(|s| s.to_string().to_lowercase())
        .collect()
}
