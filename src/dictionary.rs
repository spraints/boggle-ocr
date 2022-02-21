use serde::de::{Deserializer, MapAccess, Visitor};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::rc::Rc;

// DAWG based on https://jbp.dev/blog/dawg-basics.html

pub fn open() -> Result<Dictionary, Box<dyn Error>> {
    let j = read_to_string("OWL2.json")?;
    let mut de = serde_json::Deserializer::from_str(&j);
    let mut builder = DictBuilder::new();
    let mut n = 0;
    for (word, _) in de.deserialize_map(OWLVisitor::new())? {
        builder.insert(word, n < 10);
        n += 1;
    }
    Ok(builder.into_dict())
}

struct DictBuilder {
    previous_word: Option<String>,
    root: Rc<Node>,
    minimized: HashMap<Rc<Node>, Rc<Node>>,
    unchecked_nodes: Vec<(Rc<Node>, char, Rc<Node>)>,
}

impl DictBuilder {
    fn new() -> Self {
        Self {
            previous_word: None,
            root: Rc::new(Node::new()),
            minimized: HashMap::new(),
            unchecked_nodes: vec![],
        }
    }

    fn insert(&mut self, word: String, debug: bool) {
        if debug {
            println!("inserting '{}'", word);
        }

        let common_prefix = self.common_prefix(&word);
        if debug {
            println!("  common prefix: {}", common_prefix);
        }
        self.minimize(common_prefix);

        let suffix = Node::for_suffix(word.chars().skip(common_prefix), debug);
        if debug {
            println!("todo: add suffix {} to the right thing", suffix.unwrap().id);
        }
        self.previous_word = Some(word);
    }

    fn into_dict(self) -> Dictionary {
        Dictionary { root: self.root }
    }

    fn common_prefix(&self, word: &str) -> usize {
        match &self.previous_word {
            None => 0,
            Some(w) => {
                let z = word.chars().zip(w.chars());
                let mut max_i = 0;
                for (i, (a, b)) in z.enumerate() {
                    if a != b {
                        return i;
                    }
                    max_i = i
                }
                return max_i + 1;
            }
        }
    }

    fn minimize(&mut self, common_prefix: usize) {
        // todo
    }
}

pub struct Dictionary {
    root: Node,
}

#[derive(Clone)]
struct Node {
    terminal: bool,
    id: usize,
    children: Vec<Option<Node>>,
}

const TODO_ID: usize = 0;

impl Node {
    fn new() -> Self {
        Self {
            terminal: false,
            id: 0,
            children: vec![None; 26],
        }
    }

    fn for_suffix<C: Iterator<Item = char>>(mut chars: C, debug: bool) -> Option<Self> {
        match chars.next() {
            None => None,
            Some(c) => {
                let mut res = Self::new();
                match Self::for_suffix(chars, debug) {
                    None => {
                        if debug {
                            println!("  suffix {}: terminal", c);
                        }
                        res.terminal = true
                    }
                    Some(n) => {
                        if debug {
                            println!("  suffix: {}: has more suffix", c);
                        }
                        res.set(c, n)
                    }
                };
                Some(res)
            }
        }
    }

    fn set(&mut self, c: char, child: Self) {
        let pos = c.to_lowercase().next().unwrap() as u8 - b'a';
        assert!(pos >= 0 && pos < 26);
        let pos = pos as usize;
        assert!(self.children[pos].is_none());
        self.children[pos] = Some(child);
    }
}

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
