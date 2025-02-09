pub mod cli;
pub mod highlight;
pub(crate) mod shared;

pub use cli::Cli;
pub use highlight::{Highlight, highlight};
use std::io;
use std::ops::Deref;

pub fn highlight_input(
    Cli {
        in_file,
        out_file,
        highlighter,
    }: Cli,
) -> std::io::Result<()> {
    let inf = std::fs::read_to_string(in_file)?;
    let highlighter = get_highlighter(highlighter);
    let mut o: Box<dyn io::Write> = if let Some(of_name) = out_file {
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

pub fn get_highlighter(hc: cli::HighlighterChoice) -> Box<dyn Highlight> {
    match hc {
        cli::HighlighterChoice::TS => Box::new(highlight::ts::TS),
        cli::HighlighterChoice::Rust => Box::new(highlight::rust::Rust),
    }
}
