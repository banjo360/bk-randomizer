use crate::utils::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub enum Prop2 {
    Actor {
        flags: u32,
        position: Vector3<u16>,
        scale: u8,
        bitfield_0b: u8,
    },
    Sprite {
        flags: u32,
        position: Vector3<u16>,
        bitfield_0a: u16,
    },
}

impl Prop2 {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let flags = reader.read_u32::<BigEndian>()?;
        let position = read_3_u16(reader)?;
        let ending = reader.read_u16::<BigEndian>()?;
        assert_ne!(ending & 0b10, 1); // !is_3d and is_actor
        assert_ne!(ending & 0b10, 3); // is_3d and is_actor

        if (ending & 0b10) != 0 {
            let scale = (ending >> 8) as u8;
            let bitfield_0b = (ending & 0xFF) as u8;
            Ok(Prop2::Actor {
                flags,
                position,
                scale,
                bitfield_0b,
            })
        } else {
            let bitfield_0a = ending;
            Ok(Prop2::Sprite {
                flags,
                position,
                bitfield_0a,
            })
        }
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        match self {
            Prop2::Actor {
                flags,
                position,
                scale,
                bitfield_0b,
            } => {
                writer.write_u32::<BigEndian>(*flags)?;
                write_3_u16(writer, position)?;
                writer.write_u8(*scale)?;
                writer.write_u8(*bitfield_0b)?;
            }
            Prop2::Sprite {
                flags,
                position,
                bitfield_0a,
            } => {
                writer.write_u32::<BigEndian>(*flags)?;
                write_3_u16(writer, position)?;
                writer.write_u16::<BigEndian>(*bitfield_0a)?;
            }
        }
        Ok(())
    }
}
