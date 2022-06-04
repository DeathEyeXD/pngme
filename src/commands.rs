use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::str::FromStr;
use clap::{Args, Parser, Subcommand};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

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
    /// Encode a chunk with given chunk type and message into file
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
    output_file: Option<String>,
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

use crate::Error;
use crate::png::Png;

pub trait Command {
    fn execute_command(self) -> crate::Result<()>;
}

impl Command for EncodeArgs {
    fn execute_command(self) -> crate::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(self.file_path)?;

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let chunk_data = self.message.into_bytes();

        let chunk = Chunk::new(chunk_type, chunk_data);
        let mut png_buf = Vec::new();

        let len = file.read_to_end(&mut png_buf)?;

        let mut png = if len == 0 {
            Png::new()
        } else {
            Png::try_from(&png_buf[..])
                .map_err(|err| Error::from(format!("Invalid png file data ({})", err)))?
        };

        png.append_chunk(chunk);
        file.write_all(&png.as_bytes())?;

        /*let mut file = OpenOptions::new() // <- todo add option to dont check png file and just append chunk
            .write(true)
            .append(true)
            .open(self.file_path)?;

        let bytes = chunk.as_bytes();
        file.write_all(&bytes)?;*/

        Ok(())
    }
}
