#![allow(unused)]

use crate::utils::read_2_floats;
use crate::utils::read_3_floats;
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::error::Error;
use std::io::Read;
use std::io::Write;

pub enum GeometryLayout {
    Unknown0,
    Sort,
    LoadDl,
    Lod,
    ReferencePoint,
    Selector,
}

impl GeometryLayout {
    pub fn new<R: Read>(reader: &mut R) -> Result<(Self, bool), Box<dyn Error>> {
        let id = reader.read_u32::<BigEndian>()?;
        println!("GEO {id}");
        let length = reader.read_u32::<BigEndian>()?;

        Ok(match id {
            0 => {
                assert_eq!(length, 0x28);
                let len = reader.read_u16::<BigEndian>()?;
                assert_eq!(len, 0x18);
                let child_count = reader.read_u16::<BigEndian>()?;
                assert_eq!(child_count, 1);
                let _unk = read_3_floats(reader)?;

                loop {
                    let (_geo_layout, last) = GeometryLayout::new(reader)?;
                    if last {
                        break;
                    }
                }

                if length != 0 {
                    let padding = reader.read_u32::<BigEndian>()?;
                    assert_eq!(padding, 0);
                }

                (Self::Unknown0, length == 0)
            }
            1 => {
                let _first_child = read_3_floats(reader)?;
                let _second_child = read_3_floats(reader)?;
                let sort = reader.read_u16::<BigEndian>()?;
                assert!(sort == 1 || sort == 0);
                let _child1 = reader.read_u16::<BigEndian>()?;
                let child2 = reader.read_u32::<BigEndian>()?;
                assert!(child2 < 0x10000);

                let (_child_1, _last) = GeometryLayout::new(reader)?;
                let (_child_2, _last) = GeometryLayout::new(reader)?;

                (Self::Sort, length == 0)
            }
            3 => {
                assert!(length == 0 || length == 0x10);
                let _offset_in_dl_segment = reader.read_u16::<BigEndian>()?;
                let _tri_count = reader.read_u16::<BigEndian>()?;

                if length == 0x10 {
                    let padding = reader.read_u32::<BigEndian>()?;
                    assert_eq!(padding, 0);
                }

                (Self::LoadDl, length == 0)
            }
            8 => {
                let _distance = read_2_floats(reader)?;
                let _test = read_3_floats(reader)?;
                let offset = reader.read_u32::<BigEndian>()?;
                assert_eq!(offset, 0x20);

                loop {
                    let (_geo_layout, last) = GeometryLayout::new(reader)?;
                    if last {
                        break;
                    }
                }

                if length != 0 {
                    let padding = reader.read_u32::<BigEndian>()?;
                    assert_eq!(padding, 0);
                }

                (Self::Lod, length == 0)
            }
            10 => {
                let _ref_point = reader.read_u16::<BigEndian>()?;
                let _bone = reader.read_u16::<BigEndian>()?;
                let _offset = read_3_floats(reader)?;

                (Self::ReferencePoint, length == 0)
            }
            12 => {
                let child_count = reader.read_u16::<BigEndian>()?;
                let _selector_index = reader.read_u16::<BigEndian>()?;

                for _ in 0..child_count {
                    let _ = reader.read_u32::<BigEndian>()?;
                }

                if (child_count % 2) == 0 {
                    let padding = reader.read_u32::<BigEndian>()?;
                    assert_eq!(padding, 0);
                }

                for i in 0..child_count {
                    let _ = GeometryLayout::new(reader)?;
                    if i + 1 < child_count {
                        let padding = reader.read_u32::<BigEndian>()?;
                        assert_eq!(padding, 0);
                    }
                }

                if length != 0 {
                    let padding = reader.read_u32::<BigEndian>()?;
                    assert_eq!(padding, 0);
                }

                (Self::Selector, length == 0)
            }
            _ => panic!("TODO GeometryLayout {id}"),
        })
    }

    #[allow(unused)]
    pub fn write<W: Write>(&self, _writer: &mut W) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
