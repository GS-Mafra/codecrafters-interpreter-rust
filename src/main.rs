use std::{
    fs,
    io::{stderr, stdout},
    path::PathBuf,
};

use anyhow::Context;
use clap::{Parser, Subcommand};
use interpreter_starter_rust::Tokenizer;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Tokenize { filename: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    eprintln!("{args:#?}");

    match args.command {
        Command::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename)
                .with_context(|| format!("Failed to read file {filename:?}"))?;

            let mut stdout = stdout().lock();
            let mut stderr = stderr().lock();
            let mut tokenizer = Tokenizer::new(&file_contents, &mut stdout, &mut stderr);
            tokenizer.tokenize()?;
            std::process::exit(tokenizer.code);
        }
    }
    // Ok(())
}
