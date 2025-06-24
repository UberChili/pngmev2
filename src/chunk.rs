use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use crc::CRC_32_ISO_HDLC;

use crate::chunk_type::ChunkType;

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    // Required method
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Ensuring the slice has enough bytes
        if value.len() < 12 {
            return Err("try_from failed. Data has length less than 12.".into());
        }

        // Getting length
        let mut reader = BufReader::new(value);
        let mut length_buffer: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut length_buffer)?;
        let data_length = u32::from_be_bytes(length_buffer);

        // Getting Chunk Type
        let mut ct_buffer: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut ct_buffer)?;
        let chunk_type = ChunkType::try_from(ct_buffer)?;

        // Getting data
        let mut data: Vec<u8> = vec![0; data_length as usize];
        reader.read_exact(&mut data)?;

        // Reading CRC
        let mut crc_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut crc_bytes)?;
        let stored_crc = u32::from_be_bytes(crc_bytes);

        // Calculate CRC for validation
        let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc_data: Vec<u8> = ct_buffer.iter().chain(data.iter()).copied().collect();
        let calculated_crc = crc.checksum(&crc_data);

        // Validate CRC
        if calculated_crc != stored_crc {
            return Err("try_from failed. CRC mismatch.".into());
        }

        Ok(Chunk {
            length: data_length,
            chunk_type,
            data,
            crc: stored_crc,
        })
    }
}

impl Display for Chunk {
    // Required method
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

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;

        let result: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(data.clone())
            .collect();
        let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let calculated_crc = crc.checksum(&result);

        Chunk {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: calculated_crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let length: [u8; 4] = self.length.to_be_bytes();
        let chunk_type: [u8; 4] = self.chunk_type.bytes();
        let data: Vec<u8> = self.data.clone();
        let crc: [u8; 4] = self.crc.to_be_bytes();

        let result: Vec<u8> = length
            .into_iter()
            .chain(chunk_type.into_iter())
            .chain(data.into_iter())
            .chain(crc.into_iter())
            .collect();
        result
    }
}

#[allow(dead_code)]
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
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
        assert_eq!(chunk.crc(), 2882656334);
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
