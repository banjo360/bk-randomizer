use crate::ActorId;
use crate::WarpOrTriggerId;
use crate::utils::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Category {
    WarpOrTrigger(WarpOrTriggerId),
    CameraController(u16),
    Actor(ActorId),
    EnemyBoundary(u16),
    Path(u16),
    CameraTrigger(u16),
    Flags(u16),
    Unknown(u8, u16),
}

impl Into<u8> for Category {
    fn into(self) -> u8 {
        match self {
            Category::WarpOrTrigger(_) => 3,
            Category::CameraController(_) => 4,
            Category::Actor(_) => 6,
            Category::EnemyBoundary(_) => 7,
            Category::Path(_) => 8,
            Category::CameraTrigger(_) => 9,
            Category::Flags(_) => 10,
            Category::Unknown(value, _) => value,
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::WarpOrTrigger(actor_id) => write!(f, "WarpOrTrigger({actor_id})"),
            Category::CameraController(actor_id) => write!(f, "CameraController({actor_id})"),
            Category::Actor(actor_id) => write!(f, "Actor({actor_id})"),
            Category::EnemyBoundary(actor_id) => write!(f, "EnemyBoundary({actor_id})"),
            Category::Path(actor_id) => write!(f, "Path({actor_id})"),
            Category::CameraTrigger(actor_id) => write!(f, "CameraTrigger({actor_id})"),
            Category::Flags(actor_id) => write!(f, "Flags({actor_id})"),
            Category::Unknown(cat, actor_id) => write!(f, "Unknown({cat}, {actor_id})"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Prop1 {
    pub position: Vector3<i16>,
    pub selector_or_radius: u16,
    pub category: Category,
    pub unk_bit_0: u8,
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
        let category = ((bitfield_06 >> 1) & 0b111111) as u8;
        let category = match category {
            3 => Category::WarpOrTrigger(actor_id.into()),
            4 => Category::CameraController(actor_id),
            6 => Category::Actor(actor_id.into()),
            7 => Category::EnemyBoundary(actor_id),
            8 => Category::Path(actor_id),
            9 => Category::CameraTrigger(actor_id),
            10 => Category::Flags(actor_id),
            _ => Category::Unknown(category, actor_id),
        };
        let unk_bit_0 = (bitfield_06 & 1) as u8;

        Ok(Self {
            position,
            selector_or_radius,
            category,
            unk_bit_0,
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
        let actor_id = match self.category {
            Category::CameraController(actor_id)
            | Category::EnemyBoundary(actor_id)
            | Category::Path(actor_id)
            | Category::CameraTrigger(actor_id)
            | Category::Flags(actor_id)
            | Category::Unknown(_, actor_id) => actor_id,
            Category::WarpOrTrigger(actor_id) => actor_id.into(),
            Category::Actor(actor_id) => actor_id.into(),
        };
        writer.write_u16::<BigEndian>(actor_id)?;
        writer.write_u8(self.marker_id)?;
        writer.write_u8(self.byte_0b)?;
        writer.write_u32::<BigEndian>(self.bitfield_0c)?;
        writer.write_u32::<BigEndian>(self.bitfield_10)?;

        Ok(())
    }
}
