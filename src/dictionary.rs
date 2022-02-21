use serde::de::{Deserializer, MapAccess, Visitor};
use std::error::Error;
use std::fs::read_to_string;

// DAWG based on https://jbp.dev/blog/dawg-basics.html

pub fn open() -> Result<(), Box<dyn Error>> {
    let j = read_to_string("OWL2.json")?;
    let mut de = serde_json::Deserializer::from_str(&j);
    let v = OWLVisitor::new();
    let mut last_word = "".to_string();
    let mut last_def = vec![];
    let mut n = 0;
    for (word, definitions) in de.deserialize_map(v)? {
        n += 1;
        last_word = word;
        last_def = definitions;
    }
    println!("found {:?} words, {}: {:?}", n, last_word, last_def);
    Ok(())
}

/*
struct Node {
    terminal: bool,
    id: usize,
    children: Vec<Option<Node>>,
}
*/

struct OWLVisitor {}

impl OWLVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl<'de> Visitor<'de> for OWLVisitor {
    type Value = Vec<(String, Vec<String>)>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of word definitions")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut res = vec![];
        while let Some((key, value)) = access.next_entry()? {
            res.push((key, value));
        }
        Ok(res)
    }
}
