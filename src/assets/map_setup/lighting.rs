use crate::utils::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub struct Lighting {
    position: Vector3<f32>,
    unk: Vector2<f32>,
    red: u32,
    green: u32,
    blue: u32,
}

impl Lighting {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 2);
        let position = read_3_floats(reader)?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 3);
        let unk = read_2_floats(reader)?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 4);
        let colours = read_3_u32(reader)?;

        Ok(Self {
            position,
            unk,
            red: colours.x,
            green: colours.y,
            blue: colours.z,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_u8(1)?;

        writer.write_u8(2)?;
        write_3_floats(writer, &self.position)?;

        writer.write_u8(3)?;
        write_2_floats(writer, &self.unk)?;

        writer.write_u8(4)?;
        writer.write_u32::<BigEndian>(self.red)?;
        writer.write_u32::<BigEndian>(self.green)?;
        writer.write_u32::<BigEndian>(self.blue)?;

        Ok(())
    }
}
