use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len().try_into().unwrap();
        let crc_bytes: Vec<u8> = chunk_type
            .chunk_type
            .clone()
            .into_iter()
            .chain(data.iter().cloned())
            .collect();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let calculated_crc = crc.checksum(&crc_bytes);

        Self {
            length,
            chunk_type,
            chunk_data: data,
            crc: calculated_crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data_as_string(&self) -> Result<String, std::str::Utf8Error> {
        Ok(String::from_utf8(self.chunk_data.clone()).unwrap())
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn as_bytes(&self) -> Vec<u8> {
        let bytes = self
            .chunk_type
            .chunk_type
            .clone()
            .into_iter()
            .chain(self.chunk_data.clone())
            .collect();
        bytes
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk: {{")?;
        writeln!(f, "length: {}", self.length)?;
        writeln!(f, "Chunk Type: {}", self.chunk_type)?;
        writeln!(f, "Data: {}", self.chunk_data.len())?;
        writeln!(f, "Crc: {}", self.crc)?;
        write!(f, "}}")?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() < 12 {
            return Err("Slice is empty or holds less data than needed".into());
        }
        // Use BufReader to wrap around the slice
        let mut reader = BufReader::new(value);
        // Read the first 4 bytes to determine data length
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut buffer)?;
        let data_length: u32 = u32::from_be_bytes(buffer);

        // Read next 4 bytes to determine chunk type
        let mut chunk_type = [0u8; 4];
        reader.read_exact(&mut chunk_type)?;

        // Read n bytes to determine data
        //let mut data = [0u8; data_length as usize];
        let mut data = vec![0u8; data_length as usize];
        reader.read_exact(&mut data)?;

        // Take chunk_data and chunk_type for CRC
        let mut crc_bytes = [0u8; 4];
        reader.read_exact(&mut crc_bytes)?;

        let provided_crc = u32::from_be_bytes(crc_bytes);

        // Calculate crc over entire chunk
        let mut crc_data = Vec::with_capacity(8 + data.len());
        crc_data.extend_from_slice(&chunk_type);
        crc_data.extend_from_slice(&data);

        let calculated_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&crc_data);

        if calculated_crc != provided_crc {
            return Err("CRC mismatch".into());
        }

        Ok(Self {
            length: data_length,
            chunk_type: ChunkType {
                chunk_type: chunk_type.to_vec(),
            },
            chunk_data: data,
            crc: calculated_crc,
        })
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
