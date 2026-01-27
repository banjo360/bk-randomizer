use std::error::Error;
use std::io::Read;
use std::io::Seek;

pub struct Animation {}

impl Animation {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }
}
