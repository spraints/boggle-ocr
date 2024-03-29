use serde::de::{Deserializer, MapAccess, Visitor};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

// DAWG based on https://jbp.dev/blog/dawg-basics.html
// and https://github.com/sile/rust-dawg

const JSON_DICT: &str = "DICT.json";
const DICT: &str = "cached.dict";
const DIR: &str = "/Users/spraints/src/github.com/spraints/boggle-ocr";

const DEBUG: bool = false;

pub type Definitions = HashMap<String, String>;

// In the Dictionary, each letter is represented by its offset from 'a'.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Letter(usize);

pub const Q: Letter = Letter(16);
pub const U: Letter = Letter(20);

impl Letter {
    pub fn new(ch: char) -> Self {
        letter_pos(ch)
    }

    pub fn for_i(i: usize) -> Self {
        Self(i)
    }

    pub fn empty() -> Self {
        Self(255)
    }

    pub fn is_empty(&self) -> bool {
        self.0 >= 26
    }

    pub fn i(&self) -> usize {
        self.0
    }

    pub fn ch(&self) -> char {
        letter_for_pos(*self)
    }
}

pub fn l(ch: char) -> Letter {
    Letter::new(ch)
}

impl std::fmt::Debug for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Letter({})",
            if self.is_empty() { '-' } else { self.ch() }
        )
    }
}

pub fn open_defs(path: &Option<String>) -> Result<Definitions, Box<dyn Error>> {
    let path = match path {
        Some(ref p) => p,
        None => JSON_DICT,
    };
    open_defs_path(path)
}

pub fn open_defs_path(path: impl AsRef<Path>) -> Result<Definitions, Box<dyn Error>> {
    let f = File::open(path)?;
    let defs = serde_json::from_reader(f)?;
    Ok(defs)
}

pub fn open_json(path: &str) -> Result<(Dictionary, Definitions), Box<dyn Error>> {
    let j = magic_read_to_string(path)?;
    parse_dict(&j)
}

pub fn parse_dict(j: &str) -> Result<(Dictionary, Definitions), Box<dyn Error>> {
    let mut de = serde_json::Deserializer::from_str(j);
    let mut builder = DictionaryBuilder::new();
    let mut defs = HashMap::new();
    let map = de.deserialize_map(OWLVisitor::new())?;
    for (word, definitions) in map {
        builder.insert(&word, false);
        defs.insert(word, definitions);
    }
    Ok((builder.into_dict(false), defs))
}

pub fn open_magic(path: &Option<String>) -> Result<Dictionary, Box<dyn Error>> {
    let compile_path = match path {
        Some(ref p) => p,
        None => DICT,
    };
    if let Ok(dict) = read(compile_path) {
        return Ok(dict);
    }

    let json_path = match path {
        Some(ref p) => p,
        None => JSON_DICT,
    };

    let j = magic_read_to_string(json_path)?;
    let mut de = serde_json::Deserializer::from_str(&j);
    let mut builder = DictionaryBuilder::new();
    let map = de.deserialize_map(OWLVisitor::new())?;
    for (word, _) in map {
        builder.insert(&word, false);
    }

    Ok(builder.into_dict(DEBUG))
}

pub fn read(path: impl AsRef<Path>) -> Result<Dictionary, Box<dyn Error>> {
    let f = match File::open(&path) {
        Ok(f) => f,
        Err(orig_err) => File::open(other_path(path)).or(Err(orig_err))?,
    };
    let mut f = BufReader::new(f);
    Dictionary::from(&mut f)
}

const REPORT_TIME: bool = true;

pub fn report_time(label: &str, t: std::time::Instant) {
    if REPORT_TIME {
        println!("{}: {:.2?}", label, t.elapsed());
    }
}

