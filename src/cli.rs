use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub in_file: String,
    pub out_file: Option<String>,
}
