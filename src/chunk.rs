use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

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
}
