use args::{Args, Commands};
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

    match args.cmd {
        Commands::Encode {
            filepath,
            chunk,
            message,
        } => {
            commands::encode(&filepath, &chunk, &message)?;
        }
        Commands::Decode { filepath, chunk } => {
            commands::decode(&filepath, &chunk)?;
        }
        Commands::Remove { filepath, chunk } => {
            commands::remove(&filepath, &chunk)?;
        }
        Commands::Print { filepath } => {
            commands::print(&filepath)?;
        }
    }

    Ok(())
}
