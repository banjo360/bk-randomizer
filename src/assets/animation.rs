use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub struct Animation {}

impl Animation {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        todo!();
        Ok(Self {})
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        todo!();
        Ok(())
    }
}
