use clap::Parser;
use clighlighter::{Cli, TS, highlight};
use std::io;
use std::io::Write;
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let inf = std::fs::read_to_string(cli.in_file).expect("not utf8");
    let h = highlight(TS, &inf);
    if let Some(of_name) = cli.out_file {
        let mut o = std::fs::File::open(of_name)?;
        o.write_all(h.as_bytes())?;
        o.flush()?;
    } else {
        let mut o = io::stdout().lock();
        o.write_all(h.as_bytes())?;
        o.flush()?;
    }
    Ok(())
}
