use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub struct Sprite {
    unk04: u16,
    unk06: u16,
    unk08: u16,
    unk0a: u16,
    unk0c: u16,
    unk0e: u16,
    frames: Vec<SpriteFrame>,
}

impl Sprite {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let frame_count = reader.read_u16::<BigEndian>()?;
        let format = reader.read_u16::<BigEndian>()?;
        assert_eq!(format, 0x1000);
        let unk04 = reader.read_u16::<BigEndian>()?;
        let unk06 = reader.read_u16::<BigEndian>()?;
        let unk08 = reader.read_u16::<BigEndian>()?;
        let unk0a = reader.read_u16::<BigEndian>()?;
        let unk0c = reader.read_u16::<BigEndian>()?;
        let unk0e = reader.read_u16::<BigEndian>()?;
        let mut frames = vec![];

        // offsets
        for _ in 0..frame_count {
            let offset = reader.read_u32::<BigEndian>()?;
            assert_eq!(offset % SPRITE_FRAME_SIZE, 0);
        }

        for _ in 0..frame_count {
            let unk00 = reader.read_u16::<BigEndian>()?;
            let unk02 = reader.read_u16::<BigEndian>()?;
            let unk04 = reader.read_u16::<BigEndian>()?;
            let unk06 = reader.read_u16::<BigEndian>()?;
            let unk08 = reader.read_u16::<BigEndian>()?;
            let texture_id = reader.read_u16::<BigEndian>()?;
            let unk0c = reader.read_i16::<BigEndian>()?;
            let unk0e = reader.read_i16::<BigEndian>()?;
            let unk10 = reader.read_i16::<BigEndian>()?;
            let unk12 = reader.read_i16::<BigEndian>()?;
            let unk14 = reader.read_u16::<BigEndian>()?;
            let unk16 = reader.read_u16::<BigEndian>()?;
            let unk18 = reader.read_u16::<BigEndian>()?;
            let unk1a = reader.read_u16::<BigEndian>()?;

            assert_eq!(unk14, 0);
            assert_eq!(unk16, 0);
            assert_eq!(unk18, unk04);
            assert_eq!(unk1a, unk06);

            frames.push(SpriteFrame {
                unk00,
                unk02,
                unk04,
                unk06,
                unk08,
                texture_id,
                unk0c,
                unk0e,
                unk10,
                unk12,
                unk14,
                unk16,
                unk18,
                unk1a,
            });
        }

        Ok(Self {
            unk04,
            unk06,
            unk08,
            unk0a,
            unk0c,
            unk0e,
            frames,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_u16::<BigEndian>(self.frames.len() as u16)?;
        writer.write_u16::<BigEndian>(0x1000)?;
        writer.write_u16::<BigEndian>(self.unk04)?;
        writer.write_u16::<BigEndian>(self.unk06)?;
        writer.write_u16::<BigEndian>(self.unk08)?;
        writer.write_u16::<BigEndian>(self.unk0a)?;
        writer.write_u16::<BigEndian>(self.unk0c)?;
        writer.write_u16::<BigEndian>(self.unk0e)?;

        for i in 0..self.frames.len() {
            writer.write_u32::<BigEndian>(i as u32 * SPRITE_FRAME_SIZE)?;
        }

        for frame in &self.frames {
            writer.write_u16::<BigEndian>(frame.unk00)?;
            writer.write_u16::<BigEndian>(frame.unk02)?;
            writer.write_u16::<BigEndian>(frame.unk04)?;
            writer.write_u16::<BigEndian>(frame.unk06)?;
            writer.write_u16::<BigEndian>(frame.unk08)?;
            writer.write_u16::<BigEndian>(frame.texture_id)?;
            writer.write_i16::<BigEndian>(frame.unk0c)?;
            writer.write_i16::<BigEndian>(frame.unk0e)?;
            writer.write_i16::<BigEndian>(frame.unk10)?;
            writer.write_i16::<BigEndian>(frame.unk12)?;
            writer.write_u16::<BigEndian>(frame.unk14)?;
            writer.write_u16::<BigEndian>(frame.unk16)?;
            writer.write_u16::<BigEndian>(frame.unk18)?;
            writer.write_u16::<BigEndian>(frame.unk1a)?;
        }

        Ok(())
    }
}

const SPRITE_FRAME_SIZE: u32 = 0x1C;
pub struct SpriteFrame {
    unk00: u16,
    unk02: u16,
    unk04: u16, // width?
    unk06: u16, // height?
    unk08: u16,
    texture_id: u16,
    unk0c: i16,
    unk0e: i16,
    unk10: i16,
    unk12: i16,
    unk14: u16, // always zero. x?
    unk16: u16, // always zero. y?
    unk18: u16, // width?
    unk1a: u16, // height?
}
