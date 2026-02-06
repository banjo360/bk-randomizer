use crate::enums::SpritePropId;
use crate::utils::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum Prop2 {
    Actor {
        flags: u32,
        position: Vector3<i16>,
        scale: u8,
        bitfield_0b: u8,
    },
    Sprite {
        id: SpritePropId,
        flags: u32,
        position: Vector3<i16>,
        bitfield_0a: u16,
    },
}

impl Prop2 {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let flags = reader.read_u32::<BigEndian>()?;
        let position = read_3_i16(reader)?;
        let ending = reader.read_u16::<BigEndian>()?;
        assert_ne!(ending & 0b11, 1); // !is_3d and is_actor
        assert_ne!(ending & 0b11, 3); // is_3d and is_actor

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
            let id: SpritePropId = ((flags >> 20) as u16).into();
            let flags = flags & 0x000FFFFF;
            Ok(Prop2::Sprite {
                id,
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
                write_3_i16(writer, position)?;
                writer.write_u8(*scale)?;
                writer.write_u8(*bitfield_0b)?;
            }
            Prop2::Sprite {
                id,
                flags,
                position,
                bitfield_0a,
            } => {
                let id: u16 = (*id).into();
                let id = (id as u32) << 20;
                writer.write_u32::<BigEndian>(*flags + id)?;
                write_3_i16(writer, position)?;
                writer.write_u16::<BigEndian>(*bitfield_0a)?;
            }
        }
        Ok(())
    }
}
