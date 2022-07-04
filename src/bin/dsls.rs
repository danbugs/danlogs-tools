use std::fs::OpenOptions;

use anyhow::{Context, Result};
use clap::Parser;
use danlogs_tools::lexer_splitter::lex_and_split;

/// danlogs Script Lexer and Splitter (or, DSLS) is a tool for
/// lexing (i.e., breaking input into tokens) and then splitting
/// (i.e., between script and code files) the document I usually
/// write while preparing a video for my YouTube channel (danlogs).
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(short, long)]
    file_to_parse: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut file = OpenOptions::new()
        .read(true)
        .open(args.file_to_parse)
        .with_context(|| "failed to open file")?;
    lex_and_split(&mut file)?;
    Ok(())
}
