use clap::{Args, Parser, Subcommand};

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

    /// Compile a JSON dictionary.
    Compile(CompileOptions),

    /// Try my OCR code. This doesn't really do anything yet.
    OCR(OCROptions),
}

#[derive(Args)]
pub struct BoggleOptions {
    /// The JSON or compiled dictionary to use. Defaults to cached.dict or OWL2.json in the current directory.
    #[clap(short, long)]
    pub dict: Option<String>,

    /// The board as a text file, one line per row.
    pub board: String,
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
pub struct OCROptions {
    /// The input image file.
    pub input: String,
}
