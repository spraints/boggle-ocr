use super::dictionary::{self, letter_for_pos, letter_pos, Dictionary, Letter};
use std::collections::HashSet;

pub fn run(clues: &[Clue], dict: &Dictionary) -> Vec<String> {
    let mut known = [None; 5];
    let mut include = vec![];
    let mut ok = [true; 26];
    for c in clues {
        match c {
            Clue::Gray(ch) => ok[letter_pos(*ch).i()] = false,
            Clue::Yellow(ch) => include.push(letter_pos(*ch)),
            Clue::Green(ch, i) => known[*i] = Some(letter_pos(*ch)),
        };
    }
    let mut res = HashSet::new();
    search(
        &mut res,
        &mut [Letter::empty(); 5],
        0,
        &known,
        &include,
        &ok,
        &dict.root,
    );
    res.into_iter().collect()
}

pub enum Clue {
    Gray(char),
    Yellow(char),
    Green(char, usize),
}

fn search(
    res: &mut HashSet<String>,
    work: &mut [Letter; 5],
    i: usize,
    known: &[Option<Letter>],
    include: &[Letter],
    ok: &[bool],
    node: &dictionary::Node,
) {
    if i == work.len() {
        if node.terminal {
            res.insert(work.iter().map(|l| letter_for_pos(*l)).collect());
        }
        return;
    }
    for l in letter_choices(&work[0..i], known[i], include, ok) {
        if let Some(n) = node.lookup(l) {
            work[i] = l;
            search(res, work, i + 1, known, include, ok, n);
        }
    }
}

fn letter_choices(
    s: &[Letter],
    known: Option<Letter>,
    include: &[Letter],
    ok: &[bool],
) -> Vec<Letter> {
    // If we already know what letter is in this place, use only it.
    if let Some(l) = known {
        return vec![l];
    }

    // If we have as many letters that we have to use as places left in the puzzle, just use the
    // 'include' list. This could be a little smarter and consider known letters in the rest of the
    // puzzle.
    let mut include: HashSet<Letter> = include.iter().cloned().collect();
    for l in s {
        include.remove(l);
    }
    if include.len() >= 5 - s.len() {
        return include.into_iter().collect();
    }

    // Use any letter that hasn't been eliminated.
    ok.iter()
        .enumerate()
        .filter(|(_, ok)| **ok)
        .map(|(i, _)| Letter::for_i(i))
        .collect()
}
