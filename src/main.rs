use ::clap::Parser;
use cli::Cli;

mod chunk;
mod chunk_type;
mod cli;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
