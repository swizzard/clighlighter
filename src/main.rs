use clap::Parser;
use clighlighter::get_highlighter;
use clighlighter::{Cli, highlight};
use std::io;
use std::io::Write;
use std::ops::Deref;
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let inf = std::fs::read_to_string(cli.in_file)?;
    let highlighter = get_highlighter(cli.highlighter);
    let mut o: Box<dyn Write> = if let Some(of_name) = cli.out_file {
        Box::new(
            std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .truncate(true)
                .open(of_name)?,
        )
    } else {
        Box::new(io::stdout().lock())
    };
    highlight(highlighter.deref(), &inf, &mut o)
}
