use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    // values: [u8; 4],
    values: Vec<u8>,
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    // Required method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<u8> = Vec::new();

        if s.len() != 4 {
            return Err("Could not convert from str".into());
        }

        for c in s.chars() {
            if c.is_alphabetic() {
                values.push(c as u8);
            } else {
                return Err("Non alphabetic character".into());
            }
        }

        Ok(ChunkType { values })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    // Required method
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        // If we were using a slice on the ChunkType type
        // Ok(ChunkType { values: value })

        let result = value.into_iter().collect();
        Ok(ChunkType { values: result })
    }
}

impl Display for ChunkType {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8(self.values.clone()).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}

#[allow(dead_code)]
impl ChunkType {
    pub fn values(&self) -> &Vec<u8> {
        &self.values
    }

    pub fn bytes(&self) -> [u8; 4] {
        self.values
            .get(0..4)
            .expect("Error getting bytes")
            .try_into()
            .unwrap()
    }

    pub fn is_valid(&self) -> bool {
        for c in &self.values {
            if !c.is_ascii_alphabetic() {
                return false;
            }
        }

        if !self.is_reserved_bit_valid() {
            return false;
        }
        return true;
    }

    pub fn is_critical(&self) -> bool {
        if self.values[0].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    pub fn is_public(&self) -> bool {
        if self.values[1].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        if self.values[2].is_ascii_uppercase() {
            return true;
        }
        return false;
    }

    pub fn is_safe_to_copy(&self) -> bool {
        if self.values[3].is_ascii_lowercase() {
            return true;
        }
        return false;
    }
}

#[allow(dead_code)]
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
