use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Command to do
    #[arg()]
    pub command: String,

    // Name of the PNG file
    #[arg()]
    pub file_path: String,

    // Message to encode
    #[arg()]
    pub message: Option<String>,
}
