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
    /// Encodes a message into a file
    Encode {
        filepath: String,
        chunk: String,
        message: String,
    },
    /// Decodes a hidden message in the specified file
    Decode { filepath: String, chunk: String },
    /// Removs a message from a file, if it exists
    Remove { filepath: String, chunk: String },
    /// Prints the hidden message in a file, if it exists
    Print { filepath: String },
}
