use std::{fmt::Display, str::FromStr};

#[derive(Eq, Debug, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(values: [u8; 4]) -> Result<Self, Self::Error> {
        for value in values.iter() {
            if !ChunkType::is_valid_byte(*value) {
                return Err("inivalid bytes.");
            }
        }

        Ok(ChunkType { bytes: values })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_ascii() && s.len() == 4 {
            let s_as_bytes = s.as_bytes();
            let mut chunk_bytes = [0; 4];
            let mut i = 0;
            for byte in s_as_bytes {
                if ChunkType::is_valid_byte(*byte) {
                    chunk_bytes[i] = *byte;
                    i += 1;
                } else {
                    return Err("invalid string");
                }
            }
            return Ok(ChunkType { bytes: chunk_bytes });
        } else {
            return Err("invalid string");
        }
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        let mut i = 0;
        for byte_i in self.bytes {
            if byte_i == other.bytes[i] {
                i += 1;
            } else {
                return false;
            }
        }
        true
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.bytes).unwrap())
    }
}
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_valid_byte(byte: u8) -> bool {
        if (byte > 64 && byte < 91) || (byte > 96 && byte < 123) {
            return true;
        }
        false
    }

    fn is_valid(&self) -> bool {
        for byte in self.bytes {
            if !ChunkType::is_valid_byte(byte) {
                return false;
            }
        }

        if !self.is_reserved_bit_valid() {
            return false;
        }

        true
    }

    fn i_is_uppercase(&self, i: usize) -> bool {
        let i_byte = self.bytes[i];
        if i_byte > 64 && i_byte < 91 {
            return true;
        }
        false
    }

    fn is_critical(&self) -> bool {
        self.i_is_uppercase(0)
    }

    fn is_public(&self) -> bool {
        self.i_is_uppercase(1)
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.i_is_uppercase(2)
    }

    fn is_safe_to_copy(&self) -> bool {
        !self.i_is_uppercase(3)
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
