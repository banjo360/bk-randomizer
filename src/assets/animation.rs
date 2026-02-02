use crate::enums::Transform;
use crate::utils::fixed_to_float;
use crate::utils::float_to_fixed;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub struct Animation {
    start_frame: u16,
    end_frame: u16,
    transforms: Vec<TransformData>,
}

pub struct TransformData {
    bone: u16,
    kind: Transform,
    frames: Vec<FrameData>,
}

pub struct FrameData {
    unk1: bool,
    unk2: bool,
    frame: u16,
    factor: f32,
}

impl Animation {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let start_frame = reader.read_u16::<BigEndian>()?;
        let end_frame = reader.read_u16::<BigEndian>()?;
        let elem_count = reader.read_u16::<BigEndian>()?;
        let padding = reader.read_u16::<BigEndian>()?;
        assert_eq!(padding, 0);

        let mut transforms = vec![];
        for _ in 0..elem_count {
            let header = reader.read_u16::<BigEndian>()?;
            let data_count = reader.read_u16::<BigEndian>()?;

            let bone = header >> 4;
            let kind: Transform = (header & 0x0F).into();

            let mut frames = vec![];
            for _ in 0..data_count {
                let header = reader.read_u16::<BigEndian>()?;
                let factor = reader.read_u16::<BigEndian>()?;
                let factor = fixed_to_float(factor);

                frames.push(FrameData {
                    unk1: (header >> 15) != 0,
                    unk2: ((header >> 14) & 1) != 0,
                    frame: header & 0x3FFF,
                    factor,
                });
            }

            transforms.push(TransformData { bone, kind, frames });
        }

        Ok(Self {
            start_frame,
            end_frame,
            transforms,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_u16::<BigEndian>(self.start_frame)?;
        writer.write_u16::<BigEndian>(self.end_frame)?;
        writer.write_u16::<BigEndian>(self.transforms.len() as u16)?;
        writer.write_u16::<BigEndian>(0)?;

        for transform in &self.transforms {
            let header = transform.bone << 4;
            let tr: u16 = transform.kind.into();
            let header = header + tr;
            writer.write_u16::<BigEndian>(header)?;
            writer.write_u16::<BigEndian>(transform.frames.len() as u16)?;

            for frame in &transform.frames {
                let mut header = frame.frame;

                if frame.unk1 {
                    header += 0x8000;
                }

                if frame.unk2 {
                    header += 0x4000;
                }

                let factor = float_to_fixed(frame.factor);

                writer.write_u16::<BigEndian>(header)?;
                writer.write_u16::<BigEndian>(factor)?;
            }
        }

        Ok(())
    }
}
