#[cfg(not(feature = "cli"))]
compile_error!("CLI not enabled");

use std::{
    fs::{read_to_string, write},
    path::PathBuf,
    process::exit,
};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;
use iso_843::Iso843;

/// Convert Greek characters to Latin characters according to ISO 843:1997 Type 1.
#[derive(Clone, Debug, Parser)]
#[command(version)]
struct Command {
    /// Input file to read from.
    ///
    /// Exactly one from this and `TEXT` shall be specified.
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output file to write to.
    ///
    /// If not specified, would print to stdout.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Text to transliterate.
    ///
    /// Exactly one from this and `--input` shall be specified.
    #[arg(value_name = "TEXT")]
    pub text: Option<String>,
}

fn execute() -> anyhow::Result<()> {
    let cmd = Command::parse();

    let input = match (cmd.input, cmd.text) {
        (Some(path), None) => read_to_string(path)?,
        (Some(_), Some(_)) => {
            bail!("cannot decide which input to use: both `--input` and `TEXT` are specified")
        }
        (None, Some(text)) => text,
        (None, None) => bail!("missing input"),
    };

    let output = input.iso843_transliterate();

    if let Some(path) = cmd.output {
        write(path, output)?;
    } else {
        println!("{output}");
    }

    Ok(())
}

fn main() {
    match execute() {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{} {e}", "fatal:".bold().red());
            exit(-1)
        }
    }
}
