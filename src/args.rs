use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: CommandType,
}

trait Command {
    fn run(self) -> crate::Result<()>;
}

#[derive(Subcommand, Debug)]
enum CommandType {
    /// Encode a chunk with given chunk type and message into file
    Encode(EncodeArgs),
    Decode(DecodeArgs),

    Remove(RemoveArgs),
    /// Print png file as bytes from given path
    Print(PrintArgs),
}

#[derive(Args, Debug)]
struct EncodeArgs {
    file_path: String,
    chunk_type: String,
    message: String,
    output_file: Option<String>,
}

#[derive(Args, Debug)]
struct DecodeArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
struct RemoveArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
struct PrintArgs {
    file_path: String,
}
