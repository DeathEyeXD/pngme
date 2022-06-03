use crate::chunk::Chunk;
use crate::{Error, Result};
use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};

pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    fn new(chunks: Vec<Chunk>) -> Png {
        Png {
            header: Self::STANDARD_HEADER,
            chunks,
        }
    }

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Self::new(chunks)
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        Ok(self.chunks.remove(
            self.chunks
                .iter()
                .position(|chunk| chunk.chunk_type().to_string() == chunk_type)
                .ok_or_else(|| {
                    Error::from(format!(
                        "Cannot remove and find chunk with type {}",
                        chunk_type
                    ))
                })?,
        ))
    }

    pub fn header() -> &'static [u8; 8] {
        &Self::STANDARD_HEADER
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
    }

    fn as_bytes(&self) -> Vec<u8> {
        Self::STANDARD_HEADER
            .into_iter()
            .chain(
                self.chunks()
                    .iter()
                    .flat_map(|chunk| chunk.as_bytes())
                    .into_iter(),
            )
            .collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        if bytes.len() < 4 {
            return Err(Error::from(format!(
                "Valid png must have at least 8 bytes but only {} were provided",
                bytes.len()
            )));
        }
        let mut reader = BufReader::new(bytes);
        let mut header = [0 as u8; 8];

        reader.read_exact(&mut header)?;

        if header != Png::STANDARD_HEADER {
            return Err(Error::from(format!(
                "Valid png must contain valid signature header ({:?}), but {:?} header war provided",
                Png::STANDARD_HEADER, header
            )));
        }

        let mut len_of_next_chunk = [0; 4];

        let mut chunks = Vec::new();
        while let Ok(len) = reader.read(&mut len_of_next_chunk) {
            if len != 4 {
                break;
            };
            let len = u32::from_be_bytes(len_of_next_chunk);
            let mut chunk_buf = vec![0; 16 + len as usize];

            reader.read_exact(&mut chunk_buf)?;

            chunks.push(Chunk::try_from(&chunk_buf[..])?);
        }

        Ok(Png::from_chunks(chunks))
    }
}

impl Display for Png {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Png {{")?;
        writeln!(f, "   HEADER: {:?}", Self::STANDARD_HEADER)?;
        writeln!(f, "   Chunks: {{")?;
        for chunk in &self.chunks {
            writeln!(f, "        {}", chunk)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
