pub mod cli;
pub mod highlight;
pub(crate) mod shared;

pub use cli::Cli;
#[cfg(feature = "explore")]
pub use highlight::explore::{Explore, ExploreAll};
pub use highlight::{Highlight, highlight};

pub fn get_highlighter(hc: cli::HighlighterChoice) -> Box<dyn Highlight> {
    match hc {
        cli::HighlighterChoice::TS => Box::new(highlight::ts::TS),
        cli::HighlighterChoice::Rust => Box::new(highlight::rust::Rust),
    }
}
