use super::dictionary::{self, Q, U};
use serde::Serialize;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

pub fn find_all_in_file(path: &str, dict: dictionary::Dictionary) -> Result<(), Box<dyn Error>> {
    let raw_board = read_to_string(path)?;
    let board = boggled(&raw_board)?;

    let t = std::time::Instant::now();
    let words = find_words(&dict, &board);
    dictionary::report_time("find_words", t);

    let total_score: u32 = words.iter().map(|w| score(w)).sum();
    println!("{}", raw_board);
    println!(
        "found {} words, {} points, {:.2} per word",
        words.len(),
        total_score,
        total_score as f32 / words.len() as f32,
    );
    println!("best words:");
    let mut scored_words: Vec<(u32, String)> = words.into_iter().map(|w| (score(&w), w)).collect();
    scored_words.sort_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
    for (s, w) in scored_words.into_iter().take(20) {
        println!("  {:2} {}", s, w);
    }

    Ok(())
}

pub fn find_boggle_words(
    board: &[&str],
    dict: &dictionary::Dictionary,
    min_length: usize,
) -> Vec<Word> {
    let board: AnyBoard = board.iter().map(|line| parse_board_line(line)).collect();
    let total_letters = board.iter().map(|line| line.len()).sum();
    let mut res = HashSet::new();
    let mut scratch = Vec::with_capacity(total_letters);
    let pos_keeper = PositionKeeper::new(&board);
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            let pos = (i, j);
            visit2(
                pos,
                pos_keeper.mark(0, pos),
                &pos_keeper,
                &board,
                &dict.root,
                &mut res,
                &mut scratch,
            );
        }
    }
    let mut res: Vec<Word> = res
        .into_iter()
        .map(Word::new)
        .filter(|w| w.word.len() >= min_length)
        .collect();
    res.sort();
    res
}

fn parse_board_line(line: &str) -> Vec<dictionary::Letter> {
    line.chars().map(dictionary::letter_pos).collect()
}

#[derive(Serialize)]
pub struct Word {
    pub word: String,
    pub score: u32,
}

impl Word {
    fn new(word: Vec<dictionary::Letter>) -> Self {
        let w = stringify_word(word);
        let s = score(&w);
        Self { word: w, score: s }
    }
}

impl PartialOrd<Word> for Word {
    fn partial_cmp(&self, other: &Word) -> Option<std::cmp::Ordering> {
        self.word.partial_cmp(&other.word)
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.word.cmp(&other.word)
    }
}

impl PartialEq<Word> for Word {
    fn eq(&self, other: &Word) -> bool {
        self.word.eq(&other.word)
    }
}

impl Eq for Word {}

fn score(word: &str) -> u32 {
    match word.len() {
        0 | 1 | 2 => 0,
        3 | 4 => 1,
        5 => 2,
        6 => 3,
        7 => 5,
        _ => 11,
    }
}

fn find_words(dict: &dictionary::Dictionary, board: &Board) -> Vec<String> {
    let mut res = HashSet::new();
    let mut scratch = Vec::with_capacity(25);
    for i in 0..5 {
        for j in 0..5 {
            let pos = (i, j);
            visit(
                pos,
                mark_visit(0, pos),
                board,
                &dict.root,
                &mut res,
                &mut scratch,
            );
        }
    }
    let mut res: Vec<String> = res.into_iter().map(stringify_word).collect();
    res.sort();
    res
}

fn stringify_word(nw: Vec<dictionary::Letter>) -> String {
    nw.into_iter()
        .flat_map(|ch| QU::new(dictionary::letter_for_pos(ch)))
        .collect()
}

struct QU {
    letter: char,
    pos: usize,
}

impl QU {
    fn new(letter: char) -> Self {
        Self { letter, pos: 0 }
    }
}

impl Iterator for QU {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            0 => {
                self.pos += 1;
                Some(self.letter)
            }
            1 => match self.letter {
                'q' => {
                    self.pos += 1;
                    Some('u')
                }
                _ => None,
            },
            _ => None,
        }
    }
}

struct PositionKeeper {
    offsets: Vec<usize>,
}

impl PositionKeeper {
    fn new(board: &AnyBoard) -> Self {
        let mut offsets = vec![0];
        let mut tot = 0;
        for line in board {
            tot += line.len();
            offsets.push(tot);
        }
        Self { offsets }
    }

    fn mark(&self, visited: Visited, pos: Pos) -> Visited {
        let (i, j) = pos;
        let bit = self.offsets[i] + j;
        visited | (1 << bit)
    }
}

