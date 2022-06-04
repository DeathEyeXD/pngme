use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct CliArgs {
    #[clap(subcommand)]
    command: CommandType,
}

impl CliArgs {
    pub fn command(self) -> CommandType {
        self.command
    }
}

#[derive(Subcommand, Debug)]
pub enum CommandType {
    /// Encode a chunk with given chunk type and message into file (note: it checks whether file is a valid png)
    Encode(EncodeArgs),
    Decode(DecodeArgs),

    Remove(RemoveArgs),
    /// Print png file as bytes from given path
    Print(PrintArgs),
}

#[derive(Args, Debug)]
pub struct EncodeArgs {
    file_path: String,
    chunk_type: String,
    message: String,
    /// Outputs png file with encoded message to another file instead
    #[clap(short, long)]
    output_file: Option<String>,
    /// Don't validate png file provided
    #[clap(short, long, conflicts_with = "output-file")]
    no_check: bool,
}

#[derive(Args, Debug)]
pub struct DecodeArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    file_path: String,
    chunk_type: String,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    file_path: String,
}

use crate::png::Png;
use crate::Error;

pub trait Command {
    fn execute_command(self) -> crate::Result<()>;
}

impl Command for EncodeArgs {
    fn execute_command(self) -> crate::Result<()> {
        let redirect_output = self.output_file.is_none();

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let chunk_data = self.message.into_bytes();

        let chunk = Chunk::new(chunk_type, chunk_data);

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(redirect_output)
            .append(!self.no_check)
            .open(self.file_path)?;

        if !self.no_check {
            let mut png_buf = Vec::new();

            let len = file.read_to_end(&mut png_buf)?;

            let mut png = if len == 0 {
                Png::new()
            } else {
                Png::try_from(&png_buf[..])
                    .map_err(|err| Error::from(format!("Invalid png file data ({})", err)))?
            };

            if redirect_output {
                let to_file = self.output_file.unwrap();

                file = OpenOptions::new().write(true).create(true).open(to_file)?;
            }
            file.write_all(&png.as_bytes())?;

            png.append_chunk(chunk);
        } else {
            let bytes = chunk.as_bytes();
            file.write_all(&bytes)?;
        }

        Ok(())
    }
}
