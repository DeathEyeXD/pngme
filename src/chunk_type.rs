use crate::Error;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub(crate) struct ChunkType {
    values: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.values
    }

    pub fn is_valid(&self) -> bool {
        // bits are checked on creation, so only chgeck if reserved bit is valid
        self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        self.values[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.values[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        Self::is_reserved_byte_valid(self.values[2])
    }

    pub fn is_reserved_byte_valid(byte: u8) -> bool {
        byte.is_ascii_uppercase()
    }

    fn is_byte_valid(byte: u8) -> crate::Result<()> {
        if !byte.is_ascii_alphabetic() {
            return Err(ChunkType::error_byte(byte));
        }
        Ok(())
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.values[3].is_ascii_lowercase()
    }

    fn error_byte(byte: u8) -> Error {
        return Error::from(format!("{} is not a valid png byte", byte));
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(array: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in array {
            Self::is_byte_valid(byte)?
        }
        Ok(ChunkType { values: array })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        let len = s.len();
        if len > 4 {
            return Err(Error::from(format!(
                "Required 4 byte string got {} bytes",
                len
            )));
        }
        let bytes = [s[0], s[1], s[2], s[3]];
        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
