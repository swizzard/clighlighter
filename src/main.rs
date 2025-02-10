use clap::Parser;
use clighlighter::{Cli, highlight_input};
use std::io;
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    highlight_input(cli)
}
