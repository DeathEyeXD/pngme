use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::{Args, Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::str::FromStr;

mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
    /// (note: by default it creates given file if it doesnt exists,
    /// but if it exists it checks whether file is a valid png)
    Encode(EncodeArgs),

    /// Decode a secret message encoded in png file
    Decode(DecodeArgs),

    /// Remove (and decode) first secret message found with given chunk type encoded in png file
    /// (note: it deletes most-recent message first, and use -a flag to delete all matched messages)
    Remove(RemoveArgs),

    /// Print png file data as bytes from given path
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
    /// Remove all matched messages instead of first matching
    #[clap(short, long)]
    all: bool,
    /// Dont encode and output removed messages
    #[clap(short, long)]
    ignore_messages: bool,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    file_path: String,
}

pub fn execute_command(command: CommandType) -> Result<()> {
    match command {
        CommandType::Encode(args) => args.execute_command(),
        CommandType::Decode(args) => args.execute_command(),
        _ => Ok(()),
    }
}

pub fn run(args: CliArgs) -> Result<()> {
    let command = args.command;

    execute_command(command)
}

pub fn get_args() -> CliArgs {
    CliArgs::parse()
}

pub fn create_png_file_from_bytes(filename: &str, bytes: &[u8]) -> Result<()> {
    let png = Png::try_from(bytes)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    file.write_all(&png.as_bytes())?;

    Ok(())
}

pub fn write_byte_to_file(filename: &str, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    file.write_all(bytes)?;

    Ok(())
}

pub fn write_png_to_file(filename: &str, png: Png) -> Result<()> {
    write_byte_to_file(filename, &png.as_bytes())
}

pub trait Command {
    fn execute_command(self) -> Result<()>;

    fn get_png(bytes: &[u8]) -> Result<Png> {
        Png::try_from(bytes).map_err(|err| Error::from(format!("Invalid png file data ({})", err)))
    }

    fn open_file(path: &str, read: bool, write: bool, append: bool, create: bool) -> Result<File> {
        OpenOptions::new()
            .read(read)
            .write(write)
            .append(append)
            .create(create)
            .open(path)
            .map_err(|err| Error::from(format!("Cannot open file {}, cause: {}", path, err)))
    }
}

impl Command for EncodeArgs {
    fn execute_command(self) -> Result<()> {
        let redirect_output = self.output_file.is_some();

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let chunk_data = self.message.into_bytes();

        let chunk = Chunk::new(chunk_type, chunk_data);

        let mut file = Self::open_file(
            &self.file_path,
            true,
            true,
            !self.no_check,
            !redirect_output,
        )?;

        if !self.no_check {
            let mut png_buf = Vec::new();

            let len = file.read_to_end(&mut png_buf)?;

            let mut png = if len == 0 {
                Png::new()
            } else {
                Self::get_png(&png_buf[..])?
            };

            if redirect_output {
                let to_file = self.output_file.unwrap();

                file = Self::open_file(&to_file, false, true, false, false)?;
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

impl Command for DecodeArgs {
    fn execute_command(self) -> Result<()> {
        let mut file = Self::open_file(&self.file_path, true, false, false, false)?;

        let mut png_buf = Vec::new();

        file.read_to_end(&mut png_buf)?;
        let png = Self::get_png(&png_buf[..])?;

        let message = png
            .get_chunk_by_type(&self.chunk_type)
            .map_or(Ok(String::from("")), |chunk| chunk.data_as_string())
            .map_err(|err| Error::from(format!("Invalid message data: {}", err)))?;

        println!("{}", message);

        Ok(())
    }
}

// #[test]
// fn test() -> Result<()> {
//     /*let invalid_header = [20_u8; 8];
//     let chunks = vec![
//         Chunk::from_strings("FrSt", "I am the first chunk").unwrap(),
//         Chunk::from_strings("miDl", "I am another chunk").unwrap(),
//         Chunk::from_strings("LASt", "I am the last chunk").unwrap(),
//     ];
//
//     let bytes = invalid_header
//         .into_iter()
//         .chain(chunks.iter().flat_map(|chunk| chunk.as_bytes()).into_iter())
//         .collect::<Vec<u8>>();
//     write_byte_to_file("tests/inputs/invalid_header.png", &bytes)?;
//     Ok(())*/
//     // let header_only = Png::new();
//     //
//     // create_png_file_from_bytes("tests/inputs/header_only.png", &header_only.as_bytes())?;
//     //
//     // Ok(())
// }
