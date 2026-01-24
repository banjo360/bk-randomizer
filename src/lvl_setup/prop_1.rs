use crate::types::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Category {
    WarpOrTrigger = 3,
    CameraController = 4,
    Actor = 6,
    EnemyBoundary = 7,
    Path = 8,
    CameraTrigger = 9,
    Flags = 10,
    Unknown(u8),
}

impl Into<u8> for Category {
    fn into(self) -> u8 {
        match self {
            Category::WarpOrTrigger => 3,
            Category::CameraController => 4,
            Category::Actor => 6,
            Category::EnemyBoundary => 7,
            Category::Path => 8,
            Category::CameraTrigger => 9,
            Category::Flags => 10,
            Category::Unknown(value) => value,
        }
    }
}

impl From<u8> for Category {
    fn from(value: u8) -> Self {
        match value {
            3 => Category::WarpOrTrigger,
            4 => Category::CameraController,
            6 => Category::Actor,
            7 => Category::EnemyBoundary,
            8 => Category::Path,
            9 => Category::CameraTrigger,
            10 => Category::Flags,
            _ => Category::Unknown(value),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Prop1 {
    pub position: Vector3<i16>,
    pub selector_or_radius: u16,
    pub category: Category,
    pub unk_bit_0: u8,
    pub actor_id: u16,
    pub marker_id: u8,
    pub byte_0b: u8,
    pub bitfield_0c: u32,
    pub bitfield_10: u32,
}

impl Prop1 {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let position = read_3_i16(reader)?;
        let bitfield_06 = reader.read_u16::<BigEndian>()?;
        let actor_id = reader.read_u16::<BigEndian>()?;
        let marker_id = reader.read_u8()?;
        let byte_0b = reader.read_u8()?;
        let bitfield_0c = reader.read_u32::<BigEndian>()?;
        let bitfield_10 = reader.read_u32::<BigEndian>()?;

        let selector_or_radius = bitfield_06 >> 7;
        let category = (((bitfield_06 >> 1) & 0b111111) as u8).into();
        let unk_bit_0 = (bitfield_06 & 1) as u8;

        Ok(Self {
            position,
            selector_or_radius,
            category,
            unk_bit_0,
            actor_id,
            marker_id,
            byte_0b,
            bitfield_0c,
            bitfield_10,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        write_3_i16(writer, &self.position)?;

        let category: u8 = self.category.into();
        let bitfield_06 =
            (self.selector_or_radius << 7) + ((category << 1) + self.unk_bit_0) as u16;
        writer.write_u16::<BigEndian>(bitfield_06)?;
        writer.write_u16::<BigEndian>(self.actor_id)?;
        writer.write_u8(self.marker_id)?;
        writer.write_u8(self.byte_0b)?;
        writer.write_u32::<BigEndian>(self.bitfield_0c)?;
        writer.write_u32::<BigEndian>(self.bitfield_10)?;

        Ok(())
    }
}
