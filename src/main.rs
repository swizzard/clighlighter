use clap::Parser;
use clighlighter::get_highlighter;
use clighlighter::{Cli, highlight};
use std::io;
use std::io::Write;
use std::ops::Deref;
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let inf = std::fs::read_to_string(cli.in_file).expect("not utf8");
    let highlighter = get_highlighter(cli.highlighter);
    let h = highlight(highlighter.deref(), &inf);
    if let Some(of_name) = cli.out_file {
        let mut o = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .truncate(true)
            .open(of_name)?;
        o.write_all(h.as_bytes())?;
        o.flush()?;
    } else {
        let mut o = io::stdout().lock();
        o.write_all(h.as_bytes())?;
        o.flush()?;
    }
    Ok(())
}
