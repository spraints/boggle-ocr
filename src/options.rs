use clap::{ArgEnum, Args, Parser, Subcommand};

// Best docs: https://github.com/clap-rs/clap/tree/v3.0.14/examples/tutorial_derive

pub fn parse() -> Commands {
    Cli::parse().command
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Find words in a 5x5 Boggle board.
    Boggle(BoggleOptions),

    /// Show possible wordle words.
    Wordle(WordleOptions),

    /// Summarize words in one or more Boggle boards.
    Summarize(SummarizeOptions),

    /// Compile a JSON dictionary.
    Compile(CompileOptions),
}

#[derive(Args)]
pub struct BoggleOptions {
    /// The JSON or compiled dictionary to use. Defaults to cached.dict or DICT.json in the current directory.
    #[clap(short, long)]
    pub dict: Option<String>,

    /// The JSON dictionary to use. Defaults to DICT.json in the current directory.
    #[clap(long)]
    pub defs: Option<String>,

    /// The board as a text file, one line per row.
    pub board: String,
}

#[derive(Args)]
pub struct WordleOptions {
    /// The JSON or compiled dictionary to use. Defaults to cached.dict or OWL2.json in the current
    /// directory.
    #[clap(short, long)]
    pub dict: Option<String>,

    /// The letters you know are in the solution.
    #[clap(short, long)]
    pub include: Option<String>,

    /// The letters you know are not in the solution.
    #[clap(short, long)]
    pub exclude: Option<String>,

    /// The puzzle with green letters filled in, e.g. "--b--".
    pub pattern: Option<String>,
}

#[derive(Args)]
pub struct SummarizeOptions {
    /// The JSON or compiled dictionary to use. Defaults to cached.dict or OWL2.json in the current directory.
    #[clap(short, long)]
    pub dict: Option<String>,

    /// The board as a text file, one line per row.
    pub boards: Vec<String>,

    #[clap(arg_enum, short, long, default_value = "none")]
    pub sort: SortOrder,
}

#[derive(ArgEnum, Clone)]
pub enum SortOrder {
    /// Keep the items in the provided order.
    None,
    /// Sort by name.
    Name,
    /// Sort by number of words.
    Words,
    /// Sort by total score.
    Score,
}

#[derive(Args)]
pub struct CompileOptions {
    #[clap(short = 'f', long)]
    pub overwrite: bool,

    /// The input JSON file.
    pub input: String,

    /// The compiled output file.
    pub output: String,
}
