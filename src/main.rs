use std::{fs, path::PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
use interpreter_starter_rust::{Scanner, Token};

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

            for token in Scanner::new(&file_contents) {
                println!("{token}");
            }
            println!("{}", Token::EOF);
        }
    }
    Ok(())
}
