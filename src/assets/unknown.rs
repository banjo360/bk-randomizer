use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub struct Unknown {
    buffer: Vec<u8>,
}

impl Unknown {
    pub fn new<R: Read + Seek>(reader: &mut R, length: usize) -> Result<Self, Box<dyn Error>> {
        let mut buffer = vec![0u8; length];
        reader.read(&mut buffer)?;
        Ok(Self { buffer })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write(&self.buffer)?;
        Ok(())
    }
}
