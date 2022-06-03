use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt;
use std::io::{BufReader, Read};
const CRC_CALCULATOR: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

struct Chunk {
    chunk_type: ChunkType,
    len: u32,
    crc: u32,
    data: Vec<u8>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let len = data.len();
        let crc = CRC_CALCULATOR.checksum(&data);
        Chunk {
            chunk_type,
            len: len as u32,
            crc,
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.len
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.len
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn data_as_string(&self) -> Result<String> {
        let string = String::from_utf8(self.data.clone())?;
        Ok(string)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Error::from(format!(
                "Minimum length is 12, but {} was specified",
                value.len()
            )));
        }
        let mut reader = BufReader::new(value);

        let mut u32_buff: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut u32_buff)?;
        let len = u32::from_be_bytes(u32_buff);

        reader.read_exact(&mut u32_buff)?;
        let chunk_type = ChunkType::try_from(u32_buff)?;

        let mut data_buff: Vec<u8> = vec![0; len as usize];
        reader.read_exact(&mut data_buff)?;

        let data = data_buff.to_vec();

        reader.read_exact(&mut u32_buff)?;
        let crc = u32::from_be_bytes(u32_buff);

        Ok(Chunk {
            len,
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