pub fn build_dictionary(mut words: Vec<&str>) -> Dictionary {
    words.sort();
    let mut db = DictionaryBuilder::new();
    for word in words {
        db.insert(word, false);
    }
    db.into_dict(false)
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

    fn insert(&mut self, word: &str, debug: bool) {
        self.words += 1;

        if debug {
            println!("inserting '{}'", word);
        }

        let common_prefix = self.common_prefix(word);
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

        self.previous_word = Some(String::from(word));
    }

    fn into_dict(mut self, debug: bool) -> Dictionary {
        self.minimize(0, false);

        let sz1 = self.nodes.len();
        let mut nodes = HashMap::new(); // idx -> Arc<Node>
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

    fn map(&self, idx: usize, nodes: &mut HashMap<usize, Arc<Node>>) -> Node {
        let mut node = Node::new();
        let nb = &self.nodes[idx];
        node.terminal = nb.terminal;
        node.id = idx;
        for (i, child_idx) in nb.children.iter().enumerate() {
            if let Some(child_idx) = child_idx {
                if let Some(child) = nodes.get(child_idx) {
                    node.children[i] = Some(child.clone());
                } else {
                    let child = Arc::new(self.map(*child_idx, nodes));
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
        for (i, oidx) in node.children.iter().enumerate() {
            let pos = Letter(i);
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
        self.children[letter_pos(letter).0] = Some(child_idx);
    }
}

#[derive(Clone)]
pub struct Dictionary {
    pub root: Node,
}

impl Dictionary {
    pub fn save<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        let mut written = HashSet::new();
        self.root.save(w, &mut written)
    }

    fn from<R: BufRead>(r: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut nodes = HashMap::new();
        let mut root = None;
        while let Some(node) = Node::from(r, &mut nodes)? {
            root = Some(node);
        }
        match root {
            None => Err(Box::new(DError::NoNodesInInput)),
            Some(node) => Ok(Self { root: node }),
        }
    }
}

#[derive(Clone)]
pub struct Node {
    pub terminal: bool,
    id: usize,
    children: Vec<Option<Arc<Node>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            terminal: false,
            id: 0,
            children: vec![None; 26],
        }
    }

    pub fn lookup(&self, ch: Letter) -> Option<&Node> {
        match self.children.get(ch.0) {
            Some(Some(rc_node)) => Some(rc_node),
            _ => None,
        }
    }

    fn save<W: std::io::Write>(&self, w: &mut W, seen: &mut HashSet<usize>) -> std::io::Result<()> {
        if !seen.insert(self.id) {
            return Ok(());
        }
        for child in self.children.iter().flatten() {
            child.save(w, seen)?;
        }
        write!(w, "[{}{}]", self.id, if self.terminal { "!" } else { "" })?;
        for (i, child) in self.children.iter().enumerate() {
            if let Some(child) = child {
                write!(w, " {}:{}", i, child.id)?;
            }
        }
        write!(w, ";")?;
        Ok(())
    }

    fn from<R: BufRead>(
        r: &mut R,
        nodes: &mut HashMap<usize, Arc<Node>>,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let mut data = vec![];
        let n = r.read_until(b';', &mut data)?;
        if n == 0 {
            return Ok(None);
        }
        //println!("READ '{}'", std::str::from_utf8(&data).unwrap());
        let mut res = Self::new();
        res.parse(data, nodes)?;
        nodes.insert(res.id, Arc::new(res.clone()));
        Ok(Some(res))
    }

    fn parse(
        &mut self,
        s: Vec<u8>,
        nodes: &mut HashMap<usize, Arc<Node>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut c = s.iter();
        match c.next() {
            Some(b'[') => (),
            _ => return Err(Box::new(DError::InvalidNode(s))),
        };

        loop {
            match c.next() {
                Some(b'0') => self.id *= 10,
                Some(b'1') => self.id = self.id * 10 + 1,
                Some(b'2') => self.id = self.id * 10 + 2,
                Some(b'3') => self.id = self.id * 10 + 3,
                Some(b'4') => self.id = self.id * 10 + 4,
                Some(b'5') => self.id = self.id * 10 + 5,
                Some(b'6') => self.id = self.id * 10 + 6,
                Some(b'7') => self.id = self.id * 10 + 7,
                Some(b'8') => self.id = self.id * 10 + 8,
                Some(b'9') => self.id = self.id * 10 + 9,
                Some(b'!') => {
                    self.terminal = true;
                    continue;
                }
                Some(b']') => {
                    break;
                }
                _ => return Err(Box::new(DError::InvalidNode(s))),
            };
        }

        match c.next() {
            Some(b';') => {
                return Ok(());
            }
            Some(b' ') => {}
            _ => return Err(Box::new(DError::InvalidNode(s))),
        };

        let mut st = NodeRefParseState::new();
        loop {
            match c.next() {
                Some(b'0') => {
                    st.push_digit(0);
                }
                Some(b'1') => {
                    st.push_digit(1);
                }
                Some(b'2') => {
                    st.push_digit(2);
                }
                Some(b'3') => {
                    st.push_digit(3);
                }
                Some(b'4') => {
                    st.push_digit(4);
                }
                Some(b'5') => {
                    st.push_digit(5);
                }
                Some(b'6') => {
                    st.push_digit(6);
                }
                Some(b'7') => {
                    st.push_digit(7);
                }
                Some(b'8') => {
                    st.push_digit(8);
                }
                Some(b'9') => {
                    st.push_digit(9);
                }
                Some(b':') => {
                    st.ch_done();
                }
                Some(b' ') => {
                    st.commit(self, nodes)?;
                    st = NodeRefParseState::new()
                }
                Some(b';') => {
                    st.commit(self, nodes)?;
                    break;
                }
                _ => return Err(Box::new(DError::InvalidNode(s))),
            }
        }

        Ok(())
    }
}

struct NodeRefParseState {
    reading_pos: bool,
    pos: usize,
    child_id: usize,
}

impl NodeRefParseState {
    fn new() -> Self {
        Self {
            reading_pos: true,
            pos: 0,
            child_id: 0,
        }
    }

    fn ch_done(&mut self) {
        // ch, pos, same diff
        self.reading_pos = false;
    }

    fn push_digit(&mut self, digit: usize) {
        if self.reading_pos {
            self.pos = self.pos * 10 + digit;
        } else {
            self.child_id = self.child_id * 10 + digit;
        }
    }

    fn commit(self, node: &mut Node, nodes: &HashMap<usize, Arc<Node>>) -> Result<(), DError> {
        match nodes.get(&self.child_id) {
            Some(child) => {
                node.children[self.pos] = Some(child.clone());
                Ok(())
            }
            None => Err(DError::DanglingPointer(self.child_id)),
        }
    }
}

pub fn try_letter_pos(letter: char) -> Option<Letter> {
    letter.to_lowercase().next().and_then(|ch| {
        let pos = ch as u8 - b'a';
        if pos < 26 {
            Some(Letter(pos as usize))
        } else {
            None
        }
    })
}

pub fn letter_pos(letter: char) -> Letter {
    match try_letter_pos(letter) {
        Some(l) => l,
        None => unreachable!("can't get letter_pos for {letter}"),
    }
}

pub fn letter_for_pos(pos: Letter) -> char {
    let Letter(pos) = pos;
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
    type Value = Vec<(String, String)>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map of word definitions")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut res = vec![];
        while let Some((key, value)) = access.next_entry()? {
            if key != "__VERSION__" {
                res.push((key, value));
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::Letter;

    fn make_test_dictionary(debug: bool) -> super::Dictionary {
        let mut builder = super::DictionaryBuilder::new();
        builder.insert("cat", debug);
        builder.insert("cats", debug);
        builder.insert("fact", debug);
        builder.insert("facts", debug);
        builder.insert("facet", debug);
        builder.insert("facets", debug);
        builder.into_dict(debug)
    }

    fn check_test_words(dict: &super::Dictionary) {
        let mut words = make_some_words(10, &dict.root);
        words.sort();
        assert_eq!(
            vec!["cat", "cats", "facet", "facets", "fact", "facts"],
            words
        );
    }

    // Make the test words into a dict like I would get from the website.
    const TEST_DICT: &'static str = r#"
      {
        "__VERSION__": "anything",
        "cat": "mrow",
        "cats": "mrow mrow",
        "fact": "the truth",
        "facts": "josiah bounderby",
        "facet": "one way",
        "facets": "many ways"
      }
    "#;

    #[test]
    fn example() {
        let dict = make_test_dictionary(true);
        check_test_words(&dict);
    }

    #[test]
    fn rw_dict() {
        let dict = make_test_dictionary(false);

        let mut w = Vec::new();
        dict.save(&mut w).unwrap();

        let mut r = w.as_slice();
        let dict = super::Dictionary::from(&mut r).unwrap();

        check_test_words(&dict);
    }

    #[test]
    fn open_dict_js() {
        let (dict, defs) = super::parse_dict(TEST_DICT).unwrap();
        check_test_words(&dict);
        assert_eq!(
            super::Definitions::from([
                ("cat".to_owned(), "mrow".to_owned()),
                ("cats".to_owned(), "mrow mrow".to_owned()),
                ("fact".to_owned(), "the truth".to_owned()),
                ("facts".to_owned(), "josiah bounderby".to_owned()),
                ("facet".to_owned(), "one way".to_owned()),
                ("facets".to_owned(), "many ways".to_owned()),
            ]),
            defs
        );
    }

    fn make_some_words(n: usize, node: &super::Node) -> Vec<String> {
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
                    res.push(format!("{}{}", super::letter_for_pos(Letter::for_i(i)), w));
                    if res.len() >= n {
                        return res;
                    }
                }
            }
        }
        res
    }
}

fn magic_read_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    match read_to_string(&path) {
        Ok(f) => Ok(f),
        Err(orig_err) => read_to_string(other_path(path)).or(Err(orig_err)),
    }
}

fn other_path<P: AsRef<Path>>(path: P) -> std::path::PathBuf {
    Path::new(DIR).join(path)
}

// TODO - use thiserror
#[derive(Debug)]
enum DError {
    NoNodesInInput,
    InvalidNode(Vec<u8>),
    DanglingPointer(usize),
}

impl std::fmt::Display for DError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DError::NoNodesInInput => write!(fmt, "no nodes in input"),
            DError::InvalidNode(s) => {
                write!(
                    fmt,
                    "invalid node '{}'",
                    std::str::from_utf8(s).unwrap_or("(unprintable)"),
                )
            }
            DError::DanglingPointer(child_id) => {
                write!(fmt, "node has dangling pointer to {}", child_id)
            }
        }
    }
}

impl std::error::Error for DError {}
