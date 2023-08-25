use std::{
    fmt::Display,
    io::{BufReader, Read},
    str::{FromStr, Utf8Error},
};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type, Error};

use super::chunk_type::ChunkType;

#[derive(Debug, Clone)]
struct ChunkByteError;

impl std::error::Error for ChunkByteError {}

pub const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl Display for ChunkByteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid bytes")
    }
}

#[derive(Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut length = 0;

        let mut length_bytes: [u8; 4] = [0; 4];
        let mut read_result = reader.read_exact(&mut length_bytes);

        match read_result {
            Ok(_) => {
                length = u32::from_be_bytes(length_bytes);
                let mut chunk_type_bytes: [u8; 4] = [0; 4];
                read_result = reader.read_exact(&mut chunk_type_bytes);
                match read_result {
                    Ok(_) => {
                        let chunk_type = ChunkType::try_from(chunk_type_bytes).unwrap();
                        let ulen: u8 = length as u8;
                        let mut data_vec: Vec<_> = (0..ulen).collect();
                        read_result = reader.read_exact(&mut data_vec[..]);
                        match read_result {
                            Ok(_) => {
                                let mut crc_bytes: [u8; 4] = [0; 4];
                                read_result = reader.read_exact(&mut crc_bytes);
                                match read_result {
                                    Ok(_) => {
                                        let chunk: Chunk = Chunk::new(chunk_type, data_vec);
                                        return Ok(chunk);
                                    }
                                    Err(e) => return Err(Box::new(e)),
                                };
                            }
                            Err(e) => return Err(Box::new(e)),
                        }
                    }
                    Err(e) => return Err(Box::new(e)),
                };
            }
            Err(e) => return Err(Box::new(e)),
        };
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let len = data.len() as u32;

        Chunk {
            length: len,
            chunk_data: data.clone(),
            crc: make_crc(len, &chunk_type, data),
            chunk_type: chunk_type,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data[..]
    }

    pub fn chunk_from_strings(
        type_string: String,
        data_string: String,
    ) -> Result<Chunk, &'static str> {
        let type_result = ChunkType::from_str(&type_string[..]);

        match type_result {
            Ok(chunk_type) => return Ok(Chunk::new(chunk_type, data_string.as_bytes().to_vec())),
            Err(e) => return Err(e),
        }
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        match std::str::from_utf8(self.data()) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        let length_bytes: [u8; 4] = self.length().to_be_bytes();
        let chunk_type_bytes: [u8; 4] = self.chunk_type().bytes();
        let crc_bytes: [u8; 4] = self.crc().to_be_bytes();

        for byte in length_bytes.iter() {
            vec.push(*byte);
        }
        for byte in chunk_type_bytes.iter() {
            vec.push(*byte);
        }
        for byte in self.data().iter() {
            vec.push(*byte);
        }
        for byte in crc_bytes.iter() {
            vec.push(*byte);
        }

        vec
    }
}

fn make_crc(length: u32, chunk_type: &ChunkType, data: Vec<u8>) -> u32 {
    let length_as_bytes = length.to_be_bytes();
    let type_bytes: [u8; 4] = chunk_type.bytes();
    let data_bytes: &[u8] = &data[..];

    let mut chunk_vec: Vec<u8> = Vec::new();

    for byte in length_as_bytes.iter() {
        chunk_vec.push(*byte);
    }
    for byte in type_bytes.iter() {
        chunk_vec.push(*byte);
    }
    for byte in data_bytes.iter() {
        chunk_vec.push(*byte);
    }

    CRC_PNG.checksum(&chunk_vec[..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        println!("{}", format!("{}", chunk));
        assert_eq!(chunk.length(), 42);
        //assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        // assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        println!("{}", format!("{}", chunk));

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
