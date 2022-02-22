use serde::de::{Deserializer, MapAccess, Visitor};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::rc::Rc;

// DAWG based on https://jbp.dev/blog/dawg-basics.html
// and https://github.com/sile/rust-dawg

pub fn open() -> Result<Dictionary, Box<dyn Error>> {
    let j = read_to_string("OWL2.json")?;
    let mut de = serde_json::Deserializer::from_str(&j);
    let mut builder = DictionaryBuilder::new();
    let mut n = 0;
    for (word, _) in de.deserialize_map(OWLVisitor::new())? {
        builder.insert(word, n < 10);
        n += 1;
    }
    Ok(builder.into_dict())
}

type RcNodeBuilder = Rc<RefCell<NodeBuilder>>;

struct DictionaryBuilder {
    previous_word: Option<String>,
    root: RcNodeBuilder,
    minimized_nodes: HashMap<RcNodeBuilder, RcNodeBuilder>,
    unchecked_nodes: Vec<(RcNodeBuilder, char, RcNodeBuilder)>,
}

impl DictionaryBuilder {
    fn new() -> Self {
        Self {
            previous_word: None,
            root: RcNodeBuilder::new(),
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

        let mut node = match self.unchecked_nodes.last() {
            None => self.root.borrow_mut(),
            Some((_, _, x)) => x.borrow_mut(),
        };

        for letter in word.chars().skip(common_prefix) {
            let next_node = RcNodeBuilder::new();
            node.set_child(letter, next_node);
            self.unchecked_nodes.push((node, letter, next_node));
            node = next_node;
        }

        node.terminal = false;
        self.previous_word = Some(word);
    }

    fn into_dict(self) -> Dictionary {
        Dictionary {
            root: self.root.into_node(0),
        }
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

    fn minimize(&mut self, down_to: usize) {
        while self.unchecked_nodes.len() > down_to {
            let (parent, letter, child) = self.unchecked_nodes.pop().unwrap();
            match self.minimized_nodes.get(child) {
                Some(new_child) => parent.children.borrow_mut().set_child(letter, new_child),
                None => self.minimized_nodes.insert(child, child),
            };
        }
    }
}

struct NodeBuilder {
    terminal: bool,
    children: Vec<Option<RcNodeBuilder>>,
}

impl NodeBuilder {
    fn new() -> Self {
        Self {
            terminal: false,
            children: vec![None; 26],
        }
    }

    fn set_child(&mut self, letter: char, child: RcNodeBuilder) {
        panic!("todo");
    }

    fn into_node(self) -> Node {
        let (node, _) = self.finish(0);
        node
    }

    fn finish(self, mut n: usize) -> (Node, usize) {
        let id = n;
        let NodeBuilder { terminal, children } = self;
        let children = children
            .into_iter()
            .map(|c| match c {
                None => None,
                Some(c) => {
                    let (a, b) = c.into_inner().finish(n);
                    n = b;
                    Some(a)
                }
            })
            .collect();
        (
            Node {
                terminal,
                id,
                children,
            },
            n,
        )
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

// impl Node {
//     fn new() -> Self {
//         Self {
//             terminal: false,
//             id: 0,
//             children: vec![None; 26],
//         }
//     }
//
//     fn for_suffix<C: Iterator<Item = char>>(mut chars: C, debug: bool) -> Option<Self> {
//         match chars.next() {
//             None => None,
//             Some(c) => {
//                 let mut res = Self::new();
//                 match Self::for_suffix(chars, debug) {
//                     None => {
//                         if debug {
//                             println!("  suffix {}: terminal", c);
//                         }
//                         res.terminal = true
//                     }
//                     Some(n) => {
//                         if debug {
//                             println!("  suffix: {}: has more suffix", c);
//                         }
//                         res.set(c, n)
//                     }
//                 };
//                 Some(res)
//             }
//         }
//     }
//
//     fn set(&mut self, c: char, child: Self) {
//         let pos = c.to_lowercase().next().unwrap() as u8 - b'a';
//         assert!(pos >= 0 && pos < 26);
//         let pos = pos as usize;
//         assert!(self.children[pos].is_none());
//         self.children[pos] = Some(child);
//     }
// }

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
