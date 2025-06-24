use std::{fs, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

// Encodes a message into a file
pub fn encode(filepath: &String, chunk: &String, message: &String) -> crate::Result<()> {
    let file = fs::read(&filepath)?;

    let mut png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(&chunk)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    fs::write(filepath, png.as_bytes())?;

    Ok(())
}

// Decodes a message from a file
pub fn decode(filepath: &String, chunk: &String) -> crate::Result<()> {
    let file = fs::read(&filepath)?;

    let png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(&chunk)?;

    let wanted_chunk = png
        .chunks()
        .iter()
        .find(|c| c.chunk_type().to_string() == chunk_type.to_string());

    if let Some(found_chunk) = wanted_chunk {
        let message = String::from_utf8(found_chunk.data().to_vec())?;
        println!("Message: {}", message);
    } else {
        return Err(format!("No message found in this file.").into());
    }

    Ok(())
}

// Removes a message from a file
pub fn remove(filepath: &String, chunk: &String) -> crate::Result<()> {
    let file = fs::read(&filepath)?;
    let mut png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(&chunk)?;

    let wanted_chunk = png
        .chunks()
        .iter()
        .any(|c| c.chunk_type().to_string() == chunk_type.to_string());

    if !wanted_chunk {
        return Err(format!("No chunk of type {} was found in the file.", chunk).into());
    }

    png.remove_first_chunk(&chunk)?;

    fs::write(filepath, png.as_bytes())?;

    Ok(())
}

// Prints a message, if it exists
pub fn print(filepath: &String) -> crate::Result<()> {
    let file = fs::read(&filepath)?;
    let png = Png::try_from(file.as_slice())?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}
