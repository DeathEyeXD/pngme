use crate::Error;
use std::fmt;
use std::fmt::Formatter;
use std::path::Display;
use std::str::FromStr;

struct ChunkType {
    values: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.values
    }

    fn is_valid(&self) -> bool {
        // bits are checked on creation, so only chgeck if reserved bit is valid
        self.is_reserved_bit_valid()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        Self::is_reserved_byte_valid(self.values[2])
    }

    fn is_reserved_byte_valid(byte: u8) -> bool {
        byte.is_ascii_uppercase()
    }

    fn is_byte_valid(byte: u8) -> crate::Result<()> {
        if byte.is_ascii_alphabetic() {
            return Err(ChunkType::error_byte(byte));
        }
        Ok(())
    }
    fn is_bit_safe_to_copy(byte: u8) -> bool {
        byte.is_ascii_lowercase()
    }
    fn error_byte(byte: u8) -> Error {
        return Error::from(format!("{} is not a valid png byte", byte));
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(array: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in array {
            if let 65..=90 | 97..=122 = byte {
                return Err(Error::from(format!("{} is not a valid png byte", byte)));
            }
        }
        Ok(ChunkType { values: array })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0; 4];
        for (index, byte) in s.bytes().enumerate() {
            ChunkType::is_byte_valid(byte)?;
            bytes[index] = byte;
        }
        Ok(ChunkType { values: bytes })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes()))
    }
}
