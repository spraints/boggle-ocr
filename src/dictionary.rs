use serde::de::{Deserializer, MapAccess, Visitor};
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::rc::Rc;

// DAWG based on https://jbp.dev/blog/dawg-basics.html
// and https://github.com/sile/rust-dawg

const DEBUG: bool = false;

pub fn open() -> Result<Dictionary, Box<dyn Error>> {
    if DEBUG {
        let mut builder = DictionaryBuilder::new();
        builder.insert(String::from("cat"), true);
        builder.insert(String::from("cats"), true);
        builder.insert(String::from("fact"), true);
        builder.insert(String::from("facts"), true);
        builder.insert(String::from("facet"), true);
        builder.insert(String::from("facets"), true);
        let dict = builder.into_dict(true);
        dict.show_example_words();
    }
    // TODO - cache this to save ~ 0.5s
    let j = read_to_string("OWL2.json")?;
    let mut de = serde_json::Deserializer::from_str(&j);
    let mut builder = DictionaryBuilder::new();
    for (n, (word, _)) in de
        .deserialize_map(OWLVisitor::new())?
        .into_iter()
        .enumerate()
    {
        builder.insert(word, DEBUG && n < 10);
    }
    Ok(builder.into_dict(DEBUG))
}

struct DictionaryBuilder {
    previous_word: Option<String>,
    nodes: Vec<NodeBuilder>,
    unchecked: Vec<(usize, char, usize)>,
    minimized: HashMap<NodeBuilder, usize>,
    words: usize,
}

impl DictionaryBuilder {
    fn new() -> Self {
        Self {
            previous_word: None,
            nodes: vec![NodeBuilder::new()],
            unchecked: vec![],
            minimized: HashMap::new(),
            words: 0,
        }
    }

    fn insert(&mut self, word: String, debug: bool) {
        self.words += 1;

        if debug {
            println!("inserting '{}'", word);
        }

        let common_prefix = self.common_prefix(&word);
        if debug {
            println!("  common prefix: {}", common_prefix);
        }
        self.minimize(common_prefix, debug);

        let mut node_idx = match self.unchecked.last() {
            None => 0,
            Some((_, _, x)) => *x,
        };

        for letter in word.chars().skip(common_prefix) {
            if debug {
                println!("  adding a node for '{}'", letter);
            }
            let next_node_idx = self.nodes.len();
            self.nodes.push(NodeBuilder::new());
            self.nodes[node_idx].set_child(letter, next_node_idx);
            self.unchecked.push((node_idx, letter, next_node_idx));
            node_idx = next_node_idx;
        }

        self.nodes[node_idx].terminal = true;

        self.previous_word = Some(word);
    }

    fn into_dict(mut self, debug: bool) -> Dictionary {
        self.minimize(0, false);

        let sz1 = self.nodes.len();
        let mut nodes = HashMap::new(); // idx -> Rc<Node>
        let root = self.map(0, &mut nodes);
        if debug {
            println!(
                "generated {} nodes for {} words with {} intermediate nodes",
                nodes.len(),
                self.words,
                sz1
            );
        }
        Dictionary { root }
    }

    fn map(&self, idx: usize, nodes: &mut HashMap<usize, Rc<Node>>) -> Node {
        let mut node = Node::new();
        let nb = &self.nodes[idx];
        node.terminal = nb.terminal;
        node.id = idx;
        for (i, child_idx) in nb.children.iter().enumerate() {
            if let Some(child_idx) = child_idx {
                if let Some(child) = nodes.get(child_idx) {
                    node.children[i] = Some(child.clone());
                } else {
                    let child = Rc::new(self.map(*child_idx, nodes));
                    nodes.insert(*child_idx, child.clone());
                    node.children[i] = Some(child);
                }
            }
        }
        node
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
                max_i + 1
            }
        }
    }

    fn minimize(&mut self, down_to: usize, debug: bool) {
        while self.unchecked.len() > down_to {
            let (parent_idx, letter, child_idx) = self.unchecked.pop().unwrap();
            match self.minimized.get(&self.nodes[child_idx]) {
                Some(new_child_idx) => {
                    if debug {
                        println!(
                            "  - minimizing '{}': {} {} => {} {}",
                            letter,
                            child_idx,
                            self.describe(child_idx),
                            *new_child_idx,
                            self.describe(*new_child_idx)
                        );
                    }
                    self.nodes[parent_idx].set_child(letter, *new_child_idx);
                }
                None => {
                    if debug {
                        println!(
                            "  - '{}' ({} {}) is already minimized",
                            letter,
                            child_idx,
                            self.describe(child_idx)
                        );
                    }
                    self.minimized
                        .insert(self.nodes[child_idx].clone(), child_idx);
                }
            };
        }
    }

    fn describe(&self, idx: usize) -> String {
        let mut res = String::from("[ ");
        let node = &self.nodes[idx];
        for (pos, oidx) in node.children.iter().enumerate() {
            if let Some(idx) = oidx {
                res.push(letter_for_pos(pos));
                if node.terminal {
                    res.push('!');
                }
                res.push('>');
                res.push_str(&self.describe(*idx));
                res.push(' ');
            }
        }
        res.push(']');
        res
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct NodeBuilder {
    terminal: bool,
    children: Vec<Option<usize>>,
}

impl NodeBuilder {
    fn new() -> Self {
        Self {
            terminal: false,
            children: vec![None; 26],
        }
    }

    fn set_child(&mut self, letter: char, child_idx: usize) {
        self.children[letter_pos(letter)] = Some(child_idx);
    }
}

pub struct Dictionary {
    pub root: Node,
}

impl Dictionary {
    fn show_example_words(&self) {
        for w in make_some_words(10, &self.root) {
            println!("example word: {}", w);
        }
    }
}

fn make_some_words(n: usize, node: &Node) -> Vec<String> {
    let mut res = vec![];
    if node.terminal {
        res.push(String::from(""));
    }
    if res.len() >= n {
        return res;
    }
    for (i, child) in node.children.iter().enumerate() {
        if let Some(child) = child {
            for w in make_some_words(n - res.len(), child) {
                res.push(format!("{}{}", letter_for_pos(i), w));
                if res.len() >= n {
                    return res;
                }
            }
        }
    }
    res
}

pub struct Node {
    pub terminal: bool,
    id: usize,
    children: Vec<Option<Rc<Node>>>,
}

const Q: usize = 16;
const U: usize = 20;

impl Node {
    fn new() -> Self {
        Self {
            terminal: false,
            id: 0,
            children: vec![None; 26],
        }
    }

    pub fn lookup(&self, ch: usize) -> Option<&Node> {
        match self.children.get(ch) {
            Some(Some(rc_node)) => {
                if ch == Q {
                    rc_node.lookup(U)
                } else {
                    Some(rc_node)
                }
            }
            _ => None,
        }
    }
}

pub fn letter_pos(letter: char) -> usize {
    let pos = letter.to_lowercase().next().unwrap() as u8 - b'a';
    assert!(pos < 26);
    pos as usize
}

pub fn letter_for_pos(pos: usize) -> char {
    assert!(pos < 26);
    (b'a' + pos as u8) as char
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
