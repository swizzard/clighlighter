use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(help = "path to source file")]
    pub in_file: String,
    #[arg(help = "path to output file, defaults to STDOUT")]
    pub out_file: Option<String>,
    #[arg(
        short = 'l',
        value_name = "LANGUAGE",
        help = "highlighter to use, defaults to TS",
        default_value = "ts"
    )]
    pub highlighter: HighlighterChoice,
}

#[derive(Clone, Eq, PartialEq, ValueEnum)]
pub enum HighlighterChoice {
    /// Typescript
    TS,
}
