use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

// DAWG based on https://jbp.dev/blog/dawg-basics.html

pub fn open() -> Result<(), Box<dyn Error>> {
    let j = read_to_string("OWL2.json")?;
    let map: HashMap<String, Vec<String>> = serde_json::from_str(&j)?;
    for (k, v) in &map {
        println!("read '{}' -> {:?}", k, v);
        break;
    }
    println!("read {} words", map.len());
    Ok(()) // todo
}

struct Node {
    terminal: bool,
    id: usize,
    children: Vec<Option<Node>>,
}
