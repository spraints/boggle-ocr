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

    /// Run a web server.
    Serve(ServerOptions),
}

#[derive(Args)]
pub struct BoggleOptions {
    /// The JSON or compiled dictionary to use. Defaults to cached.dict or DICT.json in the current directory.
    #[clap(short, long)]
    pub dict: Option<String>,

    /// Use the given JSON dictionary to look up definitions. Defaults to DICT.json in the current directory.
    #[clap(long)]
    pub defs_dict: Option<String>,

    /// Show definitions for the best words.
    #[clap(long)]
    pub defs: bool,

    /// Show all matches, not just the first 20.
    #[clap(long)]
    pub show_all: bool,

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

#[derive(Args)]
pub struct ServerOptions {
    /// The address (default 127.0.0.1:0) where the server will listen.
    #[clap(long)]
    pub addr: Option<String>,

    /// The directory where static assets are found.
    #[clap(long)]
    pub assets: Option<String>,

    /// The cached.dict file (default cached.dict).
    #[clap(long)]
    pub dict: Option<String>,

    /// The full dictionary, including definitions (default DICT.json).
    #[clap(long)]
    pub defs: Option<String>,
}
