use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(bin_name = "pngme")]
pub struct Args {
    // Command to do
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Encodes a message into a chunk from a specified file
    Encode {
        filepath: String,
        chunk: String,
        message: String,
    },
    /// Decodes and prints a hidden message in the specified file and chunk
    Decode { filepath: String, chunk: String },
    /// Removs a message from a file, if it exists
    Remove { filepath: String, chunk: String },
    /// Prints all the chunks of a given file
    Print { filepath: String },
}
