use super::Prop1;
use super::Prop2;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub props_1: Vec<Prop1>,
    pub props_2: Vec<Prop2>,
    missing: bool,
}

impl Cube {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let id = reader.read_u8()?;
        Ok(match id {
            1 => Self {
                x: 0,
                y: 0,
                z: 0,
                props_1: vec![],
                props_2: vec![],
                missing: true,
            },
            3 => {
                let subheader = reader.read_u8()?;
                assert_eq!(subheader, 10);

                let mut props_1 = vec![];
                let mut props_2 = vec![];

                let prop_1_count = reader.read_u8()?;
                let mut list_type = reader.read_u8()?;
                if list_type == 11 {
                    for _ in 0..prop_1_count {
                        props_1.push(Prop1::new(reader)?);
                    }
                    list_type = reader.read_u8()?;
                }
                assert_eq!(list_type, 8);

                let prop_2_count = reader.read_u8()?;

                if prop_2_count > 0 {
                    let list_type = reader.read_u8()?;
                    assert_eq!(list_type, 9);

                    for _ in 0..prop_2_count {
                        props_2.push(Prop2::new(reader)?);
                    }
                }

                let end = reader.read_u8()?;
                assert_eq!(end, 1);

                Cube {
                    x: 0,
                    y: 0,
                    z: 0,
                    props_1,
                    props_2,
                    missing: false,
                }
            }
            _ => unimplemented!(),
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        if self.missing {
            writer.write_u8(1)?;
            return Ok(());
        }

        writer.write_u8(3)?;
        writer.write_u8(10)?;

        writer.write_u8(self.props_1.len() as u8)?;
        if self.props_1.len() > 0 {
            writer.write_u8(11)?;
            for p1 in &self.props_1 {
                p1.write(writer)?;
            }
        }
        writer.write_u8(8)?;

        writer.write_u8(self.props_2.len() as u8)?;
        if self.props_2.len() > 0 {
            writer.write_u8(9)?;
            for p2 in &self.props_2 {
                p2.write(writer)?;
            }
        }

        writer.write_u8(1)?;

        Ok(())
    }
}
