use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand};

pub use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

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
    /// Dont decode and output removed messages
    #[clap(short, long)]
    ignore_messages: bool,
}

#[derive(Args, Debug)]
pub struct PrintArgs {
    file_path: String,
}

pub fn execute_command(command: CommandType) -> Result<()> {
    match command {
        CommandType::Encode(args) => encode(args),
        CommandType::Decode(args) => decode(args),
        CommandType::Print(args) => print_png(args),
        CommandType::Remove(args) => remove_chunk(args),
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
}

pub fn get_png(filename: &str) -> Result<Png> {
    let png_buf = fs::read(filename)
        .map_err(|err| Error::from(format!("Cannot open file {}, cause: {}", filename, err)))?;

    Png::try_from(&png_buf[..])
        .map_err(|err| Error::from(format!("Invalid png file data ({})", err)))
}

fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = get_png(&args.file_path)?;

    png.append_chunk(Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.into_bytes(),
    ));

    let output = &args.output_file.unwrap_or(args.file_path);
    fs::write(output, &png.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let png = get_png(&args.file_path)?;

    let chunk = png.get_chunk_by_type(&args.chunk_type);

    if let Some(chunk) = chunk {
        let message = chunk
            .data_as_string()
            .map_err(|err| Error::from(format!("Invalid message data: {}", err)))?;

        println!("secret message: '{}'", message);
    } else {
        println!("No chunk with type '{}' was found", args.chunk_type);
    }

    Ok(())
}

fn print_png(args: PrintArgs) -> Result<()> {
    let png = get_png(&args.file_path)?;

    println!("{}", png);

    Ok(())
}

fn remove_chunk(args: RemoveArgs) -> Result<()> {
    let filename = &args.file_path;
    let mut png = get_png(filename)?;

    let deleted_chunk = png.remove_chunk(&args.chunk_type);

    if let Ok(chunk) = deleted_chunk {
        // fs::write(filename, &png.as_bytes())?;
        if !args.ignore_messages {
            println!("deleted chunk with message '{}'", chunk.data_as_string()?);
        }
        Ok(())
    } else {
        Err(Error::from(format!(
            "No chunk with type '{}' was found",
            args.chunk_type
        )))
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
