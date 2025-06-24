use args::Args;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Command: {}", args.command);
    let filepath = args.file_path;
    println!("Filepath: {}", filepath);

    if let Some(message) = args.message {
        println!("Message: {}", message)
    }

    Ok(())
}
