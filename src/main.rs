use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;

pub mod dictionary;
mod options;
mod server;
mod wordsearch;

// detect dice: https://stackoverflow.com/questions/55169645/square-detection-in-image
// opencv rust: https://docs.rs/opencv/0.62.0/opencv/index.html

fn main() {
    use options::Commands::*;
    if let Err(err) = match options::parse() {
        Boggle(opts) => boggle(opts),
        Summarize(opts) => summarize(opts),
        Compile(opts) => compile(opts),
        Server(opts) => serve(opts),
    } {
        println!("error: {}", err);
        std::process::exit(1);
    }
}

type Res = Result<(), Box<dyn std::error::Error>>;

fn boggle(opts: options::BoggleOptions) -> Res {
    let dict = dictionary::open_magic(opts.dict)?;
    wordsearch::find_all_in_file(&opts.board, dict)
}

fn summarize(opts: options::SummarizeOptions) -> Res {
    let dict = dictionary::open_magic(opts.dict)?;
    let mut total_words = 0;
    let mut total_score = 0;
    let mut scores = Vec::new();
    for board in opts.boards {
        match summarize_board(&board, &dict) {
            Ok((msg, words, score)) => {
                total_words += words;
                total_score += score;
                scores.push((board, msg, words, score));
            }
            Err(err) => println!("{}: {}", board, err),
        };
    }
    match opts.sort {
        options::SortOrder::None => (),
        options::SortOrder::Name => {
            scores.sort_by(|(a, _, _, _), (b, _, _, _)| a.partial_cmp(b).unwrap());
        }
        options::SortOrder::Words => {
            scores.sort_by(|(_, _, a, _), (_, _, b, _)| a.partial_cmp(b).unwrap());
        }
        options::SortOrder::Score => {
            scores.sort_by(|(_, _, _, a), (_, _, _, b)| a.partial_cmp(b).unwrap());
        }
    };
    for (board, msg, _, _) in &scores {
        println!("{}: {}", board, msg);
    }
    println!(
        "avg words: {}, avg score: {}, {:.2} points per word",
        total_words / scores.len(),
        total_score / scores.len() as u32,
        total_score as f64 / total_words as f64
    );
    Ok(())
}

fn summarize_board(
    board: &str,
    dict: &dictionary::Dictionary,
) -> Result<(String, usize, u32), Box<dyn Error>> {
    let board = std::fs::read_to_string(board)?;
    let lines: Vec<&str> = board.lines().collect();
    let words = wordsearch::find_boggle_words(&lines, dict, 3);
    let total_words = words.len();
    let total_score: u32 = words.iter().map(|w| w.score).sum();
    let avg_score = total_score as f64 / total_words as f64;
    Ok((
        format!(
            "found {} words, {} points, {:.2} per word",
            total_words, total_score, avg_score
        ),
        total_words,
        total_score,
    ))
}

fn compile(opts: options::CompileOptions) -> Res {
    let mut fo = OpenOptions::new();
    fo.write(true).truncate(true);
    if opts.overwrite {
        fo.create(true);
    } else {
        fo.create_new(true);
    }
    let outf = match fo.open(&opts.output) {
        Ok(f) => f,
        Err(err) => match err.kind() {
            std::io::ErrorKind::AlreadyExists => {
                return Err(Box::new(GenericError(format!(
                    "{} already exists, use --overwrite to replace it",
                    opts.output
                ))))
            }
            _ => return Err(Box::new(GenericError(format!("{}: {}", opts.output, err)))),
        },
    };
    let mut outf = BufWriter::new(outf);
    let (dict, _) = dictionary::open_json(&opts.input)?;
    dict.save(&mut outf)?;
    Ok(())
}

fn serve(opts: options::ServerOptions) -> Result<(), Box<dyn Error>> {
    let (dict, defs) = dictionary::open_json(&opts.dict)?;
    server::serve(&opts.addr, dict, defs)
}

// TODO - use thiserror
#[derive(Debug)]
struct GenericError(String);

impl std::error::Error for GenericError {}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", &self.0)
    }
}