fn visit2(
    pos: Pos,
    visited: Visited,
    pk: &PositionKeeper,
    board: &AnyBoard,
    node: &dictionary::Node,
    res: &mut HashSet<Vec<dictionary::Letter>>,
    scratch: &mut Vec<dictionary::Letter>,
) {
    let (i, j) = pos;
    let ch = board[i][j];
    if let Some(next_node) = lookup(node, ch) {
        scratch.push(ch);
        if next_node.terminal {
            res.insert(scratch.clone());
        }
        for di in -1..=1 {
            for dj in -1..=1 {
                let ni = di + i as isize;
                let nj = dj + j as isize;
                if out_of_bounds(board, ni, nj) {
                    //println!("({}, {}) -> SKIP ({}, {})", i, j, ni, nj);
                    continue;
                }
                //println!("({}, {}) -> ({}, {})", i, j, ni, nj);
                let npos = (ni as usize, nj as usize);
                let nvisited = pk.mark(visited, npos);
                if nvisited != visited {
                    visit2(npos, nvisited, pk, board, next_node, res, scratch);
                }
            }
        }
        scratch.pop();
    }
}

fn out_of_bounds(board: &AnyBoard, i: isize, j: isize) -> bool {
    if i < 0 {
        return true;
    }
    if j < 0 {
        return true;
    }
    let i = i as usize;
    if i < board.len() {
        let j = j as usize;
        if j < board[i].len() {
            return false;
        }
    }
    true
}

fn visit(
    pos: Pos,
    visited: Visited,
    board: &Board,
    node: &dictionary::Node,
    res: &mut HashSet<Vec<dictionary::Letter>>,
    scratch: &mut Vec<dictionary::Letter>,
) {
    let (i, j) = pos;
    let ch = board[i][j];
    if let Some(next_node) = lookup(node, ch) {
        scratch.push(ch);
        if next_node.terminal && scratch.len() > 2 {
            res.insert(scratch.clone());
        }
        for di in -1..=1 {
            for dj in -1..=1 {
                if di != 0 || dj != 0 {
                    let ni = di + i as isize;
                    let nj = dj + j as isize;
                    if ni >= 0 && nj >= 0 && ni < 5 && nj < 5 {
                        let npos = (ni as usize, nj as usize);
                        let nvisited = mark_visit(visited, npos);
                        if nvisited != visited {
                            visit(npos, nvisited, board, next_node, res, scratch);
                        }
                    }
                }
            }
        }
        scratch.pop();
    }
}

fn lookup(node: &dictionary::Node, ch: dictionary::Letter) -> Option<&dictionary::Node> {
    match node.lookup(ch) {
        Some(child) if ch == Q => child.lookup(U),
        res => res,
    }
}

fn mark_visit(visited: Visited, pos: Pos) -> Visited {
    let bit = pos.0 * 5 + pos.1;
    visited | (1 << bit)
}

type Visited = u32;
type Pos = (usize, usize);
type Board = [[dictionary::Letter; 5]; 5];
type AnyBoard = Vec<Vec<dictionary::Letter>>;

fn boggled(raw: &str) -> Result<Board, WSError> {
    let mut res = [[dictionary::Letter::empty(); 5]; 5];
    for (i, line) in raw.lines().enumerate() {
        if i > 4 {
            return Err(WSError::InvalidBoard(String::from(
                "too many lines in input",
            )));
        }
        for (j, ch) in line.chars().enumerate() {
            if j > 4 {
                return Err(WSError::InvalidBoard(format!(
                    "too many letters on line {}",
                    i + 1
                )));
            }
            res[i][j] = dictionary::letter_pos(ch);
        }
    }
    for (i, row) in res.iter().enumerate() {
        for ch in row {
            if ch.is_empty() {
                return Err(WSError::InvalidBoard(format!(
                    "not enough letters on line {}",
                    i + 1
                )));
            }
        }
    }
    Ok(res)
}

// TODO - use thiserror
#[derive(Debug)]
enum WSError {
    InvalidBoard(String),
}

impl std::fmt::Display for WSError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WSError::InvalidBoard(reason) => write!(fmt, "invalid board: {}", reason),
        }
    }
}

impl Error for WSError {}

#[cfg(test)]
mod test {
    use super::dictionary::build_dictionary;

    #[test]
    fn example() {
        let dict = build_dictionary(vec!["tenets", "facts", "honey"]);
        let res =
            super::find_boggle_words(&vec!["taeyl", "eohak", "yneit", "yteyl", "shaig"], &dict, 3);
        let words: Vec<String> = res.into_iter().map(|w| w.word).collect();
        assert_eq!(words, vec!["honey", "tenets"]);
    }

    #[test]
    fn example_with_implied_u_after_q() {
        let dict = build_dictionary(vec!["quit", "quick"]);
        let res = super::find_boggle_words(&vec!["qic", "xkk"], &dict, 3);
        let words: Vec<String> = res.into_iter().map(|w| w.word).collect();
        assert_eq!(words, vec!["quick"]);
    }
}
