use super::dictionary;
use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

pub fn find_all_in_file(path: &str) -> Result<(), Box<dyn Error>> {
    let dict = dictionary::open()?;

    let raw_board = read_to_string(path)?;
    let board = boggled(&raw_board)?;

    let words = find_words(&dict, &board);
    let total_score: usize = words.iter().map(|w| score(&w)).sum();
    println!("found {} words, {} points", words.len(), total_score);
    for (i, w) in words.into_iter().enumerate() {
        if i % 20 == 0 {
            println!("");
            println!("{}", raw_board);
        }
        println!("  {:2} {}", score(&w), w);
    }

    Ok(())
}

fn score(word: &str) -> usize {
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
    let mut res: Vec<String> = res.into_iter().map(|w| stringify_word(w)).collect();
    res.sort();
    res
}

fn stringify_word(nw: Vec<usize>) -> String {
    nw.into_iter()
        .map(|ch| dictionary::letter_for_pos(ch))
        .collect()
}

fn visit(
    pos: Pos,
    visited: Visited,
    board: &Board,
    node: &dictionary::Node,
    res: &mut HashSet<Vec<usize>>,
    scratch: &mut Vec<usize>,
) {
    let (i, j) = pos;
    let ch = board[i][j];
    if let Some(next_node) = node.lookup(ch) {
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

fn mark_visit(visited: Visited, pos: Pos) -> Visited {
    let bit = pos.0 * 5 + pos.1;
    visited | (1 << bit)
}

type Visited = u32;
type Pos = (usize, usize);
type Board = [[usize; 5]; 5];

fn boggled(raw: &str) -> Result<Board, WSError> {
    let mut res = [[255; 5]; 5];
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
            if *ch == 255 {
                return Err(WSError::InvalidBoard(format!(
                    "not enough letters on line {}",
                    i + 1
                )));
            }
        }
    }
    Ok(res)
}

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
