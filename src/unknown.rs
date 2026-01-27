use std::error::Error;
use std::io::Read;
use std::io::Seek;

pub struct Unknown {}

impl Unknown {
    pub fn new<R: Read + Seek>(reader: &mut R, length: usize) -> Result<Self, Box<dyn Error>> {
        let mut buffer = vec![0u8; length];
        reader.read(&mut buffer)?;
        Ok(Self {})
    }
}
