use crate::types::*;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub enum Camera {
    Empty {
        id: u16,
    },
    Pivot {
        id: u16,
        position: Vector3<f32>,
        speed: Vector2<f32>,
        rotation: f32,
        acceleration: f32,
        angles: Vector3<f32>,
        unk: u32,
    },
    Static {
        id: u16,
        position: Vector3<f32>,
        angles: Vector3<f32>,
    },
    Zoom {
        id: u16,
        position: Vector3<f32>,
        speed: Vector2<f32>,
        rotation: f32,
        acceleration: f32,
        angles: Vector3<f32>,
        unk: u32,
        distances: Vector2<f32>,
    },
    Random {
        id: u16,
        unk: u32,
    },
}

impl Camera {
    pub fn new<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let id = reader.read_u16::<BigEndian>()?;
        let camera_two = reader.read_u8()?;
        assert_eq!(camera_two, 2);
        let camera_type = reader.read_u8()?;

        Ok(match camera_type {
            0 => Camera::Empty { id },
            1 => Self::read_pivot_or_zoom_camera(reader, 1, id)?,
            2 => Self::read_static_camera(reader, id)?,
            3 => Self::read_pivot_or_zoom_camera(reader, 3, id)?,
            4 => Self::read_random_camera(reader, id)?,
            _ => todo!(),
        })
    }

    fn read_pivot_or_zoom_camera<R: Read + Seek>(
        reader: &mut R,
        camera_type: u8,
        id: u16,
    ) -> Result<Self, Box<dyn Error>> {
        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 1);
        let position = read_3_floats(reader)?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 2);
        let speed = read_2_floats(reader)?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 3);
        let rotation = reader.read_f32::<BigEndian>()?;
        let acceleration = reader.read_f32::<BigEndian>()?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 4);
        let angles = read_3_floats(reader)?;

        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 5);
        let unk = reader.read_u32::<BigEndian>()?;

        if camera_type == 3 {
            let section_id = reader.read_u8()?;
            assert_eq!(section_id, 6);
            let distances = read_2_floats(reader)?;

            let end_of_camera = reader.read_u8()?;
            assert_eq!(end_of_camera, 0);

            Ok(Camera::Zoom {
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
                distances,
            })
        } else {
            let end_of_camera = reader.read_u8()?;
            assert_eq!(end_of_camera, 0);

            Ok(Camera::Pivot {
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
            })
        }
    }

    fn read_static_camera<R: Read + Seek>(reader: &mut R, id: u16) -> Result<Self, Box<dyn Error>> {
        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 1);
        let position = read_3_floats(reader)?;
        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 2);
        let angles = read_3_floats(reader)?;
        let end_of_camera = reader.read_u8()?;
        assert_eq!(end_of_camera, 0);

        Ok(Camera::Static {
            id,
            position,
            angles,
        })
    }

    fn read_random_camera<R: Read + Seek>(reader: &mut R, id: u16) -> Result<Self, Box<dyn Error>> {
        let section_id = reader.read_u8()?;
        assert_eq!(section_id, 1);
        let unk = reader.read_u32::<BigEndian>()?;
        let end_of_camera = reader.read_u8()?;
        assert_eq!(end_of_camera, 0);

        Ok(Camera::Random { id, unk })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        match self {
            Camera::Empty { id } => Self::write_header(writer, *id, 0),
            Camera::Pivot { .. } | Camera::Zoom { .. } => self.write_pivot_or_zoom_camera(writer),
            Camera::Static {
                id,
                position,
                angles,
            } => {
                Self::write_header(writer, *id, 2)?;

                writer.write_u8(1)?;
                write_3_floats(writer, position)?;

                writer.write_u8(2)?;
                write_3_floats(writer, angles)?;

                writer.write_u8(0)?;

                Ok(())
            }
            Camera::Random { id, unk } => {
                Self::write_header(writer, *id, 4)?;

                writer.write_u8(1)?;
                writer.write_u32::<BigEndian>(*unk)?;

                writer.write_u8(0)?;

                Ok(())
            }
        }
    }

    pub fn write_header<W: Write>(
        writer: &mut W,
        id: u16,
        camera_type: u8,
    ) -> Result<(), Box<dyn Error>> {
        writer.write_u16::<BigEndian>(id)?;
        writer.write_u8(2)?;
        writer.write_u8(camera_type)?;

        Ok(())
    }

    pub fn write_pivot_or_zoom_camera<W: Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), Box<dyn Error>> {
        let (id, position, speed, rotation, acceleration, angles, unk, distances) = match self {
            Camera::Pivot {
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
            } => (
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
                None,
            ),
            Camera::Zoom {
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
                distances,
            } => (
                id,
                position,
                speed,
                rotation,
                acceleration,
                angles,
                unk,
                Some(distances),
            ),
            _ => unreachable!(),
        };

        Self::write_header(writer, *id, if distances.is_none() { 1 } else { 3 })?;

        writer.write_u8(1)?;
        write_3_floats(writer, position)?;

        writer.write_u8(2)?;
        write_2_floats(writer, speed)?;

        writer.write_u8(3)?;
        writer.write_f32::<BigEndian>(*rotation)?;
        writer.write_f32::<BigEndian>(*acceleration)?;

        writer.write_u8(4)?;
        write_3_floats(writer, angles)?;

        writer.write_u8(5)?;
        writer.write_u32::<BigEndian>(*unk)?;

        if let Some(d) = distances {
            writer.write_u8(6)?;
            write_2_floats(writer, d)?;
        }
        writer.write_u8(0)?;

        Ok(())
    }
}
